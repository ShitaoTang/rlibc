use crate::include::ctype::{c_int, c_long};
use crate::arch::syscall_arch::__syscall1 as syscall1;
use crate::arch::syscall_bits::*;

#[no_mangle]
pub unsafe extern "C" fn _Exit(code: c_int) -> !
{
    syscall1(SYS_exit_group as c_long, code as c_long);
    loop {
        syscall1(SYS_exit as c_long, code as c_long);
    }
}