use crate::include::ctype::*;

use crate::internal::syscall::socketcall;
use crate::arch::syscall_bits::*;

#[no_mangle]
pub extern "C" fn shutdown(fd: c_int, how: c_int) -> c_int
{
    socketcall(SYS_shutdown as c_int, fd as c_long, how as c_long, 0, 0, 0, 0) as c_int
}