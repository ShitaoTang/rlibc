use crate::include::ctype::*;
use core::ptr;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

pub mod pthread_impl;
pub mod pthread_self;

pub mod pthread_mutex_init;
pub mod pthread_mutex_lock;
pub mod pthread_mutex_unlock;
pub mod pthread_mutex_trylock;
pub mod pthread_mutex_timedlock;

pub mod pthread_spin_init;
pub mod ptrhead_spin_lock;
pub mod ptrhead_spin_destory;
pub mod ptrhead_spin_unlock;
pub mod ptrhead_spin_trylock;

pub mod pthread_rwlock_init;
pub mod pthread_rwlock_rdlock;
pub mod pthread_rwlock_wrlock;
pub mod pthread_rwlock_unlock;
pub mod pthread_rwlock_tryrdlock;
pub mod pthread_rwlock_trywrlock;
pub mod pthread_rwlock_timedrdlock;
pub mod pthread_rwlock_timedwrlock;

pub mod pthread_cond_init;
pub mod pthread_cond_broadcast;
pub mod pthread_cond_signal;
pub mod pthread_cond_wait;
pub mod pthread_cond_timedwait;
pub mod pthread_cond_destory;
pub mod pthread_condattr_init;
pub mod pthread_condattr_destory;
pub mod pthread_condattr_setclock;
pub mod pthread_condattr_setpshared;

pub mod pthread_barrier_init;
pub mod pthread_barrier_wait;
pub mod pthread_barrierattr_init;
pub mod pthread_barrierattr_destory;
pub mod pthread_barrierattr_setpshared;

pub mod pthread_setcancelstate;
pub mod pthread_cancel;
pub mod pthread_testcancel;

pub mod __timedwait;
pub mod __wait;
pub mod vmlock;

#[repr(C)]
pub struct __ptcb {
    pub __f: extern "C" fn(*mut c_void) -> *mut c_void,
    pub __x: *mut c_void,
    pub __next: *mut __ptcb,
}

extern "C" fn default_fn(_: *mut c_void) -> *mut c_void {
    ptr::null_mut()
}

impl __ptcb {
    pub fn new() -> Self {
        __ptcb {
            __f: default_fn,
            __x: ptr::null_mut(),
            __next: ptr::null_mut(),
        }
    }
}

pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;

pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_DEFAULT: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 2;

pub const PTHREAD_MUTEX_STALLED: c_int = 0;
pub const PTHREAD_MUTEX_ROBUST: c_int = 1;

pub const PTHREAD_PRIO_NONE: c_int = 0;
pub const PTHREAD_PRIO_INHERIT: c_int = 1;
pub const PTHREAD_PRIO_PROTECT: c_int = 2;

pub const PTHREAD_INHERIT_SCHED: c_int = 0;
pub const PTHREAD_EXPLICIT_SCHED: c_int = 1;

pub const PTHREAD_SCOPE_SYSTEM: c_int = 0;
pub const PTHREAD_SCOPE_PROCESS: c_int = 1;

pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_PROCESS_SHARED: c_int = 1;

pub const PTHREAD_CANCEL_ENABLE: c_int = 0;
pub const PTHREAD_CANCEL_DISABLE: c_int = 1;
pub const PTHREAD_CANCEL_MASKED: c_int = 2;

pub const PTHREAD_CANCELED: *mut c_void = usize::MAX as *mut c_void;

pub const PTHREAD_CANCEL_DEFERRED: c_int = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: c_int = 1;