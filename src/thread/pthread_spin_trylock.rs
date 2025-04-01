use crate::include::ctype::*;
use crate::arch::atomic_arch::*;
use crate::arch::generic::bits::errno::*;

#[no_mangle]
pub extern "C" fn pthread_spin_trylock(s: *mut pthread_spinlock_t) -> c_int
{
    a_cas(s, 0, EBUSY);
    0
}