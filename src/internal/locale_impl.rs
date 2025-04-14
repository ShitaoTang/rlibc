use crate::include::ctype::*;
use crate::thread::pthread_self::*;
use crate::locale::__lctrans::*;

pub const LOCALE_NAME_MAX: usize = 23;

#[repr(C)]
pub struct __locale_map {
    pub map: *const c_void,
    pub map_size: size_t,
    pub name: [c_char; LOCALE_NAME_MAX+1],
    pub next: *mut __locale_map,
}

#[repr(C)]
pub struct __locale_struct {
    pub cat: [*const __locale_map; 6],
}

pub type locale_t = *mut __locale_struct;

#[no_mangle]
pub fn CURREN_LOCALE() -> locale_t
{
    unsafe {(*pthread_self()).locale}
}

pub unsafe fn LCTRANS(msg: *const c_char, lc: c_int, loc: locale_t) -> *const c_char
{
    __lctrans(msg, (*loc).cat[lc as usize])
}

