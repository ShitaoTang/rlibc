use crate::include::ctype::*;
use core::arch::global_asm;

extern "C" {
    pub fn __set_thread_area(p: *mut c_void) -> c_int;
}

#[cfg(target_arch = "x86_64")]
global_asm!(
    r#"
.text
.global __set_thread_area
.type __set_thread_area, @function
__set_thread_area:
    mov rsi, rdi
    mov edi, 0x1002
    mov eax, 158
    syscall
    ret
    "#
);

#[cfg(target_arch = "aarch64")]
global_asm!(
    r#"
.global __set_thread_area
.type __set_thread_area, @function
__set_thread_area:
    msr TPIDR_EL0, x0
    mov w0, #0
    ret
    "#
);