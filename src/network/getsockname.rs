use crate::include::ctype::*;
use crate::internal::syscall::socketcall;
use super::{sockaddr, socklen_t};

#[no_mangle]
pub extern "C" fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int
{
    socketcall(libc::SYS_getsockname as c_int, fd as c_long, addr as c_long, len as c_long, 0, 0, 0) as c_int
}