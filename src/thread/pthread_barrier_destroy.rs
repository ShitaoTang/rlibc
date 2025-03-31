use core::ptr;

use crate::arch::atomic_arch::a_or;
use crate::include::ctype::*;
use crate::arch::generic::bits::errno::EINVAL;

use super::__wait::wait;
use super::vmlock::vm_wait;

#[no_mangle]
pub unsafe extern "C" fn pthread_barrier_destroy(b: *mut pthread_barrier_t) -> c_int
{
    if b.is_null() { return EINVAL; }

    if (*b)._b_limit() < 0 {
        if (*b)._b_lock() != 0 {
            let mut v: c_int;
            a_or(ptr::addr_of_mut!((*b).__u.__vi[0]), c_int::MIN);
            v = (*b)._b_lock();
            while (v & c_int::MAX) != 0 {
                wait(ptr::addr_of_mut!((*b).__u.__vi[0]), ptr::null_mut(), v, 0);
                v = (*b)._b_lock();
            }
        }
        vm_wait();
    }

    0
}