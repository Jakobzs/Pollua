use crate::*;
use core::ptr;

// //////////////////////////////////////////// //
// Lua Constants                                //
// //////////////////////////////////////////// //

pub const LUA_MULTRET: libc::c_int = -1;

#[cfg(LUA_VERSION = "5.2")]
pub const LUA_REGISTRYINDEX: libc::c_int = -LUAI_MAXSTACK - 1000;
#[cfg(not(LUA_VERSION = "5.2"))]
pub const LUA_REGISTRYINDEX: libc::c_int = -10000;

#[cfg(all(LUA_VERSION = "5.1", not(LUA_VERSION = "5.2")))]
pub const LUA_ENVIRONINDEX: libc::c_int = -10001;

// Added in Lua 5.1 Removed in Lua 5.2
#[cfg(any(not(LUA_VERSION = "5.2")))]
pub const LUA_GLOBALSINDEX: libc::c_int = -10002;

pub const LUA_YIELD: libc::c_int = 1;
pub const LUA_ERRRUN: libc::c_int = 2;
pub const LUA_ERRSYNTAX: libc::c_int = 3;
pub const LUA_ERRMEM: libc::c_int = 4;

cfg_if::cfg_if! {
    if #[cfg(LUA_VERSION = "5.2")] {
        pub const LUA_OK: libc::c_int = 0;
        pub const LUA_ERRGCMM: libc::c_int = 5;
        pub const LUA_ERRERR: libc::c_int = 6;
    } else {
        pub const LUA_ERRERR: libc::c_int = 5;
    }
}

pub const LUA_TNONE: libc::c_int = -1;
pub const LUA_TNIL: libc::c_int = 0;
pub const LUA_TBOOLEAN: libc::c_int = 1;
pub const LUA_TLIGHTUSERDATA: libc::c_int = 2;
pub const LUA_TNUMBER: libc::c_int = 3;
pub const LUA_TSTRING: libc::c_int = 4;
pub const LUA_TTABLE: libc::c_int = 5;
pub const LUA_TFUNCTION: libc::c_int = 6;
pub const LUA_TUSERDATA: libc::c_int = 7;
pub const LUA_TTHREAD: libc::c_int = 8;

cfg_if::cfg_if! {
    if #[cfg(LUA_VERSION = "5.2")] {
        pub const LUA_NUMTAGS: libc::c_int = 9;
    }
}

pub const LUA_MINSTACK: libc::c_int = 20;

// Introduced in Lua 5.2
cfg_if::cfg_if! {
    if #[cfg(LUA_VERSION = "5.2")] {
        pub const LUA_RIDX_MAINTHREAD: lua_Integer = 1;
        pub const LUA_RIDX_GLOBALS: lua_Integer = 2;
        pub const LUA_RIDX_LAST: lua_Integer = LUA_RIDX_GLOBALS;
    }
}

pub const LUA_OPADD: libc::c_int = 0;
pub const LUA_OPSUB: libc::c_int = 1;
pub const LUA_OPMUL: libc::c_int = 2;

cfg_if::cfg_if! {
    if #[cfg(LUA_VERSION = "5.3")] {
        pub const LUA_OPMOD: libc::c_int = 3;
        pub const LUA_OPPOW: libc::c_int = 4;
        pub const LUA_OPDIV: libc::c_int = 5;
        pub const LUA_OPIDIV: libc::c_int = 6;
        pub const LUA_OPBAND: libc::c_int = 7;
        pub const LUA_OPBOR: libc::c_int = 8;
        pub const LUA_OPBXOR: libc::c_int = 9;
        pub const LUA_OPSHL: libc::c_int = 10;
        pub const LUA_OPSHR: libc::c_int = 11;
        pub const LUA_OPUNM: libc::c_int = 12;
        pub const LUA_OPBNOT: libc::c_int = 13;
    } else {
        pub const LUA_OPDIV: libc::c_int = 3;
        pub const LUA_OPMOD: libc::c_int = 4;
        pub const LUA_OPPOW: libc::c_int = 5;
        pub const LUA_OPUNM: libc::c_int = 6;
    }
}

pub const LUA_OPEQ: libc::c_int = 0;
pub const LUA_OPLT: libc::c_int = 1;
pub const LUA_OPLE: libc::c_int = 2;

