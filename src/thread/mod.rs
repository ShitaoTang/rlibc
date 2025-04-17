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

pub mod pthread_atfork;
pub mod pthread_create;
pub mod pthread_key_create;
pub mod pthread_detach;
pub mod pthread_equal;
pub mod pthread_getspecific;
pub mod pthread_join;
pub mod pthread_once;
pub mod pthread_cleanup_push;


pub mod pthread_attr_destroy;
pub mod pthread_attr_setdetachstate;
pub mod pthread_attr_setguardsize;
pub mod pthread_attr_get;
pub mod pthread_attr_setschedparam;

pub mod pthread_mutex_init;
pub mod pthread_mutex_lock;
pub mod pthread_mutex_unlock;
pub mod pthread_mutex_trylock;
pub mod pthread_mutex_timedlock;
pub mod pthread_mutex_destroy;
pub mod pthread_mutex_consistent;
pub mod pthread_mutexattr_destory;
pub mod pthread_mutexattr_init;
pub mod pthread_mutexattr_setrobust;
pub mod pthread_mutexattr_settype;

pub mod pthread_spin_init;
pub mod pthread_spin_lock;
pub mod pthread_spin_destroy;
pub mod pthread_spin_unlock;
pub mod pthread_spin_trylock;

pub mod pthread_rwlock_destroy;
pub mod pthread_rwlock_init;
pub mod pthread_rwlock_rdlock;
pub mod pthread_rwlock_wrlock;
pub mod pthread_rwlock_unlock;
pub mod pthread_rwlock_tryrdlock;
pub mod pthread_rwlock_trywrlock;
pub mod pthread_rwlock_timedrdlock;
pub mod pthread_rwlock_timedwrlock;
pub mod pthread_rwlockattr_init;
pub mod pthread_rwlockattr_destroy;

pub mod pthread_cond_init;
pub mod pthread_cond_broadcast;
pub mod pthread_cond_signal;
pub mod pthread_cond_wait;
pub mod pthread_cond_timedwait;
pub mod pthread_cond_destory;
pub mod pthread_condattr_init;
pub mod pthread_condattr_destroy;
pub mod pthread_condattr_setclock;
pub mod pthread_condattr_setpshared;

pub mod pthread_barrier_init;
pub mod pthread_barrier_wait;
pub mod pthread_barrierattr_init;
pub mod pthread_barrierattr_destroy;
pub mod pthread_barrierattr_setpshared;
pub mod pthread_barrier_destroy;

pub mod pthread_setcancelstate;
pub mod pthread_cancel;
pub mod pthread_testcancel;
pub mod pthread_setcanceltype;
pub mod pthread_setspecific;

pub mod pthread_sigmask;

pub mod lock_ptc;

pub mod __timedwait;
pub mod __wait;
pub mod vmlock;
pub mod __lock;
pub mod __unmapself;

pub mod __set_thraed_area;
pub mod default_attr;

#[repr(C)]
pub struct __ptcb {
    pub __f: Option<unsafe extern "C" fn(*mut c_void)>,
    pub __x: *mut c_void,
    pub __next: *mut __ptcb,
}

impl __ptcb {
    pub fn new() -> Self {
        __ptcb {
            __f: None,
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

pub const PTHREAD_BARRIER_SERIAL_THREAD: c_int = -1;

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __u: unsafe { core::mem::zeroed() }
};

pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __u: unsafe { core::mem::zeroed() }
};

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __u: unsafe { core::mem::zeroed() }
};