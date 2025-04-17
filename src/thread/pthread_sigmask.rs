use crate::arch::bits::signal::_NSIG;
use crate::include::ctype::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
use crate::include::signal::SIG_BLOCK;
use crate::arch::generic::bits::errno::EINVAL;

#[no_mangle]
pub unsafe extern "C" fn pthread_sigmask(how: c_int, set: *const sigset_t, old: *mut sigset_t) -> c_int
{
    if !set.is_null() && (how as c_ulong).wrapping_sub(SIG_BLOCK as c_ulong) > 2 {
        return EINVAL;
    }

    let ret = -__syscall4(
        SYS_rt_sigprocmask as c_long,
        how as c_long,
        set as c_long,
        old as c_long,
        _NSIG as c_long,
    );
    if ret==0 && !old.is_null() {
        if size_of::<c_ulong>() == 8 {
            (*old).__bits[0] &= !0x380000000;
        } else {
            (*old).__bits[0] &= !0x80000000;
            (*old).__bits[1] &= !0x3;
        }
    }

    ret as c_int
}