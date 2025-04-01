use crate::include::ctype::*;
use super::pthread_self::*;

#[no_mangle]
pub unsafe extern "C" fn pthread_setspecific(k: pthread_key_t, x: *const c_void) -> c_int
{
    let _self = pthread_self();
    if ((*_self).tsd.add(k as usize)).read() != x as *mut c_void {
        (*_self).tsd.add(k as usize).write(x as *mut c_void);
        (*_self).tsd_used = 1;
    }
    0
}