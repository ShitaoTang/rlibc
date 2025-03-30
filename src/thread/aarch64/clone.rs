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
        "and x1, x1, 0xfffffffffffffff0",
        "stp x0, x3, [x1, 0xfffffffffffffff0]!",

        "uxtw x0, w2",
        "mov x2, x4",
        "mov x3, x5",
        "mov x4, x6",
        "mov x8, 220",
        "svc 0",

        "cbz x0, 2f",
        "b 3f",

        "2:",
        "ldp x1, x0, [sp], 16",
        "blr x1",
        "mov x8, 93",
        "svc 0",

        "3:",

        out ("x0") ret,   // x0
        clobber_abi("C"),  // clobber_abi
    );
    ret
}