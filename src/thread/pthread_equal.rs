use crate::include::ctype::*;
use core::ptr::addr_eq;

#[no_mangle]
pub extern "C" fn pthread_equal(a: *mut pthread_t, b: *mut pthread_t) -> c_int
{
    addr_eq(a, b) as c_int
}