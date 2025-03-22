use crate::include::ctype::*;
use super::pthread_cond_timedwait::private_cond_signal;
use super::pthread_impl::wake;
use core::ptr;
use crate::arch::atomic_arch::*;

#[no_mangle]
pub extern "C" fn pthread_cond_signal(c: *mut pthread_cond_t) -> c_int
{
    if unsafe{(*c)._c_shared()} == ptr::null_mut() {
        return private_cond_signal(c, 1);
    }
    if unsafe{(*c)._c_waiters()} == 0 {return 0;}
    a_inc(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])});
    wake(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])}, 1, 0);

    0
}