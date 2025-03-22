use crate::include::ctype::*;
use crate::internal::syscall::socketcall_cp;
use super::{sockaddr, socklen_t};
use crate::arch::syscall_bits::*;


#[no_mangle]
pub extern "C" fn recvfrom(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int, addr: *mut sockaddr, alen: *mut socklen_t) -> ssize_t
{
    socketcall_cp(SYS_recvfrom as c_int, fd as c_long, buf as c_long, len as c_long, flags as c_long, addr as c_long, alen as c_long) as ssize_t
}
