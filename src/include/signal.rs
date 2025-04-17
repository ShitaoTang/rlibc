use crate::include::ctype::*;

pub const SIG_BLOCK:   c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const SIG_SETMASK: c_int = 2;

pub const SI_ASYNCNL:  c_int = -60;
pub const SI_TKILL:    c_int = -6;
pub const SI_SIGIO:    c_int = -5;
pub const SI_ASYNCIO:  c_int = -4;
pub const SI_MESGQ:    c_int = -3;
pub const SI_TIMER:    c_int = -2;
pub const SI_QUEUE:    c_int = -1;
pub const SI_USER:     c_int = 0;
pub const SI_KERNEL:   c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent_thread {
    pub sigev_notify_function: Option<unsafe extern "C" fn(sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sigevent_fields {
    __pad: [u8; 64 - 2*size_of::<c_int>() - size_of::<sigval>()],
    pub sigev_notify_thread_id: pid_t,
    pub __sev_thread: sigevent_thread,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_value: sigval,
    pub sigev_signo: c_int,
    pub sigev_notify: c_int,
    pub __sev_fields: sigevent_fields,
}