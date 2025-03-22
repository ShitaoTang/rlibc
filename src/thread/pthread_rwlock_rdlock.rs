use crate::include::ctype::*;
use super::pthread_rwlock_timedrdlock::*;
use crate::include::time::*;

#[no_mangle]
pub extern "C" fn pthread_rwlock_rdlock(rw: *mut pthread_rwlock_t) -> c_int
{
    pthread_rwlock_timedrdlock(rw, 0 as *const timespec)
}