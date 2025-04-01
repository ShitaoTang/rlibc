use crate::arch::atomic_arch::{a_barrier, a_cas, a_swap};
use crate::include::ctype::*;
use super::pthread_cleanup_push::*;
use super::*;
use super::pthread_impl::*;
use crate::thread::__wait::*;

#[no_mangle]
unsafe extern "C" fn undo(control: *mut c_void)
{
    if a_swap(control as *mut c_int, 0) == 3 {
        wake(control as *mut c_int, -1, 1);
    }
}

#[no_mangle]
unsafe extern "C" fn __pthread_once_full(control: *mut pthread_once_t, init: Option<unsafe extern "C" fn()>) -> c_int
{
    /* The following comment is from musl:
     * Try to enter initializing state. Four possibilities:
     * 0 - we're the first or the other cancelled; run init
     * 1 - another thread is running init; wait
     * 2 - another thread finished running init(); just return
     * 3 - another thread is running init, waiters present; wait
     */
    loop { match a_cas(control, 0, 1) {
    0 => {
        let mut __cb: __ptcb = __ptcb {
            __f: None,
            __x: core::ptr::null_mut(),
            __next: core::ptr::null_mut(),
        };
        _pthread_cleanup_push(ptr::addr_of_mut!(__cb), Some(undo), control as *mut c_void);
        if let Some(init) = init {
            init();
        }
        _pthread_cleanup_pop(ptr::addr_of_mut!(__cb), 0);

        if a_swap(control, 2) == 3 {
            wake(control, -1, 1);
        }
        return 0;
    }
    1 => {
        a_cas(control, 1, 3);
        wait(control, ptr::null_mut(), 3, 1);
        continue;
    }
    3 => {
        wait(control, ptr::null_mut(), 3, 1);
        continue;
    }
    2 => {
        return 0;
    }
    _ => { return 0;}
    }}
}

#[no_mangle]
pub unsafe extern "C" fn pthread_once(control: *mut pthread_once_t, init: Option<unsafe extern "C" fn()>) -> c_int
{
    if ptr::read_volatile(control) == 2 {
        a_barrier();
        return 0;
    }

    __pthread_once_full(control, init)
}