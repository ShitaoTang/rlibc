use crate::include::ctype::*;
use crate::string::memcpy::memcpy;

// Word Size, 8 for 64-bit, 4 for 32-bit
const WS: size_t = core::mem::size_of::<size_t>();

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
{
    let mut d = dest as *mut c_char;
    let mut s = src as *const c_char;
    let mut n = n;

    if d as uintptr_t==s as uintptr_t { return d as *mut c_void; }
    // if memory areas do not overlap
    if (s as uintptr_t).wrapping_sub(d as uintptr_t).wrapping_sub(n) <= n.wrapping_neg().wrapping_mul(2) {
        return memcpy(dest, src, n) as *mut c_void;
    }

    if (d as uintptr_t) < (s as uintptr_t) {
        if s as uintptr_t % WS == d as uintptr_t % WS {
            while d as uintptr_t % WS != 0 {
                if n == 0 { return dest as *mut c_void; } n -= 1;
                *d = *s; d = d.add(1); s = s.add(1);
            }
            while n >= WS {
                *(d as *mut size_t) = *(s as *const size_t);
                d = d.add(WS); s = s.add(WS); n -= WS;
            }
        }
        while n!=0 { *d = *s; d = d.add(1); s = s.add(1); n -= 1; }
    } else {
        if s as uintptr_t % WS == d as uintptr_t % WS {
            while (d as uintptr_t).wrapping_add(n) % WS != 0 {
                if n == 0 { return dest as *mut c_void; } n -= 1;
                *d.add(n) = *s.add(n);
            }
            while n >= WS {
                n -= WS;
                *(d.add(n) as *mut size_t) = *(s.add(n) as *const size_t);
            }
        }
        while n != 0 { n -= 1; *d.add(n) = *s.add(n); }
    }

    dest as *mut c_void
}