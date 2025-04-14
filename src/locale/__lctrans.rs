use crate::include::ctype::*;
use crate::internal::locale_impl::*;
use super::locale_map::__lctrans_impl;

#[no_mangle]
pub unsafe fn __lctrans(msg: *const c_char, lm: *const __locale_map) -> *const c_char
{
    __lctrans_impl(msg, lm)
}