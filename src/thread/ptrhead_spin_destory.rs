use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_spin_destroy(_s: *mut pthread_spinlock_t) -> c_int
{
    0
}
