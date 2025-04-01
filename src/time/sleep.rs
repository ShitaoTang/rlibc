use core::ptr;

use crate::include::ctype::*;
use crate::include::time::*;
use super::nanosleep::*;

#[no_mangle]
pub unsafe extern "C" fn sleep(seconds: c_uint) -> c_uint
{
    let mut tv: timespec = timespec {
        tv_sec: seconds as c_long,
        tv_nsec: 0,
    };
    if nanosleep(ptr::addr_of_mut!(tv), ptr::addr_of_mut!(tv)) != 0 {
        return tv.tv_sec as c_uint;
    }
    0
}