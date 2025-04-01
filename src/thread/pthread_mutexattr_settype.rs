use crate::include::ctype::*;
use crate::arch::generic::bits::errno::EINVAL;

#[no_mangle]
pub unsafe extern "C" fn pthread_mutexattr_settype(a: *mut pthread_mutexattr_t, _type: c_int) -> c_int
{
    if _type as c_uint > 2 { return EINVAL; }
    (*a).__attr = ((*a).__attr & !0x3) | (_type as c_uint);
    0
}