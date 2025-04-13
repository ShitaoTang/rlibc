use crate::include::ctype::*;
use crate::include::time::*;
use super::localtime_r::*;

/* tm should be static because it's the return value */
static mut tm: tm = unsafe {core::mem::zeroed()};

#[no_mangle]
pub unsafe extern "C" fn localtime(t: *const time_t) -> *mut tm
{
    localtime_r(t, core::ptr::addr_of_mut!(tm))
}