use crate::arch::atomic_arch::a_crash;
use crate::arch::generic::bits::errno::*;
use crate::include::ctype::*;
use crate::mman::munmap::*;
use super::__timedwait::timedwait_cp;
use super::pthread_impl::DT_STATUS;
use super::pthread_testcancel::*;
use super::pthread_setcancelstate::*;
use super::pthread_create::*;
use super::*;
use crate::include::time::*;

#[no_mangle]
pub unsafe fn pthread_timedjoin_np(t: pthread_t, res: *mut *mut c_void, at: *const timespec) -> c_int
{
    let mut state: c_int;
    let mut cs: c_int = 0;
    let mut r: c_int = 0;

    pthread_testcancel();
    pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, ptr::addr_of_mut!(cs));
    if cs==PTHREAD_CANCEL_ENABLE {
        pthread_setcancelstate(cs, ptr::null_mut());
    }
    state = (*t).detach_state;
    while state!=0 && r!=ETIMEDOUT && r!=EINVAL {
        if state>=DT_STATUS::DT_DETACHED as c_int { a_crash(); }
        r = timedwait_cp(ptr::addr_of_mut!((*t).detach_state), state, CLOCK_REALTIME, at, 1);
        state = (*t).detach_state;
    }
    pthread_setcancelstate(cs, ptr::null_mut());
    if r==ETIMEDOUT || r==EINVAL { return r; }
    __tl_sync(t);
    if res!=ptr::null_mut() {
        *res = (*t).result;
    }
    if !(*t).map_base.is_null() {
        __munmap((*t).map_base as *mut c_void, (*t).map_size);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_join(t: pthread_t, res: *mut *mut c_void) -> c_int
{
    pthread_timedjoin_np(t, res, ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn pthread_tryjoin_np(t: pthread_t, res: *mut *mut c_void) -> c_int
{
    return if (*t).detach_state==DT_STATUS::DT_JOINABLE as c_int { EBUSY }
        else { pthread_join(t, res) };
}
