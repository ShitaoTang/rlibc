use crate::include::ctype::*;
use super::in_addr;
use crate::stdlib::strtol::strtoul;

// ASCII string -> Network byte order
// only for IPv4
// string in dotted-decimal notation -> 32-bit integer in network byte order
// "192.168.1.1" -> in_addr { s_addr: 0xC0A80101 }
// s0 -> dest
#[no_mangle]
pub extern "C" fn inet_aton(s0: *const c_char, dest: *mut in_addr) -> c_int
{
    let mut s: *const c_char = s0;
    let d: *mut c_uchar = dest as *mut c_uchar;
    let mut a: [c_ulong; 4] = [0; 4];
    let mut z: *mut c_char = 0 as *mut c_char;
    let mut i: c_int = 0;

    while i < 4 {
        a[i as usize] = strtoul(s, &mut z, 0);
        if z as *const c_uchar == s as *const c_uchar
         || unsafe{(*z)!=0 && (*z)!=b'.'.try_into().unwrap()}
         || unsafe{(*s as u8) as u32 - '0' as u32 >= 10} { 
            return 0;
        }
        if unsafe{(*z)==0} { break; }
        s = unsafe{z.offset(1)};
        i += 1;
    }
    if i==4 { return 0; }
    match i {
        0 => {
            a[1] = a[0] & 0xffffff;
            a[0] >>= 24;
            a[2] = a[1] & 0xffff;
            a[1] >>= 16;
            a[3] = a[2] & 0xff;
            a[2] >>= 8;
        }
        1 => {
            a[2] = a[1] & 0xffff;
            a[1] >>= 16;
            a[3] = a[2] & 0xff;
            a[2] >>= 8;
        }
        2 => {
            a[3] = a[2] & 0xff;
            a[2] >>= 8;
        }
        _ => {}
    }
    i = 0;
    while i < 4 {
        if a[i as usize] > 255 { return 0; }
        unsafe { *d.offset(i as isize) = a[i as usize] as c_uchar; }
        i += 1;
    }

    1
}