use crate::include::ctype::*;
use crate::arch::generic::bits::errno::EINVAL;

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getguardsize(a: *mut pthread_attr_t, size: size_t) -> c_int
{
    if a.is_null() || size > usize::MAX/8 {
        return EINVAL;
    }

    (*a).__u.__s[1] = size as u64;
    0
}