pub const LUA_GCSTOP: libc::c_int = 0;
pub const LUA_GCRESTART: libc::c_int = 1;
pub const LUA_GCCOLLECT: libc::c_int = 2;
pub const LUA_GCCOUNT: libc::c_int = 3;
pub const LUA_GCCOUNTB: libc::c_int = 4;
pub const LUA_GCSTEP: libc::c_int = 5;
pub const LUA_GCSETPAUSE: libc::c_int = 6;
pub const LUA_GCSETSTEPMUL: libc::c_int = 7;
pub const LUA_GCISRUNNING: libc::c_int = 9;

// Only defined in Lua 5.2
cfg_if::cfg_if! {
    if #[cfg(all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")))] {
        pub const LUA_GCSETMAJORINC: libc::c_int = 8;
        pub const LUA_GCGEN: libc::c_int = 10;
        pub const LUA_GCINC: libc::c_int = 11;
    }
}

pub const LUA_HOOKCALL: libc::c_int = 0;
pub const LUA_HOOKRET: libc::c_int = 1;
pub const LUA_HOOKLINE: libc::c_int = 2;
pub const LUA_HOOKCOUNT: libc::c_int = 3;
pub const LUA_HOOKTAILCALL: libc::c_int = 4;

pub const LUA_MASKCALL: libc::c_int = (1 as libc::c_int).wrapping_shl(LUA_HOOKCALL as libc::c_uint);
pub const LUA_MASKRET: libc::c_int = (1 as libc::c_int).wrapping_shl(LUA_HOOKRET as libc::c_uint);
pub const LUA_MASKLINE: libc::c_int = (1 as libc::c_int).wrapping_shl(LUA_HOOKLINE as libc::c_uint);
pub const LUA_MASKCOUNT: libc::c_int =
    (1 as libc::c_int).wrapping_shl(LUA_HOOKCOUNT as libc::c_uint);

pub const LUAI_MAXSTACK: libc::c_int = 1_000_000;

pub const LUA_EXTRASPACE: usize = ::core::mem::size_of::<*const libc::c_void>();

pub const LUA_IDSIZE: usize = 60;

pub const LUA_MAXINTEGER: lua_Integer = lua_Integer::max_value();
pub const LUA_MININTEGER: lua_Integer = lua_Integer::min_value();

// //////////////////////////////////////////// //
// Lua types                                    //
// //////////////////////////////////////////// //

pub type lua_Alloc = Option<
    unsafe extern "C" fn(
        ud: *mut libc::c_void,
        ptr: *mut libc::c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut libc::c_void,
>;

pub type lua_CFunction = Option<unsafe extern "C" fn(L: *mut lua_State) -> libc::c_int>;

pub type lua_KFunction = Option<
    unsafe extern "C" fn(L: *mut lua_State, status: libc::c_int, ctx: lua_KContext) -> libc::c_int,
>;

pub type lua_Reader = Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        ud: *mut libc::c_void,
        sz: *mut usize,
    ) -> *const libc::c_char,
>;

#[repr(C)]
#[derive(Debug)]
pub struct lua_State {
    _private: [u8; 0],
}

pub type lua_Writer = Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        p: *const libc::c_void,
        sz: usize,
        ud: *mut libc::c_void,
    ) -> libc::c_int,
>;

// //////////////////////////////////////////// //
// Lua Functions                                //
// //////////////////////////////////////////// //

