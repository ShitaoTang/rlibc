use crate::include::bit::isdigit;
use crate::include::ctype::*;
use crate::include::limits::*;
use crate::include::time::tm;
use crate::internal::lock::*;
use crate::string::memcmp::memcmp;
use crate::string::memcpy::*;
use crate::string::strcmp::*;
use crate::string::strlen::*;
use crate::string::strchr::*;
use super::__map_file::*;
use super::__year_to_secs::__year_to_secs;
use super::__month_to_secs::__month_to_secs;
use super::*;
use crate::env::get_env::*;
use crate::include::libc;
use crate::mman::munmap::*;
use core::ptr;

unsafe fn malloc(size: size_t) -> *mut c_void
{
    crate::malloc::lite_malloc::__simple_malloc(size)
}

static mut __timezone: c_long = 0;
static mut __daylight: c_int = 0;
static mut __tzname: [*mut c_char; 2] = [ptr::null_mut(); 2];

static mut std_name: [c_char; TZNAME_MAX+1] = [0; TZNAME_MAX+1];
static mut dst_name: [c_char; TZNAME_MAX+1] = [0; TZNAME_MAX+1];

static mut dst_off: c_int = 0;
static mut r0: [c_int; 5] = [0; 5];
static mut r1: [c_int; 5] = [0; 5];

static mut zi: *const c_uchar = ptr::null();
static mut trans: *const c_uchar = ptr::null();
static mut index: *const c_uchar = ptr::null();
static mut types: *const c_uchar = ptr::null();
static mut abbrevs: *const c_uchar = ptr::null();
static mut abbrevs_end: *const c_uchar = ptr::null();
static mut map_size: size_t = 0;

static mut old_tz_buf: [c_char; 32] = [0; 32];
static mut old_tz: *mut c_char = ptr::addr_of_mut!(old_tz_buf) as *mut c_char;
static mut old_tz_size: size_t = 32;

static mut lock: [c_int; 1] = [0];    // volatile
const __timezone_lockptr: *mut c_int = unsafe { ptr::addr_of_mut!(lock[0]) as *mut c_int };

unsafe fn getin(p: &mut *const c_char) -> c_int
{
    let mut x: c_uint = 0;
    while (**p as c_uchar).wrapping_sub(b'0') < 10 {
        x = x*10 + (**p as c_uchar).wrapping_sub(b'0') as c_uint;
        *p = (*p).add(1);
    }
    x as c_int
}

#[no_mangle]
unsafe fn getoff(p: &mut *const c_char) -> c_int
{
    let mut neg: bool = false;
    if **p == b'-' as c_char {
        *p = (*p).add(1);
        neg = true;
    } else if **p == b'+' as c_char {
        *p = (*p).add(1);
    }
    let mut off = 3600*getin(p);
    if **p == b':' as c_char {
        *p = (*p).add(1);
        off += 60*getin(p);
        if **p == b':' as c_char {
            *p = (*p).add(1);
            off += getin(p);
        }
    }
    if neg {-off} else {off}
}

unsafe fn getrule(p: &mut *const c_char, rule: &mut [c_int; 5])
{
    rule[0] = **p as c_int;
    let r = **p as c_int;

    if r != b'M' as c_int {
        if r==b'J' as c_int { *p = (*p).add(1); }
        else { rule[0] = 0; }
        rule[1] = getin(p);
    } else {
        *p = (*p).add(1); rule[1] = getin(p);
        *p = (*p).add(2); rule[1] = getin(p);
        *p = (*p).add(3); rule[1] = getin(p);
    }

    if **p == b'/' as c_char {
        *p = (*p).add(1);
        rule[4] = getoff(p);
    } else {
        rule[4] = 7200;
    }

}

unsafe fn getname(d: *mut c_char, p: *mut *const c_char)
{
    let mut i: size_t = 0;

    if **p == b'<' as c_char{
        *p = (*p).add(1);
        while (*p).add(i).read()!=0 && (*p).add(i).read()!=b'>' as c_char {
            if i < TZNAME_MAX { d.add(i).write((*p).add(i).read()); }
            i += 1;
        }
        if (*p).add(i).read() == b'>' as c_char {
            *p = (*p).add(1);
        }
    } else {
        while ((((*p).add(i).read()|32).wrapping_sub(b'a' as c_char)) as c_uint) < 26 {
            if i < TZNAME_MAX { d.add(i).write((*p).add(i).read()); }
            i += 1;
        }
    }
    *p = (*p).add(i);
    d.add(if i < TZNAME_MAX { i } else { TZNAME_MAX }).write(0);
}

unsafe fn zi_read32(z: *const c_uchar) -> uint32_t
{
    (*z as uint32_t) << 24 |
    (*z.add(1) as uint32_t) << 16 |
    (*z.add(2) as uint32_t) << 8 |
    (*z.add(3) as uint32_t)
}

