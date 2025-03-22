use crate::include::ctype::*;
use crate::arch::atomic_arch::*;

#[no_mangle]
pub extern "C" fn pthread_spin_unlock(s: *mut pthread_spinlock_t) -> c_int
{
    a_store(s, 0);
    0
}