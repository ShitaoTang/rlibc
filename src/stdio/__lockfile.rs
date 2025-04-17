use core::ptr;
use crate::arch::atomic_arch::*;
use crate::include::ctype::*;
use crate::internal::stdio_impl::*;
use crate::thread::pthread_impl::{__futexwait, __wake};
use crate::thread::pthread_self::pthread_self;

#[no_mangle]
pub unsafe fn __lockfile(f: *mut FILE) -> c_int
{
    let mut owner = (*f).lock;
    let tid = (*pthread_self()).tid;
    if (owner & !MAYBE_WAITERS) == tid { return 0; }
    owner = a_cas(&mut (*f).lock, 0, tid);
    if owner == 0 { return 1; }
    
    owner = a_cas(&mut (*f).lock, 0, tid | MAYBE_WAITERS);
    while owner!=0 {
        if (owner & MAYBE_WAITERS) !=0 ||
            a_cas(&mut (*f).lock, owner, owner|MAYBE_WAITERS) == owner {
            __futexwait(ptr::addr_of_mut!((*f).lock) as *mut c_void, owner|MAYBE_WAITERS, 1);
        }
        owner = a_cas(&mut (*f).lock, 0, tid | MAYBE_WAITERS);
    }

    1
}

#[no_mangle]
pub unsafe fn __unlockfile(f: *mut FILE)
{
    if (a_swap(&mut (*f).lock, 0) & MAYBE_WAITERS) != 0 {
        __wake(ptr::addr_of_mut!((*f).lock) as *mut c_void, 1, 1);
    }
}