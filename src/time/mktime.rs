use core::ptr;
use crate::arch::generic::bits::errno::EOVERFLOW;
use crate::include::ctype::*;
use crate::include::time::*;
use super::__tz::__secs_to_zone;
use super::__secs_to_tm::*;
use super::__tm_to_secs::*;
use crate::thread::pthread_self::*;

#[no_mangle]
pub unsafe extern "C" fn mktime(tm: *mut tm) -> time_t
{
    let tm = &mut *tm;
    let mut new: tm = core::mem::zeroed();
    let mut opp: c_long = 0;
    let mut t = __tm_to_secs(tm);

    __secs_to_zone(
        t,
        1,
        ptr::addr_of_mut!(new.tm_isdst),
        ptr::addr_of_mut!(new.__tm_gmtoff),
        &mut opp,
        ptr::addr_of_mut!(new.__tm_zone)
    );

    if tm.tm_isdst>=0 && new.tm_isdst!=tm.tm_isdst {
        t -= opp - new.__tm_gmtoff;
    }

    t -= new.__tm_gmtoff;
    if t as time_t != t {
        (*pthread_self()).errno_val = EOVERFLOW;
        return time_t::MAX;
    }

    __secs_to_zone(
        t,
        0,
        ptr::addr_of_mut!(new.tm_isdst),
        ptr::addr_of_mut!(new.__tm_gmtoff),
        &mut opp,
        ptr::addr_of_mut!(new.__tm_zone)
    );

    if __secs_to_tm(t + new.__tm_gmtoff, &mut new) < 0 {
        (*pthread_self()).errno_val = EOVERFLOW;
        return time_t::MAX;
    }

    *tm = new;
    t
}