extern "C" {
    pub fn lua_absindex(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_arith(L: *mut lua_State, op: libc::c_int);
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

    // Defined as a macro as of Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int);

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_callk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );

    pub fn lua_checkstack(L: *mut lua_State, n: libc::c_int) -> libc::c_int;
    pub fn lua_close(L: *mut lua_State);
    pub fn lua_compare(
        L: *mut lua_State,
        idx1: libc::c_int,
        idx2: libc::c_int,
        op: libc::c_int,
    ) -> libc::c_int;
    pub fn lua_concat(L: *mut lua_State, n: libc::c_int);
    pub fn lua_copy(L: *mut lua_State, fromidx: libc::c_int, toidx: libc::c_int);

    // Deprecated in Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_cpcall(L: *mut lua_State, func: lua_CFunction, ud: *mut libc::c_void)
        -> libc::c_int;

    pub fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);

    // lua_dump takes an extra argument in version 5.3+
    #[cfg(LUA_VERSION = "5.3")]
    pub fn lua_dump(
        L: *mut lua_State,
        writer: lua_Writer,
        data: *mut libc::c_void,
        strip: libc::c_int,
    ) -> libc::c_int;
    #[cfg(not(LUA_VERSION = "5.3"))]
    pub fn lua_dump(L: *mut lua_State, writer: lua_Writer, data: *mut libc::c_void) -> libc::c_int;

    // Deprecated in Lua 5.2
    #[cfg(all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.2")))]
    pub fn lua_equal(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int;

    pub fn lua_error(L: *mut lua_State) -> !;
    pub fn lua_gc(L: *mut lua_State, what: libc::c_int, data: libc::c_int) -> libc::c_int;
    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut libc::c_void) -> lua_Alloc;

    // Only present in Lua 5.2
    #[cfg(all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")))]
    pub fn lua_getctx(L: *mut lua_State, ctx: *mut libc::c_int) -> libc::c_int;

    // Removed in Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_getfenv(L: *mut lua_State, index: libc::c_int);

    pub fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char)
        -> libc::c_int;
    pub fn lua_getglobal(L: *mut lua_State, name: *const libc::c_char) -> libc::c_int;
    pub fn lua_geti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    pub fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    pub fn lua_gettable(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    pub fn lua_getuservalue(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_iscfunction(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_isinteger(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_isnumber(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_isuserdata(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_isyieldable(L: *mut lua_State) -> libc::c_int;
    pub fn lua_len(L: *mut lua_State, idx: libc::c_int);

    // Deprecated in Lua 5.2
    #[cfg(all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.2")))]
    pub fn lua_lessthan(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int;

    // Takes an extra parameter 'mode' in Lua 5.2+
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        data: *mut libc::c_void,
        chunkname: *const libc::c_char,
    ) -> libc::c_int;

    // Introduced in Lua 5.1
    #[cfg(LUA_VERSION = "5.1")]
    pub fn lua_newstate(f: lua_Alloc, ud: *mut libc::c_void) -> *mut lua_State;

    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
    pub fn lua_newuserdata(L: *mut lua_State, sz: usize) -> *mut libc::c_void;
    pub fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;

    // Introduced in Lua 5.1, renamed to lua_rawlen in 5.2
    #[cfg(all(LUA_VERSION = "5.1", not(LUA_VERSION = "5.2")))]
    pub fn lua_objlen(L: *mut lua_State, index: libc::c_int) -> usize;

    // Replaced to lua_newstate in Lua 5.1
    #[cfg(all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.1")))]
    pub fn lua_open() -> *mut lua_State;

    // Defined as a macro as of Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        msgh: libc::c_int,
    ) -> libc::c_int;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_pcallk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;

    pub fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    pub fn lua_pushcclosure(L: *mut lua_State, fn_: lua_CFunction, n: libc::c_int);
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...)
        -> *const libc::c_char;
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
    pub fn lua_pushlstring(
        L: *mut lua_State,
        s: *const libc::c_char,
        len: usize,
    ) -> *const libc::c_char;
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    pub fn lua_pushthread(L: *mut lua_State) -> libc::c_int;

    // Deprecated in Lua 5.3
    #[cfg(any(all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3"))))]
    pub fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);

    pub fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);

    #[cfg(feature = "va-list")]
    pub fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: va_list::VaList,
    ) -> *const libc::c_char;

    pub fn lua_rawequal(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int;
    pub fn lua_rawget(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> libc::c_int;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_rawgetp(L: *mut lua_State, idx: libc::c_int, p: *const libc::c_void) -> libc::c_int;

    // Renamed from lua_objlen in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_rawlen(L: *mut lua_State, idx: libc::c_int) -> usize;

    pub fn lua_rawset(L: *mut lua_State, idx: libc::c_int);
    pub fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer);

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_rawsetp(L: *mut lua_State, idx: libc::c_int, p: *const libc::c_void);

    // Takes an extra parameter 'from' in Lua 5.2+
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_resume(L: *mut lua_State, from: *mut lua_State, narg: libc::c_int) -> libc::c_int;
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_resume(L: *mut lua_State, narg: libc::c_int) -> libc::c_int;

    pub fn lua_rotate(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut libc::c_void);

    // Removed in Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_setfenv(L: *mut lua_State, index: libc::c_int) -> libc::c_int;

    pub fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    pub fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    pub fn lua_seti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer);
    pub fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    pub fn lua_settable(L: *mut lua_State, idx: libc::c_int);
    pub fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    pub fn lua_setuservalue(L: *mut lua_State, idx: libc::c_int);
    pub fn lua_status(L: *mut lua_State) -> libc::c_int;

    // Only defined as a function in Lua 5.0
    #[cfg(all(LUA_VERSION = "5.0", not(LUA_VERSION = "5.1")))]
    pub fn lua_strlen(L: *mut lua_State, idx: libc::c_int) -> usize;

    pub fn lua_stringtonumber(L: *mut lua_State, s: *const libc::c_char) -> usize;
    pub fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_tocfunction(L: *mut lua_State, idx: libc::c_int) -> lua_CFunction;

    // Defined as a macro in Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_tointeger(L: *mut lua_State, index: libc::c_int) -> lua_Integer;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_tointegerx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;

    pub fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut usize,
    ) -> *const libc::c_char;

    // Defined as a macro in Lua 5.2
    #[cfg(not(LUA_VERSION = "5.2"))]
    pub fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> lua_Number;

    // Introduced in Lua 5.2
    #[cfg(LUA_VERSION = "5.2")]
    pub fn lua_tonumberx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Number;

    pub fn lua_topointer(L: *mut lua_State, idx: libc::c_int) -> *const libc::c_void;
    pub fn lua_tothread(L: *mut lua_State, idx: libc::c_int) -> *mut lua_State;

    // Deprecated in Lua 5.3
    #[cfg(any(
        all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ))]
    pub fn lua_tounsignedx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Unsigned;

    pub fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
    pub fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    pub fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: libc::c_int);
    pub fn lua_yieldk(
        L: *mut lua_State,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
}

