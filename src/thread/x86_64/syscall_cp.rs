// use core::arch::asm;
// use super::super::pthread_cancel::cancel;

// #[inline(always)]
// #[no_mangle]
// pub unsafe extern "C" fn __syscall_cp_asm(
//     cancel_ptr: *const i32,
//     nr: i64,
//     u: i64,
//     v: i64,
//     w: i64,
//     x: i64,
//     _y: i64,
//     _z: i64,
// ) -> i64 {
//     let result: i64;
//     asm!(
//         // __cp_begin
//         "mov eax, [{cancel_ptr}]",
//         "test eax, eax",
//         "jnz 2f",
//         "mov r11, {cancel_ptr}",
//         "mov rax, {nr}",
//         "mov rdi, {u}",
//         "mov rsi, {v}",
//         "mov rdx, {w}",
//         "mov r10, {x}",
//         "mov r8, [rsp+8+56]",
//         "mov r9, [rsp+16+56]",
//         "mov [rsp+8], r11",
//         "syscall",
//         "mov [rsp], rax",
//         "mov rax, [rsp]",
//         "add rsp, $0x38",
//         "ret",

//         // __cp_cancel
//         "2:",
//         "jmp {cancel}",

//         cancel_ptr = in(reg) cancel_ptr,    // rdi
//         nr = in(reg) nr,    // rsi
//         u = in(reg) u,      // rdx
//         v = in(reg) v,      // rcx
//         w = in(reg) w,      // r8
//         x = in(reg) x,      // r9

//         cancel = sym cancel,
//         out("rax") result,
//         clobber_abi("C"),
//     );
//     result
// }

use core::arch::global_asm;

global_asm!(
    r#"
.text
.global __cp_begin
.global __cp_end
.global __cp_cancel
.global __syscall_cp_asm
.type   __syscall_cp_asm, @function
__syscall_cp_asm:

__cp_begin:
    mov eax, [rdi]
    test eax, eax
    jnz __cp_cancel
    mov r11, rdi
    mov rax, rsi
    mov rdi, rdx
    mov rsi, rcx
    mov rdx, r8
    mov r10, r9
    mov r8, [rsp+8]
    mov r9, [rsp+16]
    mov [rsp+8], r11
    syscall
__cp_end:
    ret
__cp_cancel:
    jmp __cancel
    "#
);