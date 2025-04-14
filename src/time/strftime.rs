use core::ptr;
use crate::include::ctype::*;
use crate::include::langinfo::*;
use crate::include::nl_types::*;
use crate::include::time::*;
use crate::internal::locale_impl::*;
use crate::locale::langinfo::nl_langinfo_l;
use crate::stdlib::strtol::strtoul;
use crate::string::memcpy::memcpy;
use crate::string::strlen::strlen;
use super::__tm_to_secs::__tm_to_secs;
use super::__tz::__tm_to_tzname;

#[inline(always)]
fn is_leap(y: c_int) -> bool
{
    let mut y = y;
    if y>c_int::MAX-1900 { y -= 2000; }
    y += 1900;

    y%4==0 && (y%100!=0 || y%400==0)
}

/// ## ISO 8601
/// #### 1. Week starts on Monday
/// Monday is the first day of the week, and Sunday is the last.
/// #### 2. Week 1 is the week with the first Thursday of the year
#[no_mangle]
unsafe fn week_num(tm: &tm) -> c_int
{
    /* 0..365 */
    let yday = tm.tm_yday as c_uint;
    /* 0: Sunday, 1: Monday, ..., 6: Saturday */
    let wday = tm.tm_wday as c_uint;
    /* week_num of the first Monday, 
     * 0 means Monday of current date is of prev year  */
    let mut week = (yday + 7 - (wday+6)%7) / 7;

    /* if 1 Jan is Tuesday/Wednesday/Thursday,
     * the previous week is also in this year. */
    if (wday + 371 - yday - 2) % 7 < 2 {
        week += 1;
    }

    if week == 0 {          // beginning of the year
        week = 52;
        /* if 31 Dec of prev year is Thursday,
         * or Friday of a leap year, 
         * then the prev year has 53 weeks. */
        let dec31 = (wday + 7 - yday - 1) % 7;
        if dec31==4 || (dec31==5 && is_leap(tm.tm_year%400-1)) {
            week += 1;
        }
    } else if week == 53 {  // end of the year
        /* if 1 Jan of next year is not a Thursday,
         * and not a Wednesday of a leap year,
         * then the next year has 52 weeks. */
        let jan1 = (wday + 371 - yday) % 7;
        if jan1!=4 && (jan1!=3 || !is_leap(tm.tm_year%400)) {
            week = 1;
        }
    }
    
    week as c_int
}

#[no_mangle]
pub unsafe extern "C" fn strftime(s: *mut c_char, n: size_t, f: *const c_char, tm: *const tm) -> size_t
{
    strftime_l(s, n, f, &*tm, CURREN_LOCALE())
}

#[no_mangle]
pub unsafe extern "C" fn strftime_l(s: *mut c_char, n: size_t, f: *const c_char, tm: &tm, loc: locale_t) -> size_t
{
    let mut l: size_t = 0;
    let mut k: size_t = 0;
    let mut buf = [0 as c_char; 100];
    let mut f = f;

    while l < n {
        if *f == 0 {
            if l < n {
                *s.add(l) = 0;
            }
            return l;
        }
        if *f != b'%' as c_char {
            if l < n {
                *s.add(l) = *f;
            }
            l += 1;
            f = f.add(1);
            continue;
        }
        f = f.add(1);

        let mut pad = None;
        let mut plus = false;
        let mut width;

        if matches!(*f as u8, b'-'|b'_'|b'0') {
            pad = Some(*f);
            f = f.add(1);
        }
        if *f == b'+' as c_char {
            plus = true;
            f = f.add(1);
        }

        let mut endp = ptr::null_mut();
        if (*f as u8).is_ascii_digit() {
            width = strtoul(f, &mut endp, 10);
            f = endp;
        } else {
            width = 0;
            endp = f as *mut c_char;
        }

        if matches!(*endp as u8, b'C'|b'F'|b'G'|b'Y') {
            if width==0 && f!=endp {
                width = 1
            }
        } else {
            width = 0;
        }
        f = endp;
        if matches!(*f as u8, b'E'|b'O') { f = f.add(1); }
        let mut t = strftime_fmt_1(*f as u8, &mut k, tm, loc, pad, &mut buf);
        if t.is_null() { break; }
        if width != 0 {
            if matches!(*t as u8, b'+'|b'-') {
                t = t.add(1);
                k -= 1;
            }
            while *t as u8 == b'0' && (*t.add(1) as u8).is_ascii_digit() {
                t = t.add(1);
                k -= 1;
            }
            if width < k as u64 { width = k as u64; }

            let mut d = 0;
            while (t.add(d).read() as u8).is_ascii_digit() { d += 1; }

            if tm.tm_year < -1900 {
                if l < n {
                    *s.add(l) = b'-' as c_char;
                }
                l += 1;
                width = width.saturating_sub(1);
            } else if plus && d as u64+(width.saturating_sub(k as u64)) >= (if (*f as u8)==b'C' {3} else {5}) {
                if l < n {
                    *s.add(l) = b'+' as c_char;
                }
                l += 1;
                width = width.saturating_sub(1);
            }
            for _ in 0..width.saturating_sub(k as u64) {
                if l < n {
                    *s.add(l) = b'0' as c_char;
                }
                l += 1;
            }
        }

        if k as usize > n - l {
            k = n - l;
        }
        memcpy(s.add(l) as *mut c_void, t as *const c_void, k);
        l += k as usize;

        f = f.add(1);
    }

    if n != 0 {
        if l == n { l = n-1; }
        *s.add(l) = 0;
    }

    0
}

