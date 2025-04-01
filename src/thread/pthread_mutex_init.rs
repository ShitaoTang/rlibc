use crate::include::ctype::*;
use core::ptr;

#[no_mangle]
pub extern "C" fn pthread_mutex_init(m: *mut pthread_mutex_t, a: *const pthread_mutexattr_t) -> c_int
{
    if m.is_null() {
        return -1;
    }

    unsafe {
        core::ptr::write(
            m,
            pthread_mutex_t {
                __u: ptmu {
                    #[cfg(target_pointer_width = "64")]
                    __i: [0; 10],
                    #[cfg(target_pointer_width = "32")]
                    __i: [0; 6],
                },
            },
        );
        assert_eq!((*m).__u.__i[0], 0);
        assert_eq!(ptr::read_volatile(&(*m).__u.__vi[1]), 0);
        assert_eq!(ptr::read_volatile(&(*m).__u.__vi[2]), 0);
        assert_eq!(ptr::read_volatile(&(*m).__u.__p[3]), ptr::null_mut());
        assert_eq!(ptr::read_volatile(&(*m).__u.__p[4]), ptr::null_mut());
        assert_eq!((*m).__u.__i[5], 0);

        if !a.is_null() {
            (*m).__u.__i[0] = (*a).__attr as c_int;
        }
    }

    0
}