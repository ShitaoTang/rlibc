use core::ptr;
use crate::include::ctype::*;
use super::sendto::sendto;

#[no_mangle]
pub extern "C" fn send(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t
{
    sendto(fd, buf, len, flags, ptr::null_mut(), 0)
}