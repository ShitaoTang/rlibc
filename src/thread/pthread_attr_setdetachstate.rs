use crate::include::ctype::*;
use crate::arch::generic::bits::errno::EINVAL;

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_setdetachstate(a: *mut pthread_attr_t, state: c_int) -> c_int
{
    if a.is_null() || state as u32 > 1u32 {
        return EINVAL;
    }

    (*a).__u.__i[3*__SU+0] = state;
    0
}