unsafe fn zi_dotprod(z: *const c_uchar, v: *const c_uchar, n: size_t) -> size_t
{
    let mut y: size_t = 0;
    let mut z = z;
    let mut v = v;

    for _ in 0..n {
        let x = zi_read32(z);
        y += (x as size_t) * (*v as size_t);
        z = z.add(4);
        v = v.add(1);
    }

    y
}

unsafe fn do_tzset()
{
    let buf: [c_char; NAME_MAX+25] = [0; NAME_MAX+25];
    let path_name: *mut c_char = buf.as_ptr().add(24) as *mut c_char;
    let mut _try: *const c_char = ptr::null_mut();
    let mut s: *const c_char;
    let mut p: *const c_char;
    let mut map: *const c_uchar = ptr::null();
    let mut i: size_t;
    let search: *const c_char =
        b"/usr/share/zoneinfo/\0/share/zoneinfo\0/etc/zoneinfo/\0\0" as *const u8 as *const c_char;

    s = getenv(b"TZ".as_ptr() as *const c_char);
    if s.is_null() {
        s = b"/etc/localtime\0" as *const u8 as *const c_char;
    }
    if *s==0 { s = __utc.as_ptr();}

    if !old_tz.is_null() && strcmp(s, old_tz)==0 { return; }

    for i in 0..5 {
        r0[i] = 0;
        r1[i] = 0;
    }

    if !zi.is_null() {
        __munmap(zi as *mut c_void, map_size);
    }

    i = strlen(s);
    if i > PATH_MAX+1 { s = __utc.as_ptr(); i = 3; }
    if i > old_tz_size {
        old_tz_size *= 2;
        if i >= old_tz_size {
            old_tz_size = i + 1;
        }
        if old_tz_size > PATH_MAX+2 {
            old_tz_size = PATH_MAX+2;
        }
        old_tz = malloc(old_tz_size) as *mut c_char;
    }
    if !old_tz.is_null() {
        memcpy(old_tz as *mut c_void, s as *const c_void, i+1);
    }

    let mut posix_form: bool = false;
    if *s!=':' as c_char {
        p = s;
        let mut dummy_name: [c_char; TZNAME_MAX+1] = [0; TZNAME_MAX+1];
        getname(dummy_name.as_mut_ptr(), &mut p);
        if p!=s && (*p == b'+' as c_char || *p == b'-' as c_char || isdigit(*p as c_int)
                    || strcmp(dummy_name.as_mut_ptr(), b"GMT\0" as *const u8 as *const c_char) == 0
                    || strcmp(dummy_name.as_mut_ptr(), b"UTC\0" as *const u8 as *const c_char) == 0) {
            posix_form = true;
        }
    }

    if !posix_form {
        if *s==b':' as c_char { s = s.add(1); }
        if *s==b'/' as c_char || *s==b'.' as c_char {
            if libc::libc.secure==0 || strcmp(s, b"/etc/localtime\0".as_ptr() as *const c_char) == 0 {
                map = __map_file(s, &mut *ptr::addr_of_mut!(map_size));
            }
        } else {
            let mut l: size_t = strlen(s);
            if l <= NAME_MAX && strchr(s, b'.' as c_int).is_null() {
                memcpy(path_name as *mut c_void, s as *const c_void, l+1);
                path_name.add(l).write(0);
                _try = search;
                while map.is_null() && *_try!=0 {
                    l = strlen(_try);
                    memcpy(path_name.sub(l) as *mut c_void, _try as *const c_void, l);
                    map = __map_file(path_name.sub(l), &mut *ptr::addr_of_mut!(map_size));
                    _try = _try.add(l+1);
                }
            }
        }
        if map.is_null() { s = __utc.as_ptr(); }
    }
    if !map.is_null() &&
        (map_size < 44 || memcmp(map as *const c_void, b"TZif".as_ptr() as *const c_void, 4)!=0) {
        __munmap(map as *mut c_void, map_size);
        map = ptr::null();
        s = __utc.as_ptr();
    }

    zi = map;
    if !map.is_null() {
        let mut scale = 2;
        if map.add(4).read() != b'1' {
            let v: [c_uchar; 6] = [1, 1, 8, 5, 6, 1];
            let skip = zi_dotprod(zi.add(20), v.as_ptr(), 6);
            trans = zi.add(skip+44+44);
            scale += 1; 
        } else {
            trans = zi.add(44);
        }
        index = trans.add((zi_read32(trans.sub(12)) << scale) as size_t);
        types = index.add((zi_read32(trans.sub(12))) as size_t);
        abbrevs = types.add(6*(zi_read32(trans.sub(8))) as size_t);
        abbrevs_end = abbrevs.add((zi_read32(trans.sub(4))) as size_t);

        if zi.add(map_size-1).read() == b'\n' {
            s = (zi as *const c_char).add(map_size-2);
            while *s != b'\n' as c_char {s = s.sub(1); }
            s = s.add(1);
        } else {
            let mut p: *const c_uchar;
            __tzname[0] = ptr::null_mut();
            __tzname[1] = ptr::null_mut();
            __daylight = 0;
            __timezone = 0;
            dst_off = 0;

            p = types;
            while p < abbrevs {
                if p.add(4).read()==0 && __tzname[0].is_null() {
                    __tzname[0] = (abbrevs as *mut c_char).add(p.add(5).read() as size_t);
                    __timezone = -(zi_read32(p) as c_long);
                }
                if p.add(4).read()!=0 && __tzname[1].is_null() {
                    __tzname[1] = (abbrevs as *mut c_char).add(p.add(5).read() as size_t);
                    __daylight = 1;
                    dst_off = -(zi_read32(p) as c_int);
                }
                p = p.add(6);
            }
            if __tzname[0].is_null() { __tzname[0] = __tzname[1]; }
            if __tzname[0].is_null() { __tzname[0] = __utc.as_ptr() as *mut c_char; }
            if __daylight==0 {
                __tzname[1] = __tzname[0];
                dst_off = __timezone as c_int;
            }
            return
        }
    }

    if s.is_null() { s = __utc.as_ptr(); }
    getname(ptr::addr_of_mut!(std_name) as *mut c_char, &mut s);
    __tzname[0] = ptr::addr_of_mut!(std_name) as *mut c_char;
    __timezone = getoff(&mut s) as c_long;
    getname(ptr::addr_of_mut!(dst_name) as *mut c_char , &mut s);
    __tzname[1] = ptr::addr_of_mut!(dst_name) as *mut c_char;
    if dst_name[0] != 0 {
        __daylight = 1;
        if *s == b'+' as c_char || *s == b'-' as c_char || isdigit(*s as c_int) {
            dst_off = getoff(&mut s);
        } else {
            dst_off = (__timezone - 3600) as c_int;
        }
    } else {
        __daylight = 0;
        dst_off = __timezone as c_int;
    }

    if *s == b',' as c_char { s = s.add(1); getrule(&mut s, &mut *ptr::addr_of_mut!(r0));}
    if *s == b',' as c_char { s = s.add(1); getrule(&mut s, &mut *ptr::addr_of_mut!(r1));}

}

