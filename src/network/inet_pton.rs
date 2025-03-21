use libc::{c_int, c_uchar, c_char, c_void};
use crate::thread::pthread::pthread_self;
use super::*;

fn isdigit(c: u8) -> bool
{
    c >= b'0' && c <= b'9'
}

fn hexval(c: u8) -> c_int
{
    // this is buggy
    // let mut c = c;
    // if (c - b'0') < 10 { return (c - b'0') as c_int; }
    // c |= 32;
    // if (c - b'a') < 6 { return (c - b'a' + 10) as c_int; }
    if isdigit(c) { return (c - b'0') as c_int; }
    if c >= b'a' && c <= b'f' { return (c - b'a' + 10) as c_int; }
    if c >= b'A' && c <= b'F' { return (c - b'A' + 10) as c_int; }
    -1
}

// Presentation format -> Network byte order
// human-readable IP addr -> binary IP addr (IPv4/IPv6)
// (IPv4): "192.168.1.1" -> 0xC0A80101
// (IPv6): "2001:db8::1" -> [0x20, 0x01, 0x0d, 0xb8, ...]
#[no_mangle]
pub extern "C" fn inet_pton(af: c_int, s: *const c_char, a0: *mut c_void) -> c_int
{
    let mut ip: [uint16_t; 8] = [0; 8];
    let mut a: *mut c_uchar = a0 as *mut c_uchar;
    let mut i: c_int = 0;
    let mut j: c_int;
    let mut v: c_int;
    let mut brk: c_int = -1;
    let mut need_v4: c_int = 0;
    let mut s = s;
    let mut _self = pthread_self();

    unsafe {
    if af == AF_INET {
        while i < 4 {
            v = 0; j = 0;
            while j<3 && isdigit(*s.offset(j as isize)) {
                v = v*10 + (*s.offset(j as isize) - b'0') as c_int;
                j += 1;
            }
            if j==0 || (j>1 && *s.offset(0) == b'0') || v > 255 { return 0; }
            *a.offset(i as isize) = v as c_uchar;
            if *s.offset(j as isize)==0 && i==3 { return 1; }
            if *s.offset(j as isize) != b'.' { return 0; }
            s = s.offset(j as isize + 1);
            i += 1;
        }
        return 0;
    } else if af != AF_INET6 {
        (*_self).errno_val = libc::EAFNOSUPPORT;
        return -1;
    }

    if *s==b':' && {s=s.offset(1); *s!=b':'} { return 0; }

    i = 0;
    loop {
        if *s==b':' && brk<0 {
            brk = i;
            ip[(i&7) as usize] = 0;
            s = s.offset(1);
            if *s==0 { break; }
            if i==7 { return 0; }
            i += 1;
            continue;
        }
        v = 0; j = 0;
        while j < 4 {
            let d = hexval(*s.offset(j as isize));
            if d < 0 { break; }
            v = (v << 4) + d;
            j += 1;
        }
        if j == 0 { return 0; }
        ip[(i&7) as usize] = v as uint16_t;
        if *s.offset(j as isize)==0 && (brk>=0 || i==7) { break; }
        if i == 7 { return 0; }
        if *s.offset(j as isize) != b':' {
            if *s.offset(j as isize) != b'.' || (i<6 && brk<0) { return 0; }
            need_v4 = 1;
            i += 1;
            ip[(i&7) as usize] = 0;
            break;
        }
        s = s.offset(j as isize + 1);
        i += 1;
    }
    if brk >= 0 {
        libc::memmove(ip.as_mut_ptr().offset(brk as isize + 7 - i as isize) as *mut c_void,
                      ip.as_ptr().offset(brk as isize) as *const c_void,
                      (i - brk + 1) as usize * 2
        );
        for j in 0..(7-i) {
            ip[(brk+j) as usize] = 0;
        }
    }

    for j in 0..8 {
        *a = (ip[j] >> 8) as c_uchar;
        a = a.offset(1);
        *a = ip[j] as c_uchar;
        a = a.offset(1);
    }

    if need_v4!=0 && inet_pton(AF_INET, s as *const c_char, a.offset(-4) as *mut c_void)<=0 {
        return 0;
    }
    }
    1
}