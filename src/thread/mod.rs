use crate::{util, Error, ErrorKind, LuaResult};

use std::{
    alloc::{self, Layout},
    any::Any,
    error, fmt,
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};

mod call;

pub use call::*;

#[derive(Debug)]
pub enum ThreadError {
    Panic(Box<dyn Any + Send + 'static>),
    Lua(Error),
}

impl error::Error for ThreadError {
    fn description(&self) -> &str {
        match self {
            ThreadError::Panic(_) => "panicked while running thread",
            ThreadError::Lua(_) => "lua error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", error::Error::description(self))?;
        match self {
            ThreadError::Panic(panic) => fmt::Debug::fmt(panic, f),
            ThreadError::Lua(error) => fmt::Display::fmt(error, f),
        }
    }
}

impl From<Error> for ThreadError {
    fn from(error: Error) -> ThreadError {
        ThreadError::Lua(error)
    }
}

/// Lua thread (state) wrapper.
#[derive(Debug)]
pub struct Thread {
    raw: NonNull<sys::lua_State>,
}

impl Thread {
    /// Spawns a new Lua thread and runs `f` with the new thread as a parameter.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// Thread::spawn(move |thread| {
    ///     println!("Lua version: {}", thread.version());
    /// }).unwrap()
    /// ```
    pub fn spawn<F, T>(f: F) -> Result<T, ThreadError>
    where
        F: FnOnce(&mut Thread) -> T,
    {
        // Safe because allocator is set to `None`.
        unsafe { Thread::spawn_with_allocator(f, None, ptr::null_mut::<()>()) }
    }

    /// A variant of [`Thread::spawn`] that takes an optional allocator function.alloc
    ///
    /// # Safety
    /// If present, the allocator function must behave exactly as defined in [`the Lua manual`],
    /// behavior is undefined if the function pointer is invalid, returns invalid allocations,
    /// or frees memory incorrectly.
    ///
    /// [`Thread::spawn`]: struct.Thread.html#method.spawn
    /// [`the Lua manual`]: https://www.lua.org/manual/5.3/manual.html#lua_Alloc
    pub unsafe fn spawn_with_allocator<F, T, U>(
        f: F,
        allocator: sys::lua_Alloc,
        userdata: *mut U,
    ) -> Result<T, ThreadError>
    where
        F: FnOnce(&mut Thread) -> T,
    {
        Thread::new(allocator, userdata as *mut libc::c_void)
            .map(|mut t| f(&mut t))
            .map_err(ThreadError::from)
    }

    /// Creates a `Thread` from an allocator function.
    /// If `allocator` is set to `None`, the  default allocator will be used.
    /// `userdata` is a (nullable) raw pointer passed to the allocator function,
    /// if `allocator` is `None` then this parameter is ignored.
    ///
    /// # Safety
    /// If present, the allocator function must behave exactly as defined in [`the Lua manual`],
    /// behavior is undefined if the function pointer is invalid, returns invalid allocations,
    /// or frees memory incorrectly.
    ///
    /// [`the Lua manual`]: https://www.lua.org/manual/5.3/manual.html#lua_Alloc
    unsafe fn new(allocator: sys::lua_Alloc, userdata: *mut libc::c_void) -> LuaResult<Thread> {
        let mut thread = Thread {
            raw: NonNull::new(match allocator {
                Some(_) => sys::lua_newstate(allocator, userdata),
                None => sys::lua_newstate(Some(alloc_default), ptr::null_mut()),
            })
            .ok_or_else(|| Error::new(ErrorKind::OutOfMemory, None))?,
        };
        sys::lua_atpanic(thread.raw.as_ptr(), Some(at_panic));
        thread.check_version()?;
        Ok(thread)
    }

