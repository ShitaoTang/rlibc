use crate::arch::bits::signal::_NSIG;
use crate::include::ctype::*;

#[no_mangle]
pub unsafe extern "C" fn sigfillset(set: *mut sigset_t) -> c_int
{
#[cfg(target_pointer_width = "32")]
{
    (*set).__bits[0] = 0x7fffffff;
    (*set).__bits[1] = 0xfffffffc;
    if _NSIG > 65 {
        (*set).__bits[2] = 0xffffffff;
        (*set).__bits[3] = 0xffffffff;
    }
}

#[cfg(target_pointer_width = "64")]
{
    (*set).__bits[0] = 0xfffffffc7fffffff;
    if _NSIG > 65 {
        (*set).__bits[1] = 0xffffffffffffffff;
    }
}
    0
}