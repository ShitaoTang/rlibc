use crate::include::ctype::*;
use core::ptr;
use crate::arch::generic::bits::errno::*;

#[no_mangle]
pub extern "C" fn pthread_barrier_init(b: *mut pthread_barrier_t, a: *const pthread_barrierattr_t, count: c_uint) -> c_int
{
    if count.wrapping_sub(1) > c_int::MAX as c_uint -1 {return EINVAL;}
    unsafe {
        if b.is_null() {return EINVAL;}
        let attr = if a.is_null() {0} else {(*a).__attr};
        ptr::write(
            b,
            pthread_barrier_t {
                __u: ptbu {
                    #[cfg(target_pointer_width = "64")]
                    __i: [0; 8],
                    #[cfg(target_pointer_width = "32")]
                    __i: [0; 5],
                },
            },
        );
        (*b).__u.__i[2] = ((count-1) | attr) as c_int;
    }

    0
}