use crate::include::ctype::{c_int, c_long};
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
use crate::__syscall;

#[no_mangle]
pub unsafe extern "C" fn _Exit(code: c_int) -> !
{
    // __syscall1(SYS_exit_group as c_long, code as c_long);
    // loop {
    //     __syscall1(SYS_exit as c_long, code as c_long);
    // }
    __syscall!(SYS_exit_group, code);
    loop {
        __syscall!(SYS_exit, code);
    }
}