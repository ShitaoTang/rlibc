use crate::{
    include::ctype::*, 
    internal::{intscan::__intscan, shgetc::{sh_fromstring, shcnt, shlim}}
};

#[no_mangle]
pub extern "C" fn strtox(s: *const c_char, p: *mut *mut c_char, base: c_int, lim: c_ulonglong) -> c_ulonglong
{
    let mut f: FILE = unsafe { core::mem::zeroed() };
    sh_fromstring(&mut f, s);
    shlim(&mut f, 0);
    let y = __intscan(&mut f, base as c_uint, 1, lim);
    if !p.is_null() {
        let cnt: size_t = shcnt(&mut f) as size_t;
        unsafe { *p = s.offset(cnt as isize) as *mut c_char; }
    }
    y
}

#[no_mangle]
pub extern "C" fn strtoull(s: *const c_char, p: *mut *mut c_char, base: c_int) -> c_ulonglong
{
    strtox(s, p, base, c_ulonglong::MAX) as c_ulonglong
}

#[no_mangle]
pub extern "C" fn strtoll(s: *const c_char, p: *mut *mut c_char, base: c_int) -> c_longlong
{
    strtox(s, p, base, c_longlong::MIN as c_ulonglong) as c_longlong
}

#[no_mangle]
pub extern "C" fn strtoul(s: *const c_char, p: *mut *mut c_char, base: c_int) -> c_ulong
{
    strtox(s, p, base, c_ulong::MAX as c_ulonglong) as c_ulong
}

#[no_mangle]
pub extern "C" fn strtol(s: *const c_char, p: *mut *mut c_char, base: c_int) -> c_long
{
    strtox(s, p, base, c_long::MIN as c_ulonglong) as c_long
}

#[no_mangle]
pub extern "C" fn strtoimax(s: *const c_char, p: *mut *mut c_char, base: c_int) -> intmax_t
{
    strtoll(s, p, base)
}

#[no_mangle]
pub extern "C" fn strtoumax(s: *const c_char, p: *mut *mut c_char, base: c_int) -> uintmax_t
{
    strtoull(s, p, base)
}