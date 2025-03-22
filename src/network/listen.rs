use crate::include::ctype::*;
use crate::internal::syscall::socketcall;
use crate::arch::syscall_bits::*;

#[no_mangle]
pub extern "C" fn listen(fd: c_int, backlog: c_int) -> c_int
{
    socketcall(SYS_listen as c_int, fd as c_long, backlog as c_long, 0, 0, 0, 0) as c_int
}