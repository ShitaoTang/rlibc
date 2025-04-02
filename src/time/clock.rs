use crate::include::{ctype::*, time::CLOCK_PROCESS_CPUTIME_ID};
use super::clock_gettime::*;
use crate::include::time::timespec;
use core::ptr;

#[no_mangle]
pub extern "C" fn clock() -> clock_t
{
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    
    if clock_gettime(CLOCK_PROCESS_CPUTIME_ID, ptr::addr_of_mut!(ts)) != 0 {
        return -1;
    }

    if ts.tv_sec > c_long::MAX/1000000
        || ts.tv_nsec/1000 > c_long::MAX - 1000000*ts.tv_sec {
        return -1;
    }

    ts.tv_sec*1000000 + ts.tv_nsec/1000
}