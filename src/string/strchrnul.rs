use crate::include::ctype::*;
use super::strlen::strlen;
use core::mem::size_of;

#[no_mangle]
pub unsafe extern "C" fn strchrnul(s: *const c_char, c: c_int) -> *const c_char
{
    let c= c as c_uchar;
    let mut s= s as *const c_char;
    if c==0 { return s.add(strlen(s) as usize); }

    const ALIGN: usize = size_of::<usize>();
    const UCHAR_MAX: usize = c_uchar::MAX as usize;
    const ONES: usize = size_t::MAX / UCHAR_MAX;
    const HIGHS: usize = ONES * (UCHAR_MAX/2+1);

    #[inline(always)]
    fn has_zero(x: usize) -> bool
    {
        (x.wrapping_sub(ONES) & !x & HIGHS) != 0
    }

    while s as usize % ALIGN != 0 {
        if *s == 0 || *s as c_uchar == c {
            return s;
        }
        s = s.add(1);
    }

    let k = ONES * c as usize;
    let w= s as *const usize;
    let mut i = 0;
    loop {
        let word = *w.add(i);
        if has_zero(word) || has_zero(word^k) {
            break;
        }
        i += 1;
    }
    s = w.add(i) as *const c_char;
    
    while *s!=0 && *s as c_uchar != c {
        s = s.add(1);
    }

    s
}