use crate::include::ctype::*;
use super::{sockaddr, socklen_t};
use crate::internal::syscall::socketcall;

#[no_mangle]
pub extern "C" fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int
{
    socketcall(libc::SYS_bind as c_int, fd as c_long, addr as c_long, len as c_long, 0, 0, 0) as c_int
}