use crate::include::time::*;
use crate::include::ctype::*;
use super::nanosleep::*;

#[no_mangle]
pub unsafe extern "C" fn usleep(usec: c_uint) -> c_int
{
    let mut tv: timespec = timespec {
        tv_sec: (usec / 1000000) as c_long,
        tv_nsec: (usec % 1000000 * 1000) as c_long,
    };
    nanosleep(&mut tv, &mut tv)
}