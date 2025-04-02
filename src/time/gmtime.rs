use crate::include::ctype::*;
use crate::include::time::*;
use super::gmtime_r::*;
use core::ptr;

static mut tm: tm = tm {
    tm_sec: 0,
    tm_min: 0,
    tm_hour: 0,
    tm_mday: 0,
    tm_mon: 0,
    tm_year: 0,
    tm_wday: 0,
    tm_yday: 0,
    tm_isdst: 0,
    __tm_gmtoff: 0,
    __tm_zone: core::ptr::null(),
};

#[no_mangle]
pub unsafe extern "C" fn gmtime(t: *const time_t) -> *mut tm
{
    __gmtime_r(t, ptr::addr_of_mut!(tm))
}