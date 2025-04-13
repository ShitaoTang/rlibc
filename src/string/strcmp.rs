use crate::include::ctype::*;

#[no_mangle]
pub unsafe extern "C" fn strcmp(_l: *const c_char, _r: *const c_char) -> c_int
{
    let mut l = _l as *const c_uchar;
    let mut r = _r as *const c_uchar;

    while *l != 0 && *r != 0 && *l == *r {
        l = l.add(1);
        r = r.add(1);
    }

    (*l - *r) as c_int
}