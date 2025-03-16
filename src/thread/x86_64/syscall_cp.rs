use core::arch::asm;
use crate::thread::pthread::cancel;

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
        "mov eax, [{cancel_ptr}]",
        "test eax, eax",
        "jnz 2f",
        "mov r11, {cancel_ptr}",
        "mov rax, {nr}",
        "mov rdi, {u}",
        "mov rsi, {v}",
        "mov rdx, {w}",
        "mov r10, {x}",
        "mov r8, {y}",
        "mov r9, {z}",
        "syscall",
        "jmp 3f",

        // __cp_cancel
        "2:",
        "jmp {cancel}",

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
        out("rax") result,
        clobber_abi("C"),
    );
    result
}