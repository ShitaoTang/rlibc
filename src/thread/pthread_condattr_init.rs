use crate::include::ctype::*;
use core::ptr;

#[no_mangle]
pub extern "C" fn pthread_condattr_init(a: *mut pthread_condattr_t) -> c_int
{
    unsafe {
        ptr::write(a, core::mem::zeroed::<pthread_condattr_t>());
    }
    0
}