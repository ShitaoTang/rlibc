use crate::include::ctype::*;

#[no_mangle]
pub unsafe extern "C" fn strspn(s: *const c_char, c: *const c_char) -> size_t {
    if s.is_null() || c.is_null() {
        return 0;
    }

    let a = s as *const c_char;
    let mut s = s;
    let mut c = c;
    let mut byteset = [0 as size_t; 32 / core::mem::size_of::<usize>()];

    if *c == 0 { return 0; }
    if *c.add(1) == 0 {
        while *s == *c { s = s.add(1); }
        return s as size_t - a as size_t;
    }

    while *c != 0 {
        byteset[*(c as *const c_uchar) as size_t / (8*core::mem::size_of::<size_t>())] |=
            (1 as size_t) << (*(c as *const c_uchar) as size_t % (8*core::mem::size_of::<size_t>()));
        c = c.add(1);
    }

    while *s != 0 {
        if byteset[*(s as *const c_uchar) as size_t / (8*core::mem::size_of::<size_t>())] &
            ((1 as size_t) << (*(s as *const c_uchar) as size_t % (8*core::mem::size_of::<size_t>()))) == 0 {
            break;
        }
        s = s.add(1);
    }

    s as size_t - a as size_t
}