unsafe fn scan_trans(t: c_longlong, local: c_int, alt: *mut size_t) -> size_t
{
    let scale = 3 - if trans==zi.add(44) { 1 } else { 0 };
    let mut x: uint64_t;
    let mut off: c_int = 0;

    let mut a: size_t = 0;
    let mut n: size_t = (index.offset_from(trans) >> scale) as size_t;
    let mut m: size_t;

    if n == 0 {
        if !alt.is_null() { *alt = 0; }
        return 0;
    }

    while n > 1 {
        m = a + n/2;
        x = zi_read32(trans.add(m<<scale) as *const c_uchar) as uint64_t;
        if scale == 3 {
            x = x<<32 | zi_read32(trans.add((m<<scale)+4) as *const c_uchar) as uint64_t;
        } else {
            x = x as uint32_t as uint64_t;
        }

        if local != 0 {
            off = zi_read32(types.add(6*index.add(m-1).read() as size_t) as *const c_uchar) as c_int;
        }
        if (t-off as c_longlong) < x as int64_t {
            n /= 2;
        } else {
            a = m;
            n -= n/2;
        }
    }

    n = (index.offset_from(trans) >> scale) as size_t;
    if a == n-1 { return size_t::MAX; }
    if a == 0 {
        x = zi_read32(trans as *const c_uchar) as uint64_t;
        if scale == 3 {
            x = x<<32 | zi_read32(trans.add(4) as *const c_uchar) as uint64_t;
        } else {
            x = x as uint32_t as uint64_t;
        }
        let mut j: size_t = 0;
        let mut i = abbrevs.offset_from(types) as size_t;
        while i!=0 {
            if types.offset((i-6+4) as isize).read() == 0 {
                j = (i-6) as size_t;
            }
            i -= 6;
        }
        if local != 0 {
            off = zi_read32(types.add(j) as *const c_uchar) as c_int;
        }
        if (t-off as c_longlong) < x as int64_t {
            if !alt.is_null() { *alt = index.add(0).read() as size_t; }
            return j/6;
        }
    }

    if !alt.is_null() {
        if a!=0 && types.offset((6*index.add(a-1).read() +4) as isize).read()
            != types.offset((6*index.add(a).read() +4) as isize).read() {
            *alt = index.add(a-1).read() as size_t;
        } else if a+1<n && types.offset((6*index.add(a+1).read() +4) as isize).read()
            != types.offset((6*index.add(a).read() +4) as isize).read() {
            *alt = index.add(a+1).read() as size_t;
        } else {
            *alt = index.add(a).read() as size_t;
        }
    }
    
    index.add(a).read() as size_t
}

