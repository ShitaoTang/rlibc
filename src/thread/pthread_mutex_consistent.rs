use core::ptr;

use crate::{arch::atomic_arch::a_and, include::ctype::*};
use super::pthread_self::*;
use crate::arch::generic::bits::errno::*;

#[no_mangle]
pub unsafe extern "C" fn pthread_mutex_consistent(m: *mut pthread_mutex_t) -> c_int
{
    let old = (*m)._m_lock();
    let own = old & 0x3fffffff;
    if ((*m)._m_type()&4)==0 || own==0 || (old&0x40000000)==0 {
        return EINVAL;
    }
    if own != (*pthread_self()).tid {
        return EPERM;
    }
    a_and(ptr::addr_of_mut!((*m).__u.__vi[1]), !0x40000000);

    0
}