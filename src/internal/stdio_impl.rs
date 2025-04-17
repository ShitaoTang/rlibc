use crate::include::ctype::*;
use core::ptr;
use super::locale_impl::__locale_struct;

pub const UNGET: size_t = 8;

pub const F_PERM: c_uint = 1;
pub const F_NORD: c_uint = 4;
pub const F_NOWR: c_uint = 8;
pub const F_EOF: c_uint = 16;
pub const F_ERR: c_uint = 32;
pub const F_SVB: c_uint = 64;
pub const F_APP: c_uint = 128;

#[repr(C)]
pub struct _IO_FILE {
    pub flags: c_uint,
    pub rpos: *mut c_uchar,
    pub rend: *mut c_uchar,
    // pub close: *mut extern "C" fn(*mut FILE) -> c_int,
    pub close: Option<extern "C" fn(*mut FILE) -> c_int>,
    pub wend: *mut c_uchar,
    pub wpos: *mut c_uchar,
    pub mustbezero1: *mut c_uchar,
    pub wbase: *mut c_uchar,
    // pub read: *mut extern "C" fn(*mut FILE, *mut c_uchar, size_t) -> size_t,
    pub read: Option<extern "C" fn(*mut FILE, *mut c_uchar, size_t) -> size_t>,
    // pub write: *mut extern "C" fn(*mut FILE, *const c_uchar, size_t) -> size_t,
    pub write: Option<extern "C" fn(*mut FILE, *const c_uchar, size_t) -> size_t>,
    // pub seek: *mut extern "C" fn(*mut FILE, off_t, c_int) -> off_t,
    pub seek: Option<extern "C" fn(*mut FILE, off_t, c_int) -> off_t>,
    pub buf: *mut c_uchar,
    pub buf_size: size_t,
    pub prev: *mut FILE,
    pub next: *mut FILE,
    pub fd: c_int,
    pub pipe_pid: c_int,
    pub lock_count: c_long,
    pub mode: c_int,
    pub lock: c_int,    // volatile
    pub lbf: c_int,     // line buffer
    pub cookie: *mut c_void,
    pub off: off_t,
    pub getln_buf: *mut c_char,
    pub mustbezero2: *mut c_void,
    pub shend: *mut c_uchar,
    pub shlim: off_t,
    pub shcnt: off_t,
    pub prev_locked: *mut FILE,
    pub next_locked: *mut FILE,
    pub locale: *mut __locale_struct,
}

impl Default for _IO_FILE {
    fn default() -> Self {
        _IO_FILE {
            flags: 0,
            rpos: ptr::null_mut(),
            rend: ptr::null_mut(),
            close: None,
            wend: ptr::null_mut(),
            wpos: ptr::null_mut(),
            mustbezero1: ptr::null_mut(),
            wbase: ptr::null_mut(),
            read: None,
            write: None,
            seek: None,
            buf: ptr::null_mut(),
            buf_size: 0,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            fd: -1, // invalid value
            pipe_pid: 0,
            lock_count: 0,
            mode: 0,
            lock: 0,
            lbf: 0,
            cookie: ptr::null_mut(),
            off: 0,
            getln_buf: ptr::null_mut(),
            mustbezero2: ptr::null_mut(),
            shend: ptr::null_mut(),
            shlim: 0,
            shcnt: 0,
            prev_locked: ptr::null_mut(),
            next_locked: ptr::null_mut(),
            locale: ptr::null_mut(),
        }
    }
}

// return 232 on 64-bit platform
#[no_mangle]
pub extern "C" fn get_size_of_io_file() -> usize {
    core::mem::size_of::<_IO_FILE>()
}

pub const MAYBE_WAITERS: c_int = 0x40000000;