use core::arch::asm;
use libc::uintptr_t;

#[inline(always)]
pub fn __get_tp() -> uintptr_t
{
    let tp: uintptr_t;
    unsafe {
        asm!("mrs {}, TPIDR_EL0", out(reg) tp);
    }
    tp
}