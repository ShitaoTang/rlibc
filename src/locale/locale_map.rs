use crate::include::ctype::*;
use crate::internal::locale_impl::*;
use super::__mo_lookup::__mo_lookup;

#[no_mangle]
pub unsafe fn __lctrans_impl(msg: *const c_char, lm: *const __locale_map) -> *const c_char
{
    let mut trans: *const c_char = core::ptr::null();
    if !lm.is_null() {
        trans = __mo_lookup((*lm).map, (*lm).map_size, msg);
    }
    return if !trans.is_null() {trans}
    else {msg}
}