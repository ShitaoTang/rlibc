use crate::include::ctype::*;

const ALIGN: usize = core::mem::size_of::<usize>();
const ONES: usize = usize::MAX/c_uchar::MAX as usize;
const HIGHS: usize = ONES * ((c_uchar::MAX as usize)/2 + 1);

#[inline]
fn has_zero(x: usize) -> bool
{
    x.wrapping_sub(ONES) & !x & HIGHS != 0
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> size_t
{
    let mut s = s as *const c_char;
    let a = s as *const c_char;
    type word = size_t;
    let mut w: *const word;

    while (s as uintptr_t) % ALIGN != 0 {
        if *s == 0 { return s as size_t - a as size_t; }
        s = s.add(1);
    }
    w = s as *const word;
    while !has_zero(*w) { w = w.add(1); }
    s = w as *const c_char;
    while *s != 0 { s = s.add(1); }

    s as size_t - a as size_t
}

#[no_mangle]
pub extern "C" fn strlen_test(s: *const c_char) -> size_t
{
    unsafe { strlen(s) }
}