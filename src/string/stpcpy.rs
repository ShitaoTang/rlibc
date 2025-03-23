use crate::include::ctype::*;

const ALIGN: usize = core::mem::size_of::<usize>();
const ONES: usize = usize::MAX/c_uchar::MAX as usize;
const HIGHS: usize = ONES * ((c_uchar::MAX as usize)/2 + 1);

#[inline]
fn has_zero(x: usize) -> bool
{
    x.wrapping_sub(ONES) & !x & HIGHS != 0
}

// this is unsafe when dest.len <= src.len
#[no_mangle]
pub unsafe extern "C" fn stpcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char
{
    let mut d = dest as *mut c_uchar;
    let mut s = src as *const c_uchar;
    
    if s as usize % ALIGN == d as usize % ALIGN {
        while s as usize % ALIGN != 0 {
            *d = *s;
            if *s == 0 { return dest; }
            d = d.add(1); s = s.add(1);
        }
        let mut wd = d as *mut size_t;
        let mut ws = s as *const size_t;
        while !has_zero(*ws) {
            *wd = *ws; wd = wd.add(1); ws = ws.add(1);
        }
        d = wd as *mut c_uchar;
        s = ws as *const c_uchar;
    }
    while *s != 0 {
        *d = *s; d = d.add(1); s = s.add(1);
    }
    *d = 0; // end with '\0'
    d as *mut c_char
}