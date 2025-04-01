use core::ptr;

use crate::arch::atomic_arch::a_cas;
use crate::include::ctype::*;
use super::pthread_impl::DT_STATUS;
use super::pthread_setcancelstate::*;
use super::pthread_join::*;
use super::PTHREAD_CANCEL_DISABLE;

#[no_mangle]
pub unsafe extern "C" fn pthread_detach(t: pthread_t) -> c_int
{
    if a_cas(ptr::addr_of_mut!((*t).detach_state),
        DT_STATUS::DT_JOINABLE as c_int,
        DT_STATUS::DT_DETACHED as c_int) != DT_STATUS::DT_JOINABLE as c_int {
        let mut cs: c_int = 0;
        pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, ptr::addr_of_mut!(cs));
        pthread_join(t, ptr::null_mut());
        pthread_setcancelstate(cs, ptr::null_mut());    
    }

    0
}