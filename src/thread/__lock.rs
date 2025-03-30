use crate::arch::atomic_arch::*;
use crate::include::ctype::*;
use crate::include::libc;
use super::pthread_impl::{__futexwait, __wake};

#[no_mangle]
pub unsafe extern "C" fn __lock(l: *mut c_int)
{
    let need_locks = libc::libc.need_locks;
    if need_locks == 0 {
        return;
    }

    let mut current = a_cas(l, 0, c_int::MIN + 1);
    if need_locks < 0 { libc::libc.need_locks = 0; }
    if current == 0 { return; }

    let mut i: c_uint = 0;
    while i < 10 {
        if current < 0 {
            current -= c_int::MIN + 1;
        }
        // assertion: current > 0
        let val = a_cas(l, current, c_int::MIN + (current + 1));
        if val == current { return; }
        current = val;
        i += 1;
    }

    current = a_fetch_add(l, 1) + 1;

    loop {
        if current < 0 {
            __futexwait(l as *mut c_void, current, 1);
            current -= c_int::MIN + 1;
        }

        let val = a_cas(l, current, c_int::MIN + current);
        if val == current { return; }
        current = val;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __unlock(l: *mut c_int)
{
    if *l < 0 {
        if a_fetch_add(l, -(c_int::MIN+1)) != c_int::MIN+1 {
            __wake(l as *mut c_void, 1, 1);
        }
    }
}