use libc::{c_int, c_void, size_t, ssize_t};
use super::recvfrom::recvfrom;
use core::ptr;

#[no_mangle]
pub extern "C" fn recv(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t
{
    recvfrom(fd, buf, len, flags, ptr::null_mut(), ptr::null_mut())
}