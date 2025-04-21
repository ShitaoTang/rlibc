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
//     y: i64,
//     z: i64,
// ) -> i64 {
//     let result: i64;
//     asm!(
//         // __cp_begin
//         "ldr w0, [{cancel_ptr}]",
//         "cbnz w0, 2f",
//         "mov x8, {nr}",
//         "mov x0, {u}",
//         "mov x1, {v}",
//         "mov x2, {w}",
//         "mov x3, {x}",
//         "mov x4, {y}",
//         "mov x5, {z}",
//         "svc 0",
//         "b 3f",

//         // __cp_cancel
//         "2:",
//         "b {cancel}",

//         // __cp_end
//         "3:",

//         cancel_ptr = in(reg) cancel_ptr,
//         nr = in(reg) nr,
//         u = in(reg) u,
//         v = in(reg) v,
//         w = in(reg) w,
//         x = in(reg) x,
//         y = in(reg) y,
//         z = in(reg) z,

//         cancel = sym cancel,
//         out("x0") result,
//         clobber_abi("C"),
//     );
//     result
// }

use core::arch::global_asm;

global_asm!(
    r#"
.global __cp_begin
.global __cp_cancel
.global __cp_end
.global __syscall_cp_asm
.type   __syscall_cp_asm, %function
__syscall_cp_asm:
__cp_begin:
    ldr w0, [x0]
    cbnz w0, __cp_cancel
    mov x8, x1
    mov x0, x2
    mov x1, x3
    mov x2, x4
    mov x3, x5
    mov x4, x6
    mov x5, x7
    svc 0
__cp_end:
    ret
__cp_cancel:
    b __cancel
    "#
);