fn days_in_month(m: c_int, is_leap: bool) -> c_int
{
    if m== 2 { 28+ if is_leap { 1 } else { 0 } }
    else { 30+((0xad5>>(m-1))&1)}
}

unsafe fn rule_to_secs(rule: *const c_int, year: c_int) -> c_longlong
{
    let mut is_leap: bool = false;
    let mut t: c_longlong = __year_to_secs(year as c_longlong, &mut is_leap);
    let mut x: c_int;
    let m: c_int;
    let mut n: c_int;
    let d: c_int;

    if rule.add(0).read() != b'M' as c_int {
        x = rule.add(1).read();
        if rule.add(0).read() == b'J' as c_int && (x < 60 || !is_leap) { x -= 1;}
        t += 86400 * (x as c_longlong);
    } else {
        m = rule.add(1).read();
        n = rule.add(2).read();
        d = rule.add(3).read();
        t += __month_to_secs(m-1, is_leap) as c_longlong;
        let wday = ((t + 4*86400) % (7*86400)) as c_int / 86400;
        let mut days = d - wday;
        if days < 0 { days += 7; }
        if n==5 && days+28 >= days_in_month(m, is_leap) { n = 4; }
        t += 86400 * (days + 7*(n-1)) as c_longlong;
    }
    t += rule.add(4).read() as c_longlong;
    t
}

#[no_mangle]
pub unsafe extern "C" fn __secs_to_zone(
    t: c_longlong,
    local: c_int,
    isdst: *mut c_int,
    offset: *mut c_long,
    oppoff: *mut c_long,
    zonename: *mut *const c_char,
) {
    LOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
    do_tzset();

    if !zi.is_null() {
        let mut alt: size_t = 0;
        let i = scan_trans(t, local, &mut alt);
        if i != size_t::MAX {
            *isdst = types.add(6*i+4).read() as c_int;
            *offset = (zi_read32(types.add(6*i) as *const c_uchar) as c_longlong) as int32_t as c_long;
            *zonename = (abbrevs as *const c_char).add(
                types.add(6*i+5).read() as size_t
            );
            if !oppoff.is_null() {
                *oppoff = zi_read32(types.add(6*alt) as *const c_uchar) as int32_t as c_long;
            }
            UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
            return;
        }
    }

    if __daylight==0 { __std(isdst, offset, zonename, oppoff); return }

    let mut y: c_longlong = t / 31556952 + 70;
    let mut false_flag: bool = false;
    while __year_to_secs(y, &mut false_flag) > t { y -= 1; }
    false_flag = false;
    while __year_to_secs(y+1, &mut false_flag) <= t { y += 1; }

    let mut t0: c_longlong = rule_to_secs(ptr::addr_of_mut!(r0) as *const c_int, y as c_int);
    let mut t1: c_longlong = rule_to_secs(ptr::addr_of_mut!(r1) as *const c_int, y as c_int);

    if local==0 {
        t0 += __timezone;
        t1 += dst_off as c_longlong;
    }
    if t0 < t1 {
        if t >= t0 && t < t1 { __dst(isdst, offset, zonename, oppoff); return }
        __std(isdst, offset, zonename, oppoff); return
    } else {
        if t >= t1 && t < t0 { __std(isdst, offset, zonename, oppoff); return }
        __dst(isdst, offset, zonename, oppoff); return
    }
}

unsafe fn __std(isdst: *mut c_int, offset: *mut c_long, zonename: *mut *const c_char, oppoff: *mut c_long)
{
    *isdst = 0;
    *offset = -__timezone as c_long;
    if !oppoff.is_null() { *oppoff = -dst_off as c_long; }
    *zonename = __tzname[0];
    UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
}

unsafe fn __dst(isdst: *mut c_int, offset: *mut c_long, zonename: *mut *const c_char, oppoff: *mut c_long)
{
    *isdst = 1;
    *offset = -dst_off as c_long;
    if !oppoff.is_null() { *oppoff = -__timezone as c_long; }
    *zonename = __tzname[1];
    UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
}

#[no_mangle]
pub unsafe fn __tm_to_tzname(tm: &tm) -> *const c_char
{
    let mut p = tm.__tm_zone as *const c_void;
    LOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
    do_tzset();
    if p != __utc.as_ptr() as *const c_void
     && p != __tzname[0] as *const c_void
     && p != __tzname[1] as *const c_void
     && (zi.is_null() || (p as uintptr_t) - (abbrevs as uintptr_t) >= abbrevs_end.offset_from(abbrevs) as uintptr_t) {
        p = b"\0" as *const u8 as *const c_void;
    }
    UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
    p as *const c_char
}