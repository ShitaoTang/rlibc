use core::arch::asm;
use libc::c_int;
use libc::c_void;

// Load-Acquire Exclusive Register
// v <- *p
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_ll(p: *mut c_int) -> c_int {
    let mut v: c_int;
    unsafe {
        asm!(
            "ldaxr {0:w}, [{1}]",
            out(reg) v,
            in(reg) p,
            options(nostack, pure, readonly)
        );
    }
    v
}

// Store-Release Exclusive Register
// *p <- v
// r <- 1 if successful, 0 otherwise
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_sc(p: *mut c_int, v: c_int) -> c_int {
    let mut r: c_int;
    unsafe {
        asm!(
            "stlxr {0:w}, {2:w}, [{1}]",
            lateout(reg) r,
            in(reg) p,
            in(reg) v,
            options(nostack)
        );
    }
    (r == 0) as c_int
}

// Memory Barrier
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_barrier() {
    unsafe {
        asm!(
            "dmb ish",
            options(nostack)
        );
    }
}

// Compare-and-Swap (CAS)
// *p <- s if *p == t
// Returns the old value of *p
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_cas(p: *mut c_int, t: c_int, s: c_int) -> c_int {
    let mut old: c_int;
    loop {
        old = a_ll(p);
        if old != t {
            a_barrier();
            break;
        }
        if a_sc(p, s) != 0 {
            break;
        }
    }
    old
}

// Load-Acquire Exclusive Register (Pointer version)
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_ll_p(p: *mut *mut c_void) -> *mut c_void {
    let mut v: *mut c_void;
    unsafe {
        asm!(
            "ldaxr {0}, [{1}]",
            out(reg) v,
            in(reg) p,
            options(nostack, pure, readonly)
        );
    }
    v
}

// Store-Exclusive for Pointers
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_sc_p(p: *mut *mut c_void, v: *mut c_void) -> c_int {
    let mut r: c_int;
    unsafe {
        asm!(
            "stlxr {0:w}, {2}, [{1}]",
            lateout(reg) r,
            in(reg) p,
            in(reg) v,
            options(nostack)
        );
    }
    (r == 0) as c_int
}

// Compare-And-Swap (CAS for Pointer version)
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_cas_p(p: *mut *mut c_void, t: *mut c_void, s: *mut c_void) -> *mut c_void {
    let mut old: *mut c_void;
    loop {
        old = a_ll_p(p);
        if old != t {
            a_barrier();
            break;
        }
        if a_sc_p(p, s) != 0 {
            break;
        }
    }
    old
}

// Count Trailing Zeros for 64-bit Integers
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_ctz_64(mut x: u64) -> c_int {
    unsafe {
        asm!(
            "rbit {0}, {0}",
            inout(reg) x,
            options(nostack)
        );
        asm!(
            "clz {0}, {0}",
            inout(reg) x,
            options(nostack)
        );
    }
    x as c_int
}

// Count Leading Zeros for 64-bit Integers
#[inline(always)]
#[no_mangle]
pub extern "C" fn a_clz_64(mut x: u64) -> c_int {
    unsafe {
        asm!(
            "clz {0}, {0}",
            inout(reg) x,
            options(nostack)
        );
    }
    x as c_int
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_store(p: *mut c_int, v: c_int) {
    a_barrier();
    unsafe {*p = v;}
    a_barrier();
}
