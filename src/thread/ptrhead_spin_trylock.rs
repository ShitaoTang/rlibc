use crate::include::ctype::*;
use crate::arch::atomic_arch::*;

#[no_mangle]
pub extern "C" fn pthread_spin_trylock(s: *mut pthread_spinlock_t) -> c_int
{
    a_cas(s, 0, libc::EBUSY);
    0
}