use crate::include::ctype::*;
use super::stpcpy::stpcpy;

#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char
{
    stpcpy(dest, src)
}