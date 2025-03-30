use crate::include::ctype::*;
use core::arch::asm;

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn __clone(
    _func: unsafe extern "C" fn(*mut c_void) -> c_int,
    _stack: *mut c_void,
    _flags: c_int,
    _args: *mut c_void,
    _ptid: *mut c_int,
    _tls: *mut c_void,
    _ctid: *mut c_int
) -> c_int {
    let ret: c_int;
    asm!(
        "xor eax, eax",
        "mov al, 56",
        "mov r11, rdi",
        "mov rdi, rdx",
        "mov rdx, r8",
        "mov r8, r9",
        "mov r10, [rsp + 8 + 56]",   // magic number
        "mov r9, r11",
        "and rsi, 0xfffffffffffffff0",
        "sub rsi, 8",
        "mov [rsi], rcx",
        "syscall",
        "test eax, eax",
        "jnz 2f",
        "xor ebp, ebp",
        "pop rdi",
        "call r9",
        "mov edi, eax",
        "xor eax, eax",
        "mov al, 60",
        "syscall",
        "hlt",

        "2:",
        "mov [rsp + 4], rax",
        "mov rax, [rsp + 4]",
        "add rsp, 56",
        "ret",

        out ("rax") ret,   // rax
        clobber_abi("C"),  // clobber_abi
    );
    ret
}