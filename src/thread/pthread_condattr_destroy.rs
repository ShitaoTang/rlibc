use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_condattr_destroy(_a: *mut pthread_condattr_t) -> c_int
{
    0
}