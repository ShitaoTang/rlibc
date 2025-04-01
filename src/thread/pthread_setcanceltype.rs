use crate::include::ctype::*;
use crate::arch::generic::bits::errno::EINVAL;
use super::pthread_self::*;
use super::pthread_testcancel::*;

#[no_mangle]
pub unsafe extern "C" fn pthread_setcanceltype(new: c_int, old: *mut c_int) -> c_int
{
    if new as c_uint > 1 { return EINVAL; }
    if !old.is_null() { *old = (*pthread_self()).cancelasync as c_int; }
    (*pthread_self()).cancelasync = new as c_uchar;
    if new == 0 { pthread_testcancel(); }
    0
}