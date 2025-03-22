use crate::include::ctype::*;
use core::ptr;
use crate::arch::atomic_arch::*;

#[no_mangle]
pub extern "C" fn pthread_spin_lock(s: *mut pthread_spinlock_t) -> c_int
{
    unsafe {
        while ptr::read_volatile(s) != 0 || a_cas(s, 0, libc::EBUSY) != 0 {
            a_barrier();
        }
    }
    0
}