// //////////////////////////////////////////// //
// Lua Macros (represented as inline functions) //
// //////////////////////////////////////////// //

// Defined as a macro in Lua 5.2+
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int) {
    lua_callk(L, nargs, nresults, 0, None);
}

// Deprecated in Lua 5.2
#[cfg(all(LUA_VERSION = "5.2", feature = "lua-compat"))]
pub unsafe fn lua_cpcall(
    L: *mut lua_State,
    func: lua_CFunction,
    ud: *mut libc::c_void,
) -> libc::c_int {
    lua_pushcfunction(L, func);
    lua_pushlightuserdata(L, ud);
    lua_pcall(L, 1, 0, 0);
}

// Deprecated in Lua 5.2
#[cfg(all(LUA_VERSION = "5.2", feature = "lua-compat"))]
pub unsafe fn lua_equal(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int {
    lua_compare(L, idx1, idx2, LUA_OPEQ)
}

#[inline]
pub unsafe fn lua_getextraspace(L: *mut lua_State) -> *mut libc::c_void {
    (L as usize - LUA_EXTRASPACE) as *mut libc::c_void
}

#[inline]
pub unsafe fn lua_insert(L: *mut lua_State, index: libc::c_int) {
    lua_rotate(L, index, 1);
}

#[inline]
pub unsafe fn lua_isboolean(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) == LUA_TBOOLEAN) as libc::c_int
}

#[inline]
pub unsafe fn lua_isfunction(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) == LUA_TFUNCTION) as libc::c_int
}

#[inline]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) == LUA_TLIGHTUSERDATA) as libc::c_int
}

#[inline]
pub unsafe fn lua_isnil(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) == LUA_TNIL) as libc::c_int
}

#[inline]
pub unsafe fn lua_isnone(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) == LUA_TNONE) as libc::c_int
}

#[inline]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) <= 0) as libc::c_int
}

#[inline]
pub unsafe fn lua_istable(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) == LUA_TTABLE) as libc::c_int
}

#[inline]
pub unsafe fn lua_isthread(L: *mut lua_State, index: libc::c_int) -> libc::c_int {
    (lua_type(L, index) == LUA_TTHREAD) as libc::c_int
}

