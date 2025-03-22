use core::arch::asm;
use crate::include::ctype::*;

#[inline(always)]
pub fn __get_tp() -> uintptr_t
{
    let tp: uintptr_t;
    unsafe {
        asm!(
            "mov {0}, fs:0", 
            out(reg) tp,
            options(nostack, preserves_flags)
        );
    }
    tp
}