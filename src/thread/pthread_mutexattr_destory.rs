use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_mutexattr_destroy(_a: *mut pthread_mutexattr_t) -> c_int
{
    0
}