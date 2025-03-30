use crate::include::ctype::*;
use crate::include::stdio::BUFSIZ;
use crate::internal::stdio_impl::*;
use super::__stdio_read::*;
use super::__stdio_seek::*;
use super::__stdio_close::*;
use core::ptr;

static mut BUF: [c_uchar; BUFSIZ+UNGET] = [0; BUFSIZ+UNGET];
static mut __stdin_FILE: FILE = FILE{
    buf: ptr::addr_of_mut!(BUF) as *mut c_uchar,
    buf_size: (BUFSIZ+UNGET) - UNGET,
    fd: 0,
    flags: F_PERM | F_NOWR,
    read: Some(__stdio_read),
    seek: Some(__stdio_seek),
    close: Some(__stdio_close),
    lock: -1,
    rpos: ptr::null_mut(),
    rend: ptr::null_mut(),
    wend: ptr::null_mut(),
    wpos: ptr::null_mut(),
    mustbezero1: ptr::null_mut(),
    wbase: ptr::null_mut(),
    prev: ptr::null_mut(),
    next: ptr::null_mut(),
    pipe_pid: 0,
    lock_count: 0,
    mode: 0,
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
    write: None,
};

pub const stdin: *mut FILE = ptr::addr_of_mut!(__stdin_FILE);
pub static mut __stdin_used: *mut FILE = ptr::addr_of_mut!(__stdin_FILE);   // volatile