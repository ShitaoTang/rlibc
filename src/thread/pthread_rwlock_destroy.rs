use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_rwlock_destroy(_rw: *mut pthread_rwlock_t) -> c_int
{
    0
}