    /// Checks whether the core running the call, the core that created the Lua state,
    /// and the code making the call are all using the same version of Lua.
    /// Returns [`Error::Runtime`] if there is a version mismatch.
    ///
    /// [`Error::Runtime`]: struct.Thread.html#method.from_raw
    unsafe fn check_version(&mut self) -> LuaResult<()> {
        unsafe extern "C" fn check(l: *mut sys::lua_State) -> libc::c_int {
            sys::luaL_checkversion(l);
            0
        }

        // If luaL_checkversion failed, pcall will return an error
        sys::lua_pushcfunction(self.raw.as_ptr(), Some(check));
        self.get_error(sys::lua_pcall(self.raw.as_ptr(), 0, 0, 0))
    }

    /// Returns the error for the given `code`.
    /// If `code` is not `LUA_OK` then the object at stack index -1 is used as the error message.
    pub fn get_error(&mut self, code: libc::c_int) -> LuaResult<()> {
        if code == sys::LUA_OK {
            Ok(())
        } else {
            Err(Error {
                kind: match code {
                    sys::LUA_ERRRUN => ErrorKind::Runtime,
                    sys::LUA_ERRSYNTAX => ErrorKind::Syntax,
                    sys::LUA_ERRMEM => ErrorKind::OutOfMemory,
                    sys::LUA_ERRERR => ErrorKind::MessageHandler,
                    sys::LUA_ERRGCMM => ErrorKind::GarbageCollection,
                    sys::LUA_ERRFILE | _ => ErrorKind::Io,
                },
                msg: unsafe {
                    // check if there is a value at stack index -1
                    if sys::lua_isnone(self.as_raw().as_ptr(), -1) == 0 {
                        let mut len = 0usize;
                        // get the error object as a c string
                        let s = sys::luaL_tolstring(self.as_raw().as_ptr(), -1, &mut len as *mut _);
                        // luaL_tolstring also pushes its result to the stack, so we have to pop it.
                        sys::lua_pop(self.as_raw().as_ptr(), -1);
                        if s.is_null() {
                            None
                        } else {
                            // s is garanteed to be a valid c string at this point.
                            let buf = slice::from_raw_parts(s as *const u8, len);
                            Some(String::from_utf8_lossy(buf).into_owned())
                        }
                    } else {
                        None
                    }
                },
            })
        }
    }

    /// Returns the Lua version number.
    ///
    /// # Examples
    /// ```
    /// use pollua::thread::Thread;
    ///
    /// let thread_version = Thread::spawn(move |thread| {
    ///     thread.version()
    /// }).expect("Failed to create Lua thread");
    ///
    /// assert_eq!(thread_version, pollua::lua_version());
    /// ```
    #[inline]
    pub fn version(&self) -> sys::lua_Number {
        unsafe { *sys::lua_version(self.raw.as_ptr()) }
    }

    /// Returns a raw pointer the wrapped `lua_State`.
    ///
    /// It is up to the caller to ensure that the object is still alive when accessing it through
    /// the pointer.
    #[inline]
    pub fn as_raw(&mut self) -> NonNull<sys::lua_State> {
        self.raw
    }

    /// Constructs a `Thread` from a raw pointer.
    ///
    /// After calling this function, the raw pointer is owned by the resulting `Thread`.
    /// Specifically, the `Thread` destructor will call [`lua_close`] and free the lua state.
    ///
    /// # Safety
    /// Behavior is undefined if the pointer is invalid or already in use.
    ///
    /// [`lua_close`]: sys::lua_close
    #[inline]
    pub unsafe fn from_raw(raw: NonNull<sys::lua_State>) -> Thread {
        Thread { raw }
    }

    /// Creates a `Thread` reference (of type [`ThreadRef`]) from a `lua_State` pointer.
    ///
    /// # Safety
    /// Behavior is undefined if `raw` is invalid or muliple reference to this state co-exist.
    ///
    /// [`ThreadRef`]: struct.ThreadRef.html
    #[inline]
    pub unsafe fn ref_from_raw<'a>(raw: NonNull<sys::lua_State>) -> &'a mut Thread {
        &mut *raw.cast::<Thread>().as_ptr()
    }

