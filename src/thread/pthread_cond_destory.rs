use crate::include::ctype::*;
use core::ptr;
use crate::arch::atomic_arch::*;
use super::pthread_impl::wake;
use super::__wait::*;

#[no_mangle]
pub extern "C" fn pthread_cond_destory(c: *mut pthread_cond_t) -> c_int
{
    if unsafe{(*c)._c_shared()} != ptr::null_mut() && unsafe{(*c)._c_waiters()} != 0 {
        let mut cnt: c_int;
        a_or(unsafe{ptr::addr_of_mut!((*c).__u.__vi[3])}, c_int::MIN);
        a_inc(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])});
        wake(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])}, -1 as c_int, 0);
        cnt = unsafe{(*c)._c_waiters()};
        loop {
            if (cnt&0x7fffffff) == 0 {break;}
            wait(unsafe{ptr::addr_of_mut!((*c).__u.__vi[3])}, ptr::null_mut(), cnt, 0);
            cnt = unsafe{(*c)._c_waiters()};
        }
    }
    0
}