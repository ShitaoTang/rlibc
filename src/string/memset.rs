use crate::include::ctype::*;

/* 
 * ⚠️ This code may lead to **Undefined Behavior** when compiled with 
 * opt-level >= 1, thouth may be OK in debug mode. It corrupts the stack by illegally 
 * writing to memory regions corresponding to other local variables. 
 * As a result, the compiler is allowed to assume invariants that no longer hold,
 * potentially leading to incorrect code generation or crashes.
 */
// #[no_mangle]
// pub unsafe fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void
// {
//     let mut s: *mut c_uchar = dest as *mut c_uchar;
//     let mut k: size_t;
//     let mut n = n;
// 
//     if n == 0 { return dest; }
//     *s = c as c_uchar;
//     *s.offset((n-1) as isize) = c as c_uchar;
//     if n <= 2 { return dest; }
//     *s.offset(1) = c as c_uchar;
//     *s.offset(2) = c as c_uchar;
//     *s.offset((n-2) as isize) = c as c_uchar;
//     *s.offset((n-3) as isize) = c as c_uchar;
//     if n <= 6 { return dest; }
//     *s.offset(3) = c as c_uchar;
//     *s.offset((n-4) as isize) = c as c_uchar;
//     if n <= 8 { return dest; }
// 
//     k = (s as uintptr_t).wrapping_neg() & 3;
//     s = s.offset(k as isize);
//     n = n.wrapping_sub(k);
//     n &= (4 as uintptr_t).wrapping_neg();
// 
//     let c32: u32 = (u32::MAX)/255 * (c as c_uchar) as u32;
//     
//     *(s as *mut u32) = c32;
//     *(s.offset((n+4) as isize) as *mut u32) = c32;
//     if n <= 8 { return dest; }
//     *(s.offset(4) as *mut u32) = c32;
//     *(s.offset((n+8) as isize) as *mut u32) = c32;
//     *(s.offset((n-12) as isize) as *mut u32) = c32;
//     *(s.offset((n-8) as isize) as *mut u32) = c32;
//     if n <= 24 { return dest; }
//     *(s.offset(12) as *mut u32) = c32;
//     *(s.offset(16) as *mut u32) = c32;
//     *(s.offset(20) as *mut u32) = c32;
//     *(s.offset(24) as *mut u32) = c32;
//     *(s.offset((n-28) as isize) as *mut u32) = c32;
//     *(s.offset((n-24) as isize) as *mut u32) = c32;
//     *(s.offset((n-20) as isize) as *mut u32) = c32;
//     *(s.offset((n-16) as isize) as *mut u32) = c32;
// 
//     k = 24 + ((s as uintptr_t) & 4);
//     s = s.offset(k as isize);
//     n = n.wrapping_sub(k);
// 
//     let c64: u64 = c32 as u64 | ((c32 as u64) << 32);
//     while n >= 32 {
//         *(s as *mut u64) = c64;
//         *(s.offset(8) as *mut u64) = c64;
//         *(s.offset(16) as *mut u64) = c64;
//         *(s.offset(24) as *mut u64) = c64;
//         s = s.offset(32);
//         n = n.wrapping_sub(32);
//     }
// 
//     dest as *mut c_void
// }

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void
{
    let s = dest as *mut c_uchar;
    let c = c as c_uchar;
    
    for i in 0..n {
        *s.offset(i as isize) = c;
    }
    
    dest
}