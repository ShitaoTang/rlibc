use crate::include::ctype::*;
use crate::arch::generic::bits::errno::*;

#[no_mangle]
pub extern "C" fn pthread_condattr_setclock(a: *mut pthread_condattr_t, clk: clockid_t) -> c_int
{
    if (clk < 0) || (clk as u32).wrapping_sub(2) < 2 {return EINVAL;}
    unsafe {
        (*a).__attr &= c_int::MIN as u32;
        (*a).__attr |= clk as u32;
    }
    0
}