    /// Loads a Lua chunk and creates a [`Caller`] for it if there were no errors.
    /// The resulting [`Caller`] takes no argmuents and returns nothing.
    ///
    /// [`Caller`]: struct.Caller.html
    #[inline(always)]
    pub fn caller_load<'a, B: AsRef<[u8]> + ?Sized>(
        &'a mut self,
        to_load: &B,
        chunk_name: Option<&str>,
        mode: LoadingMode,
    ) -> LuaResult<Caller<'a>> {
        self.caller_load_impl(to_load.as_ref(), chunk_name, mode)
    }

    /// Creates a [`Caller`] for the given global function name.
    /// Returns `None` if `_G.[name]` is not defined or is not a function.alloc
    ///
    /// [`Caller`]: struct.Caller.html
    #[inline(always)]
    pub fn caller_global<S: AsRef<[u8]> + ?Sized>(&mut self, name: &S) -> Option<Caller> {
        Caller::from_global(ThreadRef::from_ref(self), name.as_ref())
    }

    /// Creates a [`Caller`] for the function located at the top of the stack.
    ///
    /// # Safety
    /// Behavior is undefined if the value at the top of the stack is not a function.
    ///
    /// [`Caller`]: struct.Caller.html
    #[inline(always)]
    pub(crate) unsafe fn caller_stack_unchecked(&mut self) -> Caller {
        Caller::from_stack_unchecked(ThreadRef::from_ref(self))
    }

    /// Similar to `lua_getglobal`, but accepts any string.
    #[inline(always)]
    fn push_global<S: AsRef<[u8]> + ?Sized>(&mut self, name: &S) -> libc::c_int {
        self.push_global_impl(name.as_ref())
    }
}

// Method impls
impl Thread {
    fn caller_load_impl<'a>(
        &'a mut self,
        buffer: &[u8],
        chunk_name: Option<&str>,
        mode: LoadingMode,
    ) -> LuaResult<Caller<'a>> {
        let mut name_buf = Vec::new();
        unsafe {
            let code = sys::luaL_loadbufferx(
                self.as_raw().as_ptr(),
                util::cstr_unchecked(Some(buffer)),
                buffer.len(),
                util::cstr_buf(chunk_name, &mut name_buf),
                util::cstr_unchecked(Some(match mode {
                    LoadingMode::Binary => "b\0",
                    LoadingMode::Text => "t\0",
                    LoadingMode::Auto => "bt\0",
                })),
            );
            match self.get_error(code) {
                Ok(()) => Ok(self.caller_stack_unchecked()),
                Err(e) => Err(e),
            }
        }
    }

    fn push_global_impl(&mut self, name: &[u8]) -> libc::c_int {
        unsafe {
            let ptr = self.raw.as_ptr();
            // push the global env onto the stack
            sys::lua_rawgeti(ptr, sys::LUA_REGISTRYINDEX, sys::LUA_RIDX_GLOBALS);
            // push the global variable name onto the stack
            sys::lua_pushlstring(ptr, name.as_ptr() as *const libc::c_char, name.len());
            // fetch _G[name]
            let value_type = sys::lua_rawget(ptr, -2);
            // remove the global env from the stack
            sys::lua_replace(ptr, -2);
            value_type
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            sys::lua_close(self.raw.as_ptr());
        }
    }
}

/// Used by the [`Thread::load_bytes`] method, describes how the bytes should be interpreted.
///
/// [`Thread::load_bytes`]: struct.Thread.html#method.load_bytes
#[derive(Debug, Copy, Clone)]
pub enum LoadingMode {
    Binary,
    Text,
    Auto,
}

