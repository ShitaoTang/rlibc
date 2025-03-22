use core::ptr;
use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_cond_init(c: *mut pthread_cond_t, a: *const pthread_condattr_t) -> c_int
{
    unsafe {
        ptr::write(c, core::mem::zeroed::<pthread_cond_t>());
        assert_eq!(ptr::read_volatile(&(*c).__u.__p[0]), ptr::null_mut());
        assert_eq!(ptr::read_volatile(&(*c).__u.__vi[2]), 0);
        assert_eq!(ptr::read_volatile(&(*c).__u.__vi[3]), 0);
        assert_eq!((*c).__u.__i[4], 0);
        assert_eq!(ptr::read_volatile(&(*c).__u.__vi[8]), 0);
        assert_eq!(ptr::read_volatile(&(*c).__u.__p[1]), ptr::null_mut());
        assert_eq!(ptr::read_volatile(&(*c).__u.__p[5]), ptr::null_mut());

        if !a.is_null() {
            (*c).__u.__i[4] = ((*a).__attr & 0x7fffffff) as c_int;
            if (*a).__attr >> 31 != 0 {
                ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[0]), usize::MAX as *mut c_void);
            }
        }
    }

    0
}