// Deprecated in Lua 5.2
#[cfg(all(LUA_VERSION = "5.2", feature = "lua-compat"))]
pub unsafe fn lua_lessthan(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int {
    lua_compare(L, idx1, idx2, LUA_OPLT)
}

#[inline]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0);
}

#[inline]
pub unsafe fn lua_numbertointeger(n: lua_Number, p: *mut lua_Integer) -> libc::c_int {
    if n >= LUA_MININTEGER as lua_Number && n < -(LUA_MININTEGER as lua_Number) {
        *p = n as lua_Integer;
        1
    } else {
        0
    }
}

// Defined as a macro in Lua 5.2+
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn lua_pcall(
    L: *mut lua_State,
    nargs: libc::c_int,
    nresults: libc::c_int,
    msgh: libc::c_int,
) -> libc::c_int {
    lua_pcallk(L, nargs, nresults, msgh, 0, None)
}

#[inline]
pub unsafe fn lua_pop(L: *mut lua_State, n: libc::c_int) {
    lua_settop(L, -n - 1);
}

#[inline]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0);
}

// Introduced in Lua 5.2
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn lua_pushglobaltable(L: *mut lua_State) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS);
}

#[inline]
pub unsafe fn lua_pushliteral(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char {
    lua_pushstring(L, s)
}

// Deprecated in Lua 5.3
#[cfg(all(LUA_VERSION = "5.3", feature = "lua-compat"))]
pub unsafe fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned) {
    lua_pushinteger(L, n as lua_Integer);
}

#[inline]
pub unsafe fn lua_register(L: *mut lua_State, name: *const libc::c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, name);
}

#[inline]
pub unsafe fn lua_remove(L: *mut lua_State, index: libc::c_int) {
    lua_rotate(L, index, -1);
    lua_pop(L, 1);
}

#[inline]
pub unsafe fn lua_replace(L: *mut lua_State, index: libc::c_int) {
    lua_copy(L, -1, index);
    lua_pop(L, 1);
}

// Deprecated in Lua 5.2
#[cfg(all(LUA_VERSION = "5.2", feature = "lua-compat"))]
pub unsafe fn lua_strlen(L: *mut lua_State, idx: libc::c_int) -> usize {
    lua_rawlen(L, idx)
}
#[cfg(all(LUA_VERSION = "5.1", not(LUA_VERSION = "5.2")))]
pub unsafe fn lua_strlen(L: *mut lua_State, idx: libc::c_int) -> usize {
    lua_objlen(L, idx)
}

// Defined as a macro in Lua 5.2
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn lua_tointeger(L: *mut lua_State, index: libc::c_int) -> lua_Integer {
    lua_tointegerx(L, index, ptr::null_mut())
}

// Defined as a macro in Lua 5.2
#[cfg_attr(LUA_VERSION = "5.2", inline)]
#[cfg(LUA_VERSION = "5.2")]
pub unsafe fn lua_tonumber(L: *mut lua_State, index: libc::c_int) -> lua_Number {
    lua_tonumberx(L, index, ptr::null_mut())
}

#[inline]
pub unsafe fn lua_tostring(L: *mut lua_State, index: libc::c_int) -> *const libc::c_char {
    lua_tolstring(L, index, ptr::null_mut())
}

// Deprecated in Lua 5.3
#[cfg_attr(
    any(
        all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")),
        all(LUA_VERSION = "5.3", feature = "lua-compat")
    ),
    inline
)]
#[cfg(any(
    all(LUA_VERSION = "5.2", not(LUA_VERSION = "5.3")),
    all(LUA_VERSION = "5.3", feature = "lua-compat")
))]
pub unsafe fn lua_tounsigned(L: *mut lua_State, index: libc::c_int) -> lua_Unsigned {
    lua_tounsignedx(L, index, ptr::null_mut())
}

#[inline]
pub const unsafe fn lua_upvalueindex(i: libc::c_int) -> libc::c_int {
    LUA_REGISTRYINDEX - i
}

#[inline]
pub unsafe fn lua_yield(L: *mut lua_State, nresults: libc::c_int) -> libc::c_int {
    lua_yieldk(L, nresults, 0, None)
}
