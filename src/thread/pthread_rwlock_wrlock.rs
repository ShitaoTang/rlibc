use crate::include::ctype::*;
use super::pthread_rwlock_timedwrlock::*;

#[no_mangle]
pub extern "C" fn pthread_rwlock_wrlock(rw: *mut pthread_rwlock_t) -> c_int
{
    pthread_rwlock_timedwrlock(rw, 0 as *const libc::timespec)
}