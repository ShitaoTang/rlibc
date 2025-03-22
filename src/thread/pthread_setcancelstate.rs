use crate::include::ctype::*;
use core::ptr;
use super::pthread_self::*;

#[no_mangle]
pub extern "C" fn pthread_setcancelstate(new: c_int, old: *mut c_int) -> c_int
{
    if new as c_uint > 2u32 {return libc::EINVAL;}   // trick, only when 0<=new<=2, it's valid (negatives are invalid)
    let mut _self: pthread_t = pthread_self();
    // unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    if old != ptr::null_mut() { unsafe  {*old = ptr::read_volatile(ptr::addr_of_mut!((*_self).canceldisable)) as c_int};}
    unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).canceldisable), new as c_uchar)};
    0
}