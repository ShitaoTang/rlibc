use crate::include::ctype::*;
use super::strchrnul::*;

#[no_mangle]
pub unsafe extern "C" fn strchr(s: *const c_char, c: c_int) -> *const c_char
{
    let r = strchrnul(s, c);
    if *(r as *const c_uchar) == *(c as *const c_uchar) {
        return r;
    } else {
        return core::ptr::null();
    }
}