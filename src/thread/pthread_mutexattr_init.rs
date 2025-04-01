use crate::include::ctype::*;

#[no_mangle]
pub unsafe extern "C" fn pthread_mutexattr_init(a: *mut pthread_mutexattr_t) -> c_int
{
    (*a) = pthread_mutexattr_t {
        __attr: core::mem::zeroed(),
    };
    0
}