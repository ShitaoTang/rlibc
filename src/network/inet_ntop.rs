use crate::include::ctype::*;
use super::*;
use crate::thread::pthread_self::pthread_self;
use crate::arch::generic::bits::errno::*;

// Network byte order -> Presentation format
// binary IP addr (IPv4/IPv6) -> human-readable IP addr
// (IPv4): 0xC0A80101 -> "192.168.1.1"
// (IPv6): [0x20, 0x01, 0x0d, 0xb8, ...] -> "2001:db8::1"
#[no_mangle]
pub extern "C" fn inet_ntop(af: c_int, a0: *const c_void, s: *mut c_char, l: socklen_t) -> *const c_char
{
    let a: *const c_uchar = a0 as *const c_uchar;
    let mut i: usize = 0;
    let mut j: c_int;
    let mut max: usize = 2;
    let mut best: usize = 0;
    let mut _self = pthread_self();
    let mut buf: [c_char; 100] = [0; 100];

    unsafe {
    match af {
    AF_INET => {
        if libc::snprintf(s, l as usize, b"%d.%d.%d.%d\0".as_ptr() as *const c_char,
                            *a as c_uint, *(a.offset(1)) as c_uint, 
                            *(a.offset(2)) as c_uint, *(a.offset(3)) as c_uint
            ) < l.try_into().unwrap() {
            return s;
        }
    }
    AF_INET6 => {
        if libc::memcmp(a0, b"\0\0\0\0\0\0\0\0\0\0\xff\xff".as_ptr() as *const c_void, 12) != 0 {
            libc::snprintf(buf.as_mut_ptr() as *mut c_char, buf.len() as usize, 
                           b"%x:%x:%x:%x:%x:%x:%x:%x\0".as_ptr() as *const c_char,
                           (((*a.offset(0) as u16) << 8) + *a.offset(1) as u16) as c_int,
                           (((*a.offset(2) as u16) << 8) + *a.offset(3) as u16) as c_int,
                           (((*a.offset(4) as u16) << 8) + *a.offset(5) as u16) as c_int,
                           (((*a.offset(6) as u16) << 8) + *a.offset(7) as u16) as c_int,
                           (((*a.offset(8) as u16) << 8) + *a.offset(9) as u16) as c_int,
                           (((*a.offset(10) as u16) << 8) + *a.offset(11) as u16) as c_int,
                           (((*a.offset(12) as u16) << 8) + *a.offset(13) as u16) as c_int,
                           (((*a.offset(14) as u16) << 8) + *a.offset(15) as u16) as c_int
            );
        } else {
            libc::snprintf(buf.as_mut_ptr() as *mut c_char, buf.len() as usize, 
                           b"%x:%x:%x:%x:%x:%x:%d.%d.%d.%d\0".as_ptr() as *const c_char,
                           (((*a.offset(0) as u16) << 8) + *a.offset(1) as u16) as c_int,
                           (((*a.offset(2) as u16) << 8) + *a.offset(3) as u16) as c_int,
                           (((*a.offset(4) as u16) << 8) + *a.offset(5) as u16) as c_int,
                           (((*a.offset(6) as u16) << 8) + *a.offset(7) as u16) as c_int,
                           (((*a.offset(8) as u16) << 8) + *a.offset(9) as u16) as c_int,
                           (((*a.offset(10) as u16) << 8) + *a.offset(11) as u16) as c_int,
                            *a.offset(12) as c_int, *a.offset(13) as c_int,
                            *a.offset(14) as c_int, *a.offset(15) as c_int
            );
        }
        while buf[i] != 0 {
            if i!=0 && buf[i]!=b':' as c_char { i += 1; continue; }
            // the accept string should end with '\0', otherwise it will lead to UB --- reading uninitialized memory
            // for zero compressed IPv6 address
            j = libc::strspn(buf.as_ptr().add(i), b":0\0".as_ptr() as *const c_char) as c_int; 
            if j > max as c_int {
                best = i;
                max = j as usize;
            }
            i += 1;
        }
        if max > 3 {
            buf[best as usize] = b':'.try_into().unwrap();
            buf[(best+1) as usize] = b':'.try_into().unwrap();
            libc::memmove(buf.as_mut_ptr().add(best+2) as *mut c_void, 
                     buf.as_ptr().add(best+max) as *const c_void, (i-best-max+1) as usize);
        }
        if libc::strlen(buf.as_ptr()) < l.try_into().unwrap() {
            libc::strcpy(s, buf.as_ptr());
            return s;
        }
    }
    _ => {
        (*_self).errno_val = EAFNOSUPPORT;
        return 0 as *const c_char;
    }
    }}
    unsafe {(*_self).errno_val = ENOSPC};
    0 as *const c_char
}