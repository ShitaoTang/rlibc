use core::arch::asm;
use super::super::pthread_cancel::cancel;

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn __syscall_cp_asm(
    cancel_ptr: *const i32,
    nr: i64,
    u: i64,
    v: i64,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
) -> i64 {
    let result: i64;
    asm!(
        // __cp_begin
        "ldr w0, [{cancel_ptr}]",
        "cbnz w0, 2f",
        "mov x8, {nr}",
        "mov x0, {u}",
        "mov x1, {v}",
        "mov x2, {w}",
        "mov x3, {x}",
        "mov x4, {y}",
        "mov x5, {z}",
        "svc 0",
        "b 3f",

        // __cp_cancel
        "2:",
        "b {cancel}",

        // __cp_end
        "3:",

        cancel_ptr = in(reg) cancel_ptr,
        nr = in(reg) nr,
        u = in(reg) u,
        v = in(reg) v,
        w = in(reg) w,
        x = in(reg) x,
        y = in(reg) y,
        z = in(reg) z,

        cancel = sym cancel,
        out("x0") result,
        clobber_abi("C"),
    );
    result
}
