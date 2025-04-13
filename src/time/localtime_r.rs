use crate::include::ctype::*;
use crate::include::time::*;
use crate::thread::pthread_self::pthread_self;
use crate::arch::generic::bits::errno::EOVERFLOW;
use super::__tz::__secs_to_zone;
use super::__secs_to_tm::*;
use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn localtime_r(t: *const time_t, tm: *mut tm) -> *mut tm
{
    if *t < (c_int::MIN as time_t)*MAX_SEC_YEAR
        || *t > (c_int::MAX as time_t)*MAX_SEC_YEAR {
        return ptr::null_mut();
    }

    __secs_to_zone(
        *t,
        0,
        ptr::addr_of_mut!((*tm).tm_isdst),
        ptr::addr_of_mut!((*tm).__tm_gmtoff),
        ptr::null_mut(),
        ptr::addr_of_mut!((*tm).__tm_zone)
    );

    if __secs_to_tm(*t as c_longlong + (*tm).__tm_gmtoff, tm) < 0 {
        (*pthread_self()).errno_val = EOVERFLOW;
        return ptr::null_mut();
    }

    tm
}