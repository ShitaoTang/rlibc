use crate::include::ctype::*;

use super::vmlock::vm_wait;

#[no_mangle]
pub unsafe extern "C" fn pthread_mutex_destroy(m: *mut pthread_mutex_t) -> c_int
{
    if (*m)._m_type() > 128 { vm_wait(); }
    0
}