use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_barrierattr_destroy(_a: *mut pthread_barrierattr_t) -> c_int
{
    0
}