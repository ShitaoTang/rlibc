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