unsafe fn number(val: c_longlong, buf: *mut c_char, offset: size_t, width: size_t, pad: c_char) -> size_t
{
    let mut tmp = [0 as c_char; 32];
    let mut len = 0;
    let mut v = if val<0 { -val } else { val };

    if v == 0 {
        tmp[len] = b'0' as c_char;
        len += 1;
    } else {
        while v != 0 {
            tmp[len] = (v%10) as c_char + b'0' as c_char;
            v /= 10;
            len += 1;
        }
    }

    let mut digits = [0 as c_char; 32];
    for i in 0..len {
        digits[i] = tmp[len-i-1];
    }

    let mut written = len;
    if val < 0 { written += 1; }

    match pad as u8 {
    b'-' => {
        if val < 0 {
            *buf.add(offset) = b'-' as c_char;
            memcpy(buf.add(offset+1) as *mut c_void, digits.as_ptr() as *const c_void, len);
        } else {
            memcpy(buf.add(offset) as *mut c_void, digits.as_ptr() as *const c_void, len);
        }
        return written;
    }
    b'_' => {
        let total = if width>written { width } else { written };
        for i in 0..total - written {
            *buf.add(offset+i) = b' ' as c_char;
        }
        if val < 0 {
            *buf.add(offset + total - written) = b'-' as c_char;
            memcpy(buf.add(offset + total - written + 1) as *mut c_void, digits.as_ptr() as *const c_void, len);
        } else {
            memcpy(buf.add(offset + total - written) as *mut c_void, digits.as_ptr() as *const c_void, len);
        }
        return total;
    }
    b'0' | _ => {
        /* zero-padding by default */
        let total = if width>written { width } else { written };
        for i in 0..total - written {
            *buf.add(offset + i) = b'0' as c_char;
        }
        if val < 0 {
            *buf.add(offset + total - written) = b'-' as c_char;
            memcpy(buf.add(offset + total - written + 1) as *mut c_void, digits.as_ptr() as *const c_void, len);
        } else {
            memcpy(buf.add(offset + total - written) as *mut c_void, digits.as_ptr() as *const c_void, len);
        }
        return total;
    }}
}

