use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_rwlockattr_destroy(_a: *mut pthread_rwlockattr_t) -> c_int
{
    0
}