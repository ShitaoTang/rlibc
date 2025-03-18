use libc::{c_int, c_long, c_void, size_t, ssize_t};
use super::{sockaddr, socklen_t};
use crate::internal::syscall::socketcall_cp;

#[no_mangle]
pub extern "C" fn sendto(fd: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, alen: socklen_t) -> ssize_t
{
    socketcall_cp(libc::SYS_sendto as c_int, fd as c_long, buf as c_long, len as c_long, flags as c_long, addr as c_long, alen as c_long) as ssize_t
}