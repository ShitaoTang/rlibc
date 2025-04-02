use crate::include::ctype::*;
use crate::include::time::*;
use crate::thread::pthread_self::*;
use crate::arch::generic::bits::errno::*;
use super::__secs_to_tm::*;
use super::*;

#[no_mangle]
pub unsafe extern "C" fn __gmtime_r(t: *const time_t, tm: *mut tm) -> *mut tm
{
    if __secs_to_tm(*t, tm) < 0 {
        (*pthread_self()).errno_val = EOVERFLOW;
        return core::ptr::null_mut();
    }

    (*tm).tm_isdst = 0;
    (*tm).__tm_gmtoff = 0;
    (*tm).__tm_zone = __utc.as_ptr();
    tm
}