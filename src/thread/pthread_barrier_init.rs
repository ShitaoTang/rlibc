use crate::include::ctype::*;
use core::ptr;

#[no_mangle]
pub extern "C" fn pthread_barrier_init(b: *mut pthread_barrier_t, a: *const pthread_barrierattr_t, count: c_uint) -> c_int
{
    if count.wrapping_sub(1) > libc::INT_MAX as c_uint -1 {return libc::EINVAL;}
    unsafe {
        if b.is_null() {return libc::EINVAL;}
        let attr = if a.is_null() {0} else {(*a).__attr};
        ptr::write(b, core::mem::zeroed::<pthread_barrier_t>());
        (*b).__u.__i[2] = ((count-1) | attr) as c_int;
    }

    0
}