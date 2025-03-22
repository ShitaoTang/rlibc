use core::ptr;
use crate::include::ctype::*;
use crate::arch::atomic_arch::*;
use crate::arch::generic::bits::errno::*;

#[no_mangle]
pub extern "C" fn pthread_rwlock_trywrlock(rw: *mut pthread_rwlock_t) -> c_int
{
    if a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, 0, 0x7fffffff) != 0 {return EBUSY;}
    0
}