/// A mutable reference to a [`Thread`].
///
/// [`Thread`]: struct.Thread.html
pub struct ThreadRef<'a> {
    // thread's destructor must never be called, as it would trigger a double-free.
    thread: ManuallyDrop<Thread>,
    _marker: PhantomData<&'a mut Thread>,
}

impl<'a> ThreadRef<'a> {
    /// Creates a `ThreadRef` from a raw `lua_State` pointer.
    ///
    /// # Safety
    /// Behavior is undefined if `raw` is invalid or multiple reference to this `lua_State` exists.
    #[inline]
    pub unsafe fn from_raw(raw: NonNull<sys::lua_State>) -> ThreadRef<'a> {
        ThreadRef {
            thread: ManuallyDrop::new(Thread::from_raw(raw)),
            _marker: PhantomData,
        }
    }

    /// Creates a `ThreadRef` from a [`Thread`] mutable reference.
    ///
    /// [`Thread`]: struct.Thread.html
    #[inline]
    pub fn from_ref(thread: &'a mut Thread) -> ThreadRef<'a> {
        unsafe { ThreadRef::from_raw(thread.as_raw()) }
    }
}

impl fmt::Debug for ThreadRef<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.thread.fmt(f)
    }
}

impl<'a> Deref for ThreadRef<'a> {
    type Target = Thread;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.thread
    }
}

impl<'a> DerefMut for ThreadRef<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.thread
    }
}

/// Default panic handler function.
unsafe extern "C" fn at_panic(thread: *mut sys::lua_State) -> libc::c_int {
    match Thread::ref_from_raw(NonNull::new_unchecked(thread)).get_error(sys::LUA_ERRRUN) {
        Ok(()) => 0,
        Err(Error { msg: None, .. }) => panic!("Lua panic: <no error message>"),
        Err(Error { msg: Some(m), .. }) => panic!("Lua panic: {}", m),
    }
}

/// Default allocation function.
/// Uses the liballoc functions instead of the one from libc.
unsafe extern "C" fn alloc_default(
    _ud: *mut libc::c_void,
    ptr: *mut libc::c_void,
    osize: usize,
    nsize: usize,
) -> *mut libc::c_void {
    if nsize == 0 {
        alloc::dealloc(ptr as *mut u8, Layout::from_size_align_unchecked(osize, 1));
        ptr::null_mut()
    } else if ptr.is_null() {
        alloc::alloc(Layout::from_size_align_unchecked(nsize, 1)) as *mut _
    } else {
        alloc::realloc(
            ptr as *mut u8,
            Layout::from_size_align_unchecked(osize, 1),
            nsize,
        ) as *mut _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn stack_top(thread: &mut Thread) -> libc::c_int {
        unsafe { sys::lua_gettop(thread.as_raw().as_ptr()) }
    }

    fn type_at(thread: &mut Thread, index: libc::c_int) -> libc::c_int {
        unsafe { sys::lua_type(thread.as_raw().as_ptr(), index) }
    }

    #[test]
    fn test_thread_push_global() {
        Thread::spawn(move |thread| {
            let mut top;
            top = stack_top(thread);
            thread.push_global("undef_var");
            assert_eq!(type_at(thread, -1), sys::LUA_TNIL);
            assert_eq!(stack_top(thread), top + 1);

            unsafe {
                sys::lua_pushinteger(thread.as_raw().as_ptr(), 42);
                sys::lua_setglobal(thread.as_raw().as_ptr(), b"num_var\0".as_ptr() as *const _);
            }
            top = stack_top(thread);
            thread.push_global("num_var");
            assert_eq!(type_at(thread, -1), sys::LUA_TNUMBER);
            assert!(unsafe { sys::lua_isinteger(thread.as_raw().as_ptr(), -1) } != 0);
            assert_eq!(
                unsafe { sys::lua_tointeger(thread.as_raw().as_ptr(), -1) },
                42
            );
            assert_eq!(stack_top(thread), top + 1);
        })
        .unwrap()
    }
}
