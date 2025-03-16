use core::arch::asm;
use libc::{c_int, c_void};

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_cas(p: *mut c_int, t: c_int, s: c_int) -> c_int
{
    let old: c_int;
    unsafe {
        asm!(
            "lock cmpxchg [{1}], {0:e}",
            in(reg) s,
            in(reg) p,
            inout("eax") t => old,
            options(nostack, preserves_flags)
        );
    }
    old
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_cas_p(p: *mut *mut c_void, t: *mut c_void, s: *mut c_void) -> *mut c_void
{
    let old: *mut c_void;
    unsafe {
        asm!(
            "lock cmpxchg [{1}], {0:r}",
            in(reg) s,
            in(reg) p,
            inout("rax") t => old,
            options(nostack, preserves_flags)
        );
    } 
    old
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_swap(p: *mut c_int, v: c_int) -> c_int
{
    let r: c_int;
    unsafe {
        asm!(
            "xchg [{1}], {0:e}",
            inout(reg) v => r,
            in(reg) p,
            options(nostack, preserves_flags)
        );
    }
    r
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_fetch_add(p: *mut c_int, v: c_int) -> c_int
{
    let old: c_int;
    unsafe {
        asm!(
            "lock xadd [{1}], {0:e}",
            inout(reg) v => old,
            in(reg) p,
            options(nostack)
        );
    }
    old
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_and(p: *mut c_int, v: c_int)
{
    unsafe {
        asm!(
            "lock and [{0}], {1:e}",
            in(reg) p,
            in(reg) v,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_or(p: *mut c_int, v: c_int)
{
    unsafe {
        asm!(
            "lock or [{0}], {1:e}",
            in(reg) p,
            in(reg) v,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_xor(p: *mut c_int, v: c_int)
{
    unsafe {
        asm!(
            "lock xor [{0}], {1:e}",
            in(reg) p,
            in(reg) v,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_and_64(p: *mut u64, v: u64)
{
    unsafe {
        asm!(
            "lock and [{0}], {1}",
            in(reg) p,
            in(reg) v,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_or_64(p: *mut u64, v: u64)
{
    unsafe {
        asm!(
            "lock or [{0}], {1}",
            in(reg) p,
            in(reg) v,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_inc(p: *mut c_int)
{
    unsafe {
        asm!(
            "lock add dword ptr [{0}], 1",
            in(reg) p,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_dec(p: *mut c_int)
{
    unsafe {
        asm!(
            "lock sub dword ptr [{0}], 1",
            in(reg) p,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_store(p: *mut c_int, x: c_int)
{
    unsafe {
        asm!(
            "mov [{0}], {1:e}",
            in(reg) p,
            in(reg) x,
            options(nostack)
        );
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_barrier()
{
    unsafe {
        asm!("", options(nostack, preserves_flags));
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_spin()
{
    unsafe {
        asm!("pause", options(nostack, preserves_flags));
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_crash()
{
    unsafe {
        asm!("hlt", options(nostack, preserves_flags));
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_ctz_64(mut x: u64) -> c_int
{
    unsafe {
        asm!(
            "bsf {0}, {0}",
            inout(reg) x,
            options(nostack)
        );
    }
    x as c_int
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn a_clz_64(mut x: u64) -> c_int
{
    unsafe {
        asm!(
            "bsr {0}, {0}",
            "xor {0}, $63",
            inout(reg) x,
            options(nostack)
        );
    }
   x as c_int
}
