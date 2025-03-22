use crate::include::ctype::*;
use core::ptr;
use super::pthread_mutex_timedlock::*;
use crate::arch::atomic_arch::*;

#[no_mangle]
pub extern "C" fn pthread_mutex_lock(m: *mut pthread_mutex_t) -> c_int
{
    if m.is_null() {
        return -1;
    }

    if ((unsafe{(*m)._m_type()} & 15) == libc::PTHREAD_MUTEX_NORMAL)
     && a_cas(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, libc::EBUSY) == 0 {
        return 0;
    }

    pthread_mutex_timedlock(m, 0 as *const libc::timespec)
}