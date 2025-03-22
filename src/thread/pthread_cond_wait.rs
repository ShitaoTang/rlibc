use crate::include::ctype::*;
use super::pthread_cond_timedwait::*;
use crate::include::time::*;

#[no_mangle]
pub extern "C" fn pthread_cond_wait(c: *mut pthread_cond_t, m: *mut pthread_mutex_t) -> c_int
{
    pthread_cond_timedwait(c, m, 0 as *const timespec)
}