#[no_mangle]
unsafe fn strftime_fmt_1(fbyte: u8, l: &mut size_t, tm: &tm, loc: locale_t, pad: Option<c_char>, buf: &mut [c_char; 100]) -> *const c_char
{
    let item: nl_item;
    let def_pad = b'0' as c_char;
    let pad = pad.unwrap_or(def_pad);
    let mut fmt = "-\0".as_ptr() as *const c_char;
    let mut width = 2;

    match fbyte {
    b'a' => {
        if (tm.tm_wday as c_uint) > 6 {
            *l = strlen(fmt);
            return fmt;
        }
        item = ABDAY_1 + tm.tm_wday;
        fmt = nl_langinfo_l(item, loc);
        *l = strlen(fmt);
        return fmt;
    }
    b'A' => {
        if (tm.tm_wday as c_uint) > 6 {
            *l = strlen(fmt);
            return fmt;
        }
        item = DAY_1 + tm.tm_wday;
        fmt = nl_langinfo_l(item, loc);
        *l = strlen(fmt);
        return fmt;
    }
    b'h' | b'b' => {
        if (tm.tm_mon as c_uint) > 11 {
            *l = strlen(fmt);
            return fmt;
        }
        item = ABMON_1 + tm.tm_mon;
        fmt = nl_langinfo_l(item, loc);
        *l = strlen(fmt);
        return fmt;
    }
    b'B' => {
        if (tm.tm_mon as c_uint) > 11 {
            *l = strlen(fmt);
            return fmt;
        }
        item = MON_1 + tm.tm_mon;
        fmt = nl_langinfo_l(item, loc);
        *l = strlen(fmt);
        return fmt;
    }
    b'c' => {
        item = D_T_FMT;
        fmt = nl_langinfo_l(item, loc);
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b'C' => {
        let val = (1900 + tm.tm_year) as c_longlong/ 100;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'd' => {
        let val = tm.tm_mday as c_longlong;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'D' => {
        fmt = b"%m/%d/%y\0".as_ptr() as *const c_char;
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b'e' => {
        let val = tm.tm_mday as c_longlong;
        let pad = if pad == b'0' as c_char { b'_' as c_char } else { pad };
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'F' => {
        fmt = b"%Y-%m-%d\0".as_ptr() as *const c_char;
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b'g' | b'G' => {
        let mut val = (tm.tm_year + 1900) as c_longlong;
        if tm.tm_yday<3 && week_num(tm)!=1 { val -= 1; }
        else if tm.tm_yday>360 && week_num(tm)==1 { val += 1; }
        if fbyte == b'g' { val %= 100; }
        else { width = 4; }
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'H' => {
        let val = tm.tm_hour as c_longlong;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'I' => {
        let mut val = tm.tm_hour as c_longlong;
        if val == 0 { val = 12; }
        else if val > 12 { val -= 12; }
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'j' => {
        let val = tm.tm_yday as c_longlong + 1;
        width = 3;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'm' => {
        let val = tm.tm_mon as c_longlong + 1;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'M' => {
        let val = tm.tm_min as c_longlong;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'n' => {
        *l = 1;
        return b"\n\0".as_ptr() as *const c_char;
    }
    b'p' => {
        item = if tm.tm_hour >= 12 { PM_STR } else { AM_STR };
        fmt = nl_langinfo_l(item, loc);
        *l = strlen(fmt);
        return fmt;
    }
    b'r' => {
        item = T_FMT_AMPM;
        fmt = nl_langinfo_l(item, loc);
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b'R' => {
        fmt = b"%H:%M\0".as_ptr() as *const c_char;
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b's' => {
        let val = __tm_to_secs(tm) - tm.__tm_gmtoff;
        width = 1;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'S' => {
        let val = tm.tm_sec as c_longlong;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b't' => {
        *l = 1;
        return b"\t\0".as_ptr() as *const c_char;
    }
    b'T' => {
        fmt = b"%H:%M:%S\0".as_ptr() as *const c_char;
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b'u' => {
        let val = if tm.tm_wday==0 { 7 } else { tm.tm_wday };
        width = 1;
        *l = number(val as c_longlong, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'U' => {
        let val = ((tm.tm_yday as c_uint + 7 - tm.tm_wday as c_uint) / 7) as c_longlong;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'V' => {
        let val = week_num(tm) as c_longlong;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'w' => {
        let val = tm.tm_wday as c_longlong;
        width = 1;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'W' => {
        let val = ((tm.tm_yday as c_uint + 7 - (tm.tm_wday as c_uint + 6)%7) / 7) as c_longlong;
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'x' => {
        item = D_FMT;
        fmt = nl_langinfo_l(item, loc);
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b'X' => {
        item = T_FMT;
        fmt = nl_langinfo_l(item, loc);
        *l = strftime_l(buf.as_mut_ptr(), buf.len(), fmt, tm, loc);
        if *l == 0 { return ptr::null(); }
        return buf.as_ptr();
    }
    b'y' => {
        let mut val = (tm.tm_year + 1900) as c_longlong % 100;
        if val < 0 { val = -val; }
        *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        return buf.as_ptr();
    }
    b'Y' => {
        let val = (tm.tm_year + 1900) as c_longlong;
        if val >= 10000 {
            (*buf)[0] = b'+' as c_char;
            *l = number(val, buf.as_mut_ptr(), 1, width, pad) + 1;
        } else {
            *l = number(val, buf.as_mut_ptr(), 0, width, pad);
        }
        return buf.as_ptr();
    }
    b'z' => {
        if tm.tm_isdst < 0 {
            *l = 0;
            return b"\0".as_ptr() as *const c_char;
        }
        let val = tm.__tm_gmtoff/3600*100 + tm.__tm_gmtoff%3600/60;
        let sign = if val>=0 { b'+' as c_char } else { b'-' as c_char };
        (*buf)[0] = sign as c_char;
        *l = number(val.abs(), buf.as_mut_ptr(), 1, 4, pad) + 1;
        return buf.as_ptr();
    }
    b'Z' => {
        if tm.tm_isdst < 0 {
            *l = 0;
            return b"\0".as_ptr() as *const c_char;
        }
        let fmt = __tm_to_tzname(tm);
        *l = strlen(fmt);
        if *l >= buf.len() {
            *l = buf.len() - 1;
        }
        memcpy(buf.as_mut_ptr() as *mut c_void, fmt as *const c_void, *l);
        return buf.as_ptr()
    }
    b'%' => {
        *l = 1;
        return b"%\0".as_ptr() as *const c_char;
    }
    _ => { return ptr::null(); }
    }
}
