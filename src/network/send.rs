use core::ptr;
use libc::{c_int, size_t, ssize_t, c_void};
use super::sendto::sendto;

#[no_mangle]
pub extern "C" fn send(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t
{
    sendto(fd, buf, len, flags, ptr::null_mut(), 0)
}