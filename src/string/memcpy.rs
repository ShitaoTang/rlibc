use crate::include::ctype::*;
use core::ptr;

// // this is 4x~5x slower than in musl
// #[no_mangle]
// pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
// {
//     let d = dest as *mut c_uchar;
//     let s = src as *const c_uchar;

//     unsafe {
//         ptr::copy_nonoverlapping(s, d, n);
//     }

//     dest
// }

// this can be 4x~5x slower than this in musl
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let mut d = dest as *mut c_uchar;
    let mut s = src as *const c_uchar;
    let mut n = n;

    // alignment
    while (s as usize) % 4 != 0 && n != 0 {
        *d = *s; d = d.add(1); s = s.add(1);
    }

    // copy on block of 4 bytes
    if (d as uintptr_t) % 4 == 0 {
        while n >= 16 {
            *(d as *mut u32) = *(s as *const u32);
            *(d.add(4) as *mut u32) = *(s.add(4) as *const u32);
            *(d.add(8) as *mut u32) = *(s.add(8) as *const u32);
            *(d.add(12) as *mut u32) = *(s.add(12) as *const u32);
            s = s.add(16); d = d.add(16); n -= 16;
        }
        if n&8 != 0 {
            *(d as *mut u32) = *(s as *const u32);
            *(d.add(4) as *mut u32) = *(s.add(4) as *const u32);
            s = s.add(8); d = d.add(8);
        }
        if n&4 != 0 {
            *(d as *mut u32) = *(s as *const u32);
            s = s.add(4); d = d.add(4);
        }
        if n&2 != 0 {
            *d = *s; s = s.add(1); d = d.add(1);
            *d = *s; s = s.add(1); d = d.add(1);
        }
        if n&1 != 0 {
            *d = *s;
        }
        // *d = 0; // end with '\0'
        return dest;
    }

    if n >= 32 { match (d as uintptr_t) % 4 {
        1 => {
            let mut w = ptr::read(s as *const u32);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            n -= 3;
            while n >= 17 {
                #[cfg(target_endian = "little")]
                {
                    let x = ptr::read(s.add(1) as *const u32);
                    *(d as *mut u32) = (w >> 24) | (x << 8);
                    w = ptr::read(s.add(5) as *const u32);
                    *(d.add(4) as *mut u32) = (x >> 24) | (w << 8);
                    let x = ptr::read(s.add(9) as *const u32);
                    *(d.add(8) as *mut u32) = (w >> 24) | (x << 8);
                    w = ptr::read(s.add(13) as *const u32);
                    *(d.add(12) as *mut u32) = (x >> 24) | (w << 8);
                }
                #[cfg(target_endian = "big")]
                {
                    let x = ptr::read(s.add(1) as *const u32);
                    *(d as *mut u32) = (w << 24) | (x >> 8);
                    w = ptr::read(s.add(5) as *const u32);
                    *(d.add(4) as *mut u32) = (x << 24) | (w >> 8);
                    let x = ptr::read(s.add(9) as *const u32);
                    *(d.add(8) as *mut u32) = (w << 24) | (x >> 8);
                    w = ptr::read(s.add(13) as *const u32);
                    *(d.add(12) as *mut u32) = (x << 24) | (w >> 8);
                }
                s = s.add(16); d = d.add(16); n -= 16;
            }
        }
        2 => {
            let mut w = ptr::read(s as *const u32);
            *d = *s; d = d.add(1); s = s.add(1);
            *d = *s; d = d.add(1); s = s.add(1);
            n -= 2;
            while n >= 18 {
                #[cfg(target_endian = "little")]
                {
                    let x = ptr::read(s.add(2) as *const u32);
                    *(d as *mut u32) = (w >> 16) | (x << 16);
                    w = ptr::read(s.add(6) as *const u32);
                    *(d.add(4) as *mut u32) = (x >> 16) | (w << 16);
                    let x = ptr::read(s.add(10) as *const u32);
                    *(d.add(8) as *mut u32) = (w >> 16) | (x << 16);
                    w = ptr::read(s.add(14) as *const u32);
                    *(d.add(12) as *mut u32) = (x >> 16) | (w << 16);
                }
                #[cfg(target_endian = "big")]
                {
                    *(d as *mut u32) = (w << 16) | (x >> 16);
                    w = ptr::read(s.add(6) as *const u32);
                    *(d.add(4) as *mut u32) = (x << 16) | (w >> 16);
                    let x = ptr::read(s.add(10) as *const u32);
                    *(d.add(8) as *mut u32) = (w << 16) | (x >> 16);
                    w = ptr::read(s.add(14) as *const u32);
                    *(d.add(12) as *mut u32) = (x << 16) | (w >> 16);
                }
                s = s.add(16); d = d.add(16); n -= 16;
            }
        }
        3 => {
            let mut w = ptr::read(s as *const u32);
            *d = *s; d = d.add(1); s = s.add(1);
            n -= 1;
            while n >= 19 {
                #[cfg(target_endian = "little")]
                {
                    let x = ptr::read(s.add(3) as *const u32);
                    *(d as *mut u32) = (w >> 8) | (x << 24);
                    w = ptr::read(s.add(7) as *const u32);
                    *(d.add(4) as *mut u32) = (x >> 8) | (w << 24);
                    let x = ptr::read(s.add(11) as *const u32);
                    *(d.add(8) as *mut u32) = (w >> 8) | (x << 24);
                    w = ptr::read(s.add(15) as *const u32);
                    *(d.add(12) as *mut u32) = (x >> 8) | (w << 24);
                }
                #[cfg(target_endian = "big")]
                {
                    let x = ptr::read(s.add(3) as *const u32);
                    *(d as *mut u32) = (w << 24) | (x >> 8);
                    w = ptr::read(s.add(7) as *const u32);
                    *(d.add(4) as *mut u32) = (x << 24) | (w >> 8);
                    let x = ptr::read(s.add(11) as *const u32);
                    *(d.add(8) as *mut u32) = (w << 24) | (x >> 8);
                    w = ptr::read(s.add(15) as *const u32);
                    *(d.add(12) as *mut u32) = (x << 24) | (w >> 8);
                }
                s = s.add(16); d = d.add(16); n -= 16;
            }
        }
        _ => {}
    }}

    if n&16 != 0 {  // 16x: *d++ = *s++
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
    }
    if n&8 != 0 {
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
   }
    if n&4 != 0 {
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
    }
    if n&2 != 0 {
        *d = *s; d = d.add(1); s = s.add(1); *d = *s; d = d.add(1); s = s.add(1);
    }
    if n&1 != 0 {
        *d = *s;
    }
    // *d = 0; // end with '\0'
    dest
}

