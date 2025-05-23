use crate::include::ctype::*;
use crate::internal::syscall::socketcall_cp;
use super::{sockaddr, socklen_t};
use crate::arch::syscall_bits::*;

#[no_mangle]
pub extern "C" fn accept(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int
{
    socketcall_cp(SYS_accept as c_int, fd as c_long, addr as c_long, len as c_long, 0, 0, 0) as c_int
}