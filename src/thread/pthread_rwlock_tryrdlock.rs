use crate::include::ctype::*;
use crate::arch::atomic_arch::*;
use core::ptr;
use crate::arch::generic::bits::errno::*;

#[no_mangle]
pub extern "C" fn pthread_rwlock_tryrdlock(rw: *mut pthread_rwlock_t) -> c_int
{
    let mut val: c_int;
    let mut cnt: c_int;
    loop {
        val = unsafe {(*rw)._rw_lock()};
        cnt = val & 0x7fffffff;
        if cnt == 0x7fffffff {return EBUSY;}
        if cnt == 0x7ffffffe {return EAGAIN;}
        if a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, val, val+1) == val {break;}
    }
    0
}