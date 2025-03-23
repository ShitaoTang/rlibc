use crate::include::ctype::*;

#[no_mangle]
pub unsafe extern "C" fn memcmp(vl: *const c_void, vr: *const c_void, n: size_t) -> c_int
{
    let mut l = vl as *const c_uchar;
    let mut r = vr as *const c_uchar;
    let mut n = n;
    while n!=0 && *l == *r {
        n -= 1; l = l.add(1); r = r.add(1);
    }
    if n == 0 { 0 } 
    else { *l as c_int - *r as c_int }
}