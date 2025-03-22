use core::ptr;
use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_rwlock_init(rw: *mut pthread_rwlock_t, a: *const pthread_rwlockattr_t) -> c_int
{
    if rw.is_null() {return -1;}

    unsafe {
        ptr::write(rw, core::mem::zeroed::<pthread_rwlock_t>());
        assert_eq!(ptr::read_volatile(&(*rw).__u.__vi[0]), 0);
        assert_eq!(ptr::read_volatile(&(*rw).__u.__vi[1]), 0);
        assert_eq!((*rw).__u.__i[2], 0);
    }

    if !a.is_null() {
        unsafe {(*rw).__u.__i[2] = ((*a).__attr[0] * 128) as c_int;}
    }

    0
}