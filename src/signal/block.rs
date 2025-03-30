use crate::arch::syscall_arch::__syscall4;
use crate::arch::syscall_bits::SYS_rt_sigprocmask;
use crate::include::ctype::*;
use crate::arch::bits::signal::*;
use crate::include::signal::SIG_BLOCK;

#[cfg(target_pointer_width = "32")]
const fn all_mask_values() -> &'static [c_ulong] {
    if _NSIG > 65 {
        &[!0, !0, !0, !0]
    } else {
        &[!0, !0]
    }
}

#[cfg(target_pointer_width = "64")]
const fn all_mask_values() -> &'static [c_ulong] {
    if _NSIG > 65 {
        &[!0, !0]
    } else {
        &[!0]
    }
}

const all_mask: &[c_ulong] = all_mask_values();

#[cfg(target_pointer_width = "32")]
const fn app_mask_values() -> &'static [c_ulong] {
    if _NSIG == 65 {
        &[0x7fffffff, 0xfffffffc]
    } else {
        &[0x7fffffff, 0xfffffffc, !0, !0]
    }
}

#[cfg(target_pointer_width = "64")]
const fn app_mask_values() -> &'static [c_ulong] {
    if _NSIG == 65 {
        &[0xfffffffc7fffffff]
    } else {
        &[0xfffffffc7fffffff, !0]
    }
}

const app_mask: &[c_ulong] = app_mask_values();

pub unsafe fn __block_all_sigs(set: *mut c_void)
{
    let _ = __syscall4(SYS_rt_sigprocmask as c_long,
        SIG_BLOCK as c_long,
        all_mask.as_ptr() as c_long,
        set as c_long,
        (_NSIG/8) as c_long) as c_long;
}

pub unsafe fn __block_app_sigs(set: *mut c_void)
{
    let _ = __syscall4(SYS_rt_sigprocmask as c_long,
        SIG_BLOCK as c_long,
        app_mask.as_ptr() as c_long,
        set as c_long,
        (_NSIG/8) as c_long) as c_long;
}

pub unsafe fn __restore_sigs(set: *const c_void)
{
    let _ = __syscall4(SYS_rt_sigprocmask as c_long,
        SIG_BLOCK as c_long,
        set as c_long,
        0,
        (_NSIG/8) as c_long) as c_long;
}