use crate::include::ctype::*;
use core::ptr;

#[no_mangle]
pub extern "C" fn pthread_barrierattr_init(a: *mut pthread_barrierattr_t) -> c_int
{
    unsafe {
        ptr::write(a, core::mem::zeroed::<pthread_barrierattr_t>());
    }
    0
}