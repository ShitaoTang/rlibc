use crate::include::ctype::*;
use core::ptr;
use super::*;
use super::pthread_self::*;
use super::syscall_cp::*;
use crate::arch::syscall_arch::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;

#[no_mangle]
pub extern "C" fn testcancel() -> ()
{
    let _self: pthread_t = pthread_self();
    // unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    if unsafe{ptr::read_volatile(ptr::addr_of_mut!((*_self).cancel))} != 0 
    && unsafe{ptr::read_volatile(ptr::addr_of!((*_self).canceldisable))} == 0 {
        cancel();
    } 
}

#[no_mangle]
pub extern "C" fn cancel() -> c_long
{
    let _self: pthread_t = pthread_self();
    // unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    unsafe {
        if {ptr::read_volatile(ptr::addr_of!((*_self).canceldisable))} == PTHREAD_CANCEL_ENABLE as u8
        || {ptr::read_volatile(ptr::addr_of!((*_self).cancelasync))} != 0 {
            libc::pthread_exit(PTHREAD_CANCELED);
        }
    }
    unsafe{ptr::write_volatile(ptr::addr_of_mut!((*_self).canceldisable), PTHREAD_CANCEL_DISABLE as u8);}
    
    -ECANCELED as c_long
}

#[no_mangle]
pub unsafe extern "C" fn __syscall_cp_c(nr: syscall_arg_t,
                             u: syscall_arg_t, v: syscall_arg_t, w: syscall_arg_t,
                             x: syscall_arg_t, y: syscall_arg_t, z: syscall_arg_t) -> c_long 
{
    // let mut _self: pthread_t = pthread_self();
    let mut _self: pthread_t = ptr::null_mut();
    _self = pthread_self();
    let mut r: c_long;
    let st: c_int;

    st = (*_self).canceldisable as c_int;
    if st != 0 && (st==PTHREAD_CANCEL_DISABLE || nr==SYS_close as c_long) {
        return __syscall6(nr, u, v, w, x, y, z);
    }

    // unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    r = __syscall_cp_asm(ptr::addr_of!((*_self).cancel) as *const i32, nr, u, v, w, x, y, z);
    if r==-EINTR as c_long && nr!=SYS_close as c_long && (*_self).cancel!=0 && (*_self).canceldisable != PTHREAD_CANCEL_DISABLE as u8 {
        r = cancel();
    }

    r
}