use crate::include::ctype::*;

#[no_mangle]
pub unsafe extern "C" fn pthread_rwlockattr_init(a: *mut pthread_rwlockattr_t) -> c_int
{
    (*a) = pthread_rwlockattr_t {
        __attr: core::mem::zeroed(),
    };
    0
}