use core::arch::asm;
use libc::{c_int, c_void};

#[inline(always)]
#[no_mangle]
pub unsafe fn a_cas(p: *mut c_int, t: c_int, s: c_int) -> c_int {
    let old: c_int;
    asm!(
        "lock ; cmpxchg {s}, [{p}]",
        inout("eax") t => old,
        in("r") s,
        in("r") p,
        options(nostack, preserves_flags)
    );
    old
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_cas_p(p: *mut *mut c_void, t: *mut c_void, s: *mut c_void) -> *mut c_void {
    let old: *mut c_void;
    asm!(
        "lock ; cmpxchg {s}, [{p}]",
        inout("rax") t => old,
        in("r") s,
        in("r") p,
        options(nostack, preserves_flags)
    );
    old
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_swap(p: *mut c_int, v: c_int) -> c_int {
    let old: c_int;
    asm!(
        "xchg {v}, [{p}]",
        inout("r") v => old,
        in("r") p,
        options(nostack)
    );
    old
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_fetch_add(p: *mut c_int, v: c_int) -> c_int {
    let old: c_int;
    asm!(
        "lock ; xadd {v}, [{p}]",
        inout("r") v => old,
        in("r") p,
        options(nostack)
    );
    old
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_and(p: *mut c_int, v: c_int) {
    asm!(
        "lock ; and {v}, [{p}]",
        in("r") v,
        in("r") p,
        options(nostack)
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_or(p: *mut c_int, v: c_int) {
    asm!(
        "lock ; or {v}, [{p}]",
        in("r") v,
        in("r") p,
        options(nostack)
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_and_64(p: *mut u64, v: u64) {
    asm!(
        "lock ; and {v}, [{p}]",
        in("r") v,
        in("r") p,
        options(nostack)
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_or_64(p: *mut u64, v: u64) {
    asm!(
        "lock ; or {v}, [{p}]",
        in("r") v,
        in("r") p,
        options(nostack)
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_inc(p: *mut c_int) {
    asm!(
        "lock ; incl [{p}]",
        in("r") p,
        options(nostack)
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_dec(p: *mut c_int) {
    asm!(
        "lock ; decl [{p}]",
        in("r") p,
        options(nostack)
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_store(p: *mut c_int, x: c_int) {
    asm!(
        "mov {x}, [{p}]",
        "lock ; orl $0, (%rsp)",
        in("r") x,
        in("r") p,
        options(nostack)
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_barrier() {
    asm!("", options(nostack, preserves_flags));
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_spin() {
    asm!("pause", options(nostack, preserves_flags));
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_crash() {
    asm!("hlt", options(nostack, preserves_flags));
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_ctz_64(mut x: u64) -> c_int {
    asm!(
        "bsf {0}, {0}",
        inout("r") x,
        options(nostack)
    );
    x as c_int
}

#[inline(always)]
#[no_mangle]
pub unsafe fn a_clz_64(mut x: u64) -> c_int {
    asm!(
        "bsr {0}, {0}",
        "xor $63, {0}",
        inout("r") x,
        options(nostack)
    );
    x as c_int
}
