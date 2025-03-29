use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(
    r#"
.section .init
.global _init
_init:
    push rax
    pop rax
    ret

.section .fini
.global _fini
_fini:
    push rax
    pop rax
    ret
    "#
);

#[cfg(target_arch = "aarch64")]
global_asm!(
    r#"
.section .init
.global _init
.type _init, %function
_init:
    stp x29, x30, [sp, -16]!
    mov x29, sp
    ldp x29, x30, [sp], 16
    ret

.section .fini
.global _fini
.type _fini, %function
_fini:
    stp x29, x30, [sp, -16]!
    mov x29, sp
    ldp x29, x30, [sp], 16
    ret
    "#
);