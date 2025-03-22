use crate::include::ctype::*;
use crate::arch::atomic_arch::*;
use core::ptr;
use super::pthread_impl::*;

#[no_mangle]
pub extern "C" fn pthread_rwlock_unlock(rw: *mut pthread_rwlock_t) -> c_int
{
    let mut val: c_int;
    let mut cnt: c_int;
    let mut waiters: c_int;
    let mut new: c_int;
    let lock_priv: c_int = unsafe{(*rw)._rw_shared()} ^ 128;

    loop {
        val = unsafe {(*rw)._rw_lock()};
        cnt = val & 0x7fffffff;
        waiters = unsafe {(*rw)._rw_waiters()};
        if cnt == 0x7fffffff || cnt == 1 {
            new = 0;
        } else {
            new = val - 1;
        }
        if a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, val, new) == val {break;}
    }

    if new == 0 && (waiters != 0 || val < 0) {
        wake(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, cnt, lock_priv);
    }

    0
}