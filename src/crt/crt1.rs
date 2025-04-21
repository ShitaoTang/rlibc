use core::arch::global_asm;
use crate::include::ctype::*;
use crate::env::__libc_start_main::*;

// only support static linking
#[cfg(target_arch = "x86_64")]
global_asm!(
    r#"
.text
.global _start
.type _start, @function
_start:
    xor rbp, rbp
    mov rdi, rsp
    mov rsi, 0
    and rsp, 0xFFFFFFFFFFFFFFF0
    call _start_c
    .size _start, . - _start
    "#
);

#[cfg(target_arch = "aarch64")]
global_asm!(
    r#"
.text
.global _start
.type _start, @function
_start:
    mov x29, 0
    mov x30, 0
    mov x0, sp
    adrp x1, 0x400000
    add x1, x1, 0
    and sp, x0, 0xFFFFFFFFFFFFFFF0
    b _start_c
    .size _start, . - _start
    "#
);

extern "C" {
    fn main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
    fn _init();
    fn _fini();
}

extern "C" fn safe_main(argc: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int {
    unsafe { main(argc, argv, envp) }
}

extern "C" fn safe_init() {
    unsafe { _init() }
}

extern "C" fn safe_fini() {
    unsafe { _fini() }
}

#[no_mangle]
pub unsafe extern "C" fn _start_c(p: *mut c_long) -> !
{
    let argc = *p as c_int;
    let argv = p.add(1) as *mut *mut c_char;

    __libc_start_main(
        safe_main,
        argc,
        argv,
        Some(safe_init),
        Some(safe_fini),
        None,
    );
}