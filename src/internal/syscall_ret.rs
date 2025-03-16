use libc::{c_int, c_long, c_ulong};

use crate::thread::pthread::*;

pub fn __syscall_ret(r: c_ulong) -> c_long
{
    if r > (!0-4095) as c_ulong {
        let _self: pthread_t = pthread_self();
        unsafe {
            (*_self).errno_val = r as c_int;
            return -1;
        }
    }
    r as c_long
}