use crate::include::{bit::*, ctype::*};

#[no_mangle]
pub unsafe extern "C" fn atoi(s: *const c_char) -> c_int
{
    let mut s = s;
    let mut n: c_int = 0;
    let mut neg: bool = false;
    while isspace(*s as c_int)!=0 { s = s.add(1); }
    if *s == b'-' as c_char { neg = true; s = s.add(1); }
    else if *s == b'+' as c_char { s = s.add(1); }

    while isdigit(*s as c_int) {
        n = n.wrapping_mul(10).wrapping_add(*s as c_int - '0' as c_int);
        s = s.add(1);
    }

    return if neg {-n} else {n};
}