use core::arch::asm;
use crate::include::ctype::*;

#[inline(always)]
pub unsafe fn __syscall0(n: c_long) -> c_long {
    let x8 = n;
    let mut x0: c_long;
    asm!(
        "svc 0",
        in("x8") x8,
        lateout("x0") x0,
        options(nostack)
    );
    x0
}

#[inline(always)]
pub unsafe fn __syscall1(n: c_long, a: c_long) -> c_long {
    let x8 = n;
    let mut x0 = a;
    asm!(
        "svc 0",
        in("x8") x8,
        inout("x0") x0,
        options(nostack)
    );
    x0
}

#[inline(always)]
pub unsafe fn __syscall2(n: c_long, a: c_long, b: c_long) -> c_long {
    let x8 = n;
    let mut x0 = a;
    let x1 = b;
    asm!(
        "svc 0",
        in("x8") x8,
        inout("x0") x0,
        in("x1") x1,
        options(nostack)
    );
    x0
}

#[inline(always)]
pub unsafe fn __syscall3(n: c_long, a: c_long, b: c_long, c: c_long) -> c_long {
    let x8 = n;
    let mut x0 = a;
    let x1 = b;
    let x2 = c;
    asm!(
        "svc 0",
        in("x8") x8,
        inout("x0") x0,
        in("x1") x1,
        in("x2") x2,
        options(nostack)
    );
    x0
}

#[inline(always)]
pub unsafe fn __syscall4(n: c_long, a: c_long, b: c_long, c: c_long, d: c_long) -> c_long {
    let x8 = n;
    let mut x0 = a;
    let x1 = b;
    let x2 = c;
    let x3 = d;
    asm!(
        "svc 0",
        in("x8") x8,
        inout("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        options(nostack)
    );
    x0
}

#[inline(always)]
pub unsafe fn __syscall5(n: c_long, a: c_long, b: c_long, c: c_long, d: c_long, e: c_long) -> c_long {
    let x8 = n;
    let mut x0 = a;
    let x1 = b;
    let x2 = c;
    let x3 = d;
    let x4 = e;
    asm!(
        "svc 0",
        in("x8") x8,
        inout("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        in("x4") x4,
        options(nostack)
    );
    x0
}

#[inline(always)]
pub unsafe fn __syscall6(n: c_long, a: c_long, b: c_long, c: c_long, d: c_long, e: c_long, f: c_long) -> c_long {
    let x8 = n;
    let mut x0 = a;
    let x1 = b;
    let x2 = c;
    let x3 = d;
    let x4 = e;
    let x5 = f;
    asm!(
        "svc 0",
        in("x8") x8,
        inout("x0") x0,
        in("x1") x1,
        in("x2") x2,
        in("x3") x3,
        in("x4") x4,
        in("x5") x5,
        options(nostack)
    );
    x0
}