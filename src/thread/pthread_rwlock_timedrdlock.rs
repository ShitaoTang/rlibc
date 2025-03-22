use crate::include::ctype::*;
use crate::arch::atomic_arch::*;
use super::__timedwait::*;
use super::pthread_rwlock_tryrdlock::*;
use core::ptr;

#[no_mangle]
pub extern "C" fn pthread_rwlock_timedrdlock(rw: *mut pthread_rwlock_t, at: *const libc::timespec) -> c_int
{
    let mut r: c_int = pthread_rwlock_tryrdlock(rw);
    if r != libc::EBUSY {return r;}

    let mut spins: c_int = 100;
    while spins != 0 {
        if unsafe {(*rw)._rw_lock()} == 0 || unsafe {(*rw)._rw_waiters()} != 0 {
            break;
        }
        a_barrier();
        spins -= 1;
    }

    r = pthread_rwlock_tryrdlock(rw);
    while r == libc::EBUSY {
        r = unsafe {(*rw)._rw_lock()};
        if r == 0 || (r & 0x7fffffff)!=0x7fffffff {
            r = pthread_rwlock_tryrdlock(rw);
            continue;
        }
        let t = r | libc::INT_MIN;
        a_inc(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[1])});
        a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, r, t);
        r = timedwait(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, t, libc::CLOCK_REALTIME, at, unsafe{(*rw)._rw_shared()}^128);
        a_dec(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[1])});
        if r != 0 && r != libc::EINTR {return r;} 
        r = pthread_rwlock_tryrdlock(rw);
    }

    r
}