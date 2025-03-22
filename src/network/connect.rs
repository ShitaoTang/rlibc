use crate::include::ctype::*;
use crate::internal::syscall::socketcall_cp;
use super::{sockaddr, socklen_t};
use crate::arch::syscall_bits::*;

#[no_mangle]
pub extern "C" fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int
{
    socketcall_cp(SYS_connect as c_int, fd as c_long, addr as c_long, len as c_long, 0, 0, 0) as c_int
}