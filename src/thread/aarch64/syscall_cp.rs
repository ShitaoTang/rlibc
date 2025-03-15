use core::arch::asm;

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn __syscall_cp_asm(cancel_ptr: *const i32, nr: i64, u: i64, v: i64, w: i64, x: i64, y: i64, z: i64) -> i64
{
    let result: i64;
    asm!(
        "ldr w0, [{cancel_ptr}]",
        "cbnz w0, 1f",
        "mov x8, {nr}",
        "mov x0, {u}",
        "mov x1, {v}",
        "mov x2, {w}",
        "mov x3, {x}",
        "mov x4, {y}",
        "mov x5, {z}",
        "svc #0",
        "b 2f",
        "1:",
        "bl cancel",
        "2:",

        cancel_ptr = in(reg) cancel_ptr,
        nr = in(reg) nr,
        u = in(reg) u,
        v = in(reg) v,
        w = in(reg) w,
        x = in(reg) x,
        y = in(reg) y,
        z = in(reg) z,

        out("x0") result,

        clobber_abi("C"),
    );
    result
}
