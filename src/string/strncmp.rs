use crate::include::ctype::*;

#[no_mangle]
pub unsafe extern "C" fn strncmp(_l: *const c_char, _r: *const c_char, _n: size_t) -> c_int
{
    let mut l = _l as *const c_uchar;
    let mut r = _r as *const c_uchar;
    let mut n = _n;
    if n==0 { return 0; } n -=1;

    while *l!=0 && *r!=0 && n!=0 && *l==*r {
        l = l.add(1);
        r = r.add(1);
        n -= 1;
    }

    *l as c_int - *r as c_int
}