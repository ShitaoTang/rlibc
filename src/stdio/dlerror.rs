use crate::include::ctype::*;
use crate::thread::pthread_self::*;
use crate::arch::atomic_arch::*;
use core::ptr;

/// *freebuf_queue is volatile
static mut freebuf_queue: *mut c_void = core::ptr::null_mut();

#[no_mangle]
pub unsafe fn __dl_thread_cleanup()
{
    let _self = pthread_self();
    if (*_self).dlerror_buf.is_null() || (*_self).dlerror_buf == usize::MAX as *mut c_char {
        return;
    }
    let mut h: *mut c_void;
    loop {
        h = freebuf_queue;
        ptr::write(
            ptr::addr_of_mut!(freebuf_queue) as *mut *mut c_void,
            h,
        );
        if a_cas_p(
            ptr::addr_of_mut!(freebuf_queue),
            h,
            (*_self).dlerror_buf as *mut c_void,
        ) == h { break; }
    }
}