use super::*;

#[no_mangle]
pub extern "C" fn pthread_spin_init(s: *mut pthread_spinlock_t, _pshared: c_int) -> c_int
{
    unsafe {*s = 0;}
    0
}