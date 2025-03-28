// use core::arch::global_asm;

// #[cfg(target_arch = "x86_64")]
// global_asm!(
//     r#"
// .section .init
//     pop rax
//     ret

// .section .fini
//     pop rax
//     ret
//     "#
// );