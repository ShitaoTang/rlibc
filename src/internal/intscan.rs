use crate::{
    arch::generic::bits::errno::{EINVAL, ERANGE}, 
    include::{bit::isspace, ctype::*}
};
use super::shgetc::*;
use crate::thread::pthread_self::pthread_self;

static table: [c_uchar; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   255, 255, 255, 255, 255, 255,
    255, 10,  11,  12,  13,  14,  15,  16,  17,  18,  19,  20,  21,  22,  23,  24,
    25,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  255, 255, 255, 255, 255,
    255, 10,  11,  12,  13,  14,  15,  16,  17,  18,  19,  20,  21,  22,  23,  24,
    25,  26,  27,  28,  29,  30,  31,  32,  33,  34,  35,  255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

#[no_mangle]
pub extern "C" fn __intscan(f: &mut FILE, base: c_uint, pok: c_int, lim: c_ulonglong) -> c_ulonglong
{
    let val = &table[1..];
    let mut neg: c_int = 0;
    let mut c: c_int;
    let mut x: c_uint;
    let mut y: c_ulonglong = 0;
    let mut base = base;

    if base>36 || base==1 {
        let _self = pthread_self();
        unsafe { (*_self).errno_val = EINVAL; }
    }

    loop {
        c = shgetc(f);
        if isspace(c) == 0 {    // read until get a non-space character
            break;
        }
    }

    if (base==0 || base==16) && c==b'0' as c_int {
        c = shgetc(f);
        if (c|32)==b'x' as c_int {
            c = shgetc(f);
            if val[c as usize] as c_int > 16 {
                shunget(f);
                if pok!= 0 { shunget(f); }
                else { shlim(f, 0); }
                return 0;
            }
            base = 16;
        } else if base==0 {
            base = 8;
        }
    } else {
        if base ==0 { base = 10; }
        if val[c as usize] as c_uint >= base {
            shunget(f);
            shlim(f, 0);
            let _self = pthread_self();
            unsafe { (*_self).errno_val = EINVAL; }
            return 0;
        }
    }

    if base == 10 {
        x = 0;
        while ((c as c_uint).wrapping_sub(b'0' as c_uint) as c_uint)<10 && x<=c_uint::MAX/10-1 {
            x = x*10 + (c as c_uint - b'0' as c_uint);
            c = shgetc(f);
        }
        y = x as c_ulonglong;
        while ((c as c_uint).wrapping_sub(b'0' as c_uint) as c_uint)<10
         && y<=c_ulonglong::MAX/10
         && y*10 <= c_ulonglong::MAX - (c as c_ulong - b'0' as c_ulong) {
            y = y*10 + (c as c_ulong - b'0' as c_ulong);
            c = shgetc(f);
        }
        if ((c as c_uint).wrapping_sub(b'0' as c_uint) as c_uint) >= 10 { return done(f, y, neg, lim); }
    } else if (base & base-1)==0 {
        let bs = [0, 1, 2, 4, 7, 3, 6, 5][((0x17*base)>>5&7) as usize];
        x = 0;
        while (val[c as usize] as c_uint) < base
         && x <= c_uint::MAX/32 {
            x = x<<bs | val[c as usize] as c_uint;
            c = shgetc(f);
        }
        y = x as c_ulonglong;
        while (val[c as usize] as c_uint) < base
         && y <= c_ulonglong::MAX/(base as c_ulong)
         && y*base as c_ulong <= c_ulonglong::MAX - val[c as usize] as c_ulong {
            y = y*base as c_ulong + val[c as usize] as c_ulong;
            c = shgetc(f);
        }
    }
    if (val[c as usize] as c_uint) < base {
        while (val[c as usize] as c_uint) < base {
            c = shgetc(f);
        }
        let _self = pthread_self();
        unsafe { (*_self).errno_val = EINVAL; }
        y = lim;
        if (lim&1)!=0 { neg = 0; }
    }

    done(f, y, neg, lim)
}

#[no_mangle]
fn done(f: &mut FILE, y: c_ulonglong, neg: c_int, lim: c_ulonglong) -> c_ulonglong
{
    shunget(f);
    if y >= lim {
        if (lim&1)==0 && neg==0 {
            let _self = pthread_self();
            unsafe { (*_self).errno_val = EINVAL; }
            return lim-1;
        } else if y >lim {
            let _self = pthread_self();
            unsafe { (*_self).errno_val = ERANGE; }
            return lim;
        }
    }
    (y^neg as c_ulonglong)-neg as c_ulonglong
}