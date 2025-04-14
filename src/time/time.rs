use crate::include::ctype::*;
use crate::include::time::*;
use super::clock_gettime::*;

#[no_mangle]
pub unsafe extern "C" fn time(t: *mut time_t) -> time_t
{
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if clock_gettime(CLOCK_REALTIME, &mut ts) != 0 {
        return -1;
    }
    if !t.is_null() {
        *t = ts.tv_sec;
    }
    return ts.tv_sec;
}