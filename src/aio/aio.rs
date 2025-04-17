use core::ptr;
use crate::include::ctype::*;
use crate::include::aio::*;
use crate::include::signal::*;
use crate::arch::atomic_arch::*;
use crate::arch::generic::bits::errno::*;
use crate::signal::sigfillset::sigfillset;
use crate::thread::pthread_self::pthread_self;
use crate::thread::pthread_sigmask::pthread_sigmask;

/// volatile
static mut aio_fd_cnt: c_int = 0;

#[repr(C)]
pub struct aio_thread {
    pub td: pthread_t,
    pub cb: *mut aiocb,
    pub next: *mut aio_thread,
    pub prev: *mut aio_thread,
    pub q: *mut aio_queue,
    pub running: c_int,     // volatile
    pub err: c_int,
    pub op: c_int,
    pub ret: ssize_t,
}

#[repr(C)]
pub struct aio_queue {
    pub fd: c_int,
    pub seekable: c_int,
    pub append: c_int,
    pub _ref: c_int,
    pub init: c_int,
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub head: *mut aio_thread,
}

#[no_mangle]
pub unsafe extern "C" fn aio_cancel(fd: c_int, cb: *mut aiocb) -> c_int
{
    let mut allmask =  sigset_t::new();
    let mut origmask = sigset_t::new();
    let mut _ret = AIO_ALLDONE;
    let mut _p: *mut aio_thread;
    let mut _q: *mut aio_queue;

    if !cb.is_null() && fd != (*cb).aio_fildes {
        (*pthread_self()).errno_val = EINVAL;
        return -1;
    }

    sigfillset(&mut allmask);
    pthread_sigmask(SIG_BLOCK, &mut allmask, &mut origmask);

    (*pthread_self()).errno_val = ENOENT;
    
    /* TODO */

    0
}

#[no_mangle]
pub unsafe fn __aio_close(fd: c_int) -> c_int
{
    a_barrier();
    if ptr::read_volatile(ptr::addr_of_mut!(aio_fd_cnt)) != 0 {
        aio_cancel(fd, ptr::null_mut());
    }
    fd
}