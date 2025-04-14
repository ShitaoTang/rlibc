use crate::include::ctype::*;
use crate::include::langinfo::*;
use crate::include::nl_types::nl_item;
use crate::include::locale::*;
use crate::internal::locale_impl::*;

static C_TIME: &[u8] = b"Sun\0Mon\0Tue\0Wed\0Thu\0Fri\0Sat\0\
Sunday\0Monday\0Tuesday\0Wednesday\0Thursday\0Friday\0Saturday\0\
Jan\0Feb\0Mar\0Apr\0May\0Jun\0Jul\0Aug\0Sep\0Oct\0Nov\0Dec\0\
January\0February\0March\0April\0May\0June\0July\0August\0\
September\0October\0November\0December\0\
AM\0PM\0\
%a %b %e %T %Y\0\
%m/%d/%y\0\
%H:%M:%S\0\
%I:%M:%S %p\0\
\0\0\
%m/%d/%y\0\
0123456789\0\
%a %b %e %T %Y\0\
%H:%M:%S";

static C_MESSAGE: &[u8] = b"^[yY]\0^[nN]\0yes\0no";

static C_NUMERIC: &[u8] = b".\0";

/* Note: The returned string should not be modified by the caller,
 * even though it returns *mut c_char claimed in POSIX.
 * See: https://pubs.opengroup.org/onlinepubs/9699919799/functions/nl_langinfo.html */
#[no_mangle]
pub unsafe extern "C" fn nl_langinfo_l(item: nl_item, loc: locale_t) -> *mut c_char
{
    let cat = item >> 16;
    let idx = (item & 65535) as c_uint;
    let str: *mut c_char;

    if item == CODESET {
        return if !(*loc).cat[LC_CTYPE as usize].is_null() {
            b"UTF-8\0".as_ptr() as *mut c_char
        } else {
            b"ASCII\0".as_ptr() as *mut c_char
        };
    }

    match cat {
    LC_NUMERIC => {
        if idx > 1 {
            return b"\0".as_ptr() as *mut c_char;
        }
        str = getstr(C_NUMERIC.as_ptr(), idx);
    }
    LC_TIME => {
        if idx > 0x31 {
            return b"\0".as_ptr() as *mut c_char;
        }
        str = getstr(C_TIME.as_ptr(), idx);
    }
    LC_MONETARY => {
        if idx > 0 {
            return b"\0".as_ptr() as *mut c_char;
        }
        str = b"\0".as_ptr() as *mut c_char;
    }
    LC_MESSAGES => {
        if idx > 3 {
            return b"\0".as_ptr() as *mut c_char;
        }
        str = getstr(C_MESSAGE.as_ptr(), idx);
    }
    _ => {
        return b"\0".as_ptr() as *mut c_char;
    }}

    if cat != LC_NUMERIC && *str != 0 {
        return LCTRANS(str as *const c_char, cat, loc) as *mut c_char;
    }

    str
}

unsafe fn getstr(base: *const u8, idx: u32) -> *mut c_char
{
    let mut base = base;
    let mut idx = idx;
    while idx > 0 {
        while *base != 0 {
            base = base.add(1);
        }
        base = base.add(1); // skip the '\0'
        idx -= 1;
    }
    base as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn nl_langinfo(item: nl_item) -> *mut c_char
{
    nl_langinfo_l(item, CURREN_LOCALE())
}