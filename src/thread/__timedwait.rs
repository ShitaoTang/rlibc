use crate::include::ctype::*;
use core::ptr;
use crate::time::clock_gettime::*;
use crate::__syscall;
use crate::arch::syscall_arch::*;
use super::pthread_setcancelstate::*;
use super::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::internal::futex::*;
use crate::include::time::*;
use crate::signal::sigaction;

#[no_mangle]
pub extern "C" fn futex4_cp(addr: *mut c_void, op: c_int, val: c_int, to: *const timespec) -> c_int
{
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    // let r: c_int = unsafe {
    //     __syscall6(SYS_futex as c_long, addr as c_long, op as c_long, val as c_long, to as c_long, 0 as c_long, 0 as c_long) as c_int
    // };
    let r = __syscall!(SYS_futex, addr, op, val, to, 0, 0) as c_int;
    if r != -ENOSYS {return r;}
    let tmp = (op as c_int) & !(FUTEX_PRIVATE as c_int);
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    // unsafe {__syscall6(SYS_futex as c_long, addr as c_long, tmp as c_long, val as c_long, to as c_long, 0 as c_long, 0 as c_long) as c_int}
    __syscall!(SYS_futex, addr, tmp, val, to, 0, 0) as c_int
}

pub extern "C" fn timedwait(addr: *mut c_int, val: c_int, clk: clockid_t, at: *const timespec, lock_priv: c_int) -> c_int
{
    let mut cs: c_int = 0;
    let r: c_int;

    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, &mut cs);
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    r = timedwait_cp(addr, val, clk, at, lock_priv);
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    pthread_setcancelstate(cs, ptr::null_mut());
    r
}

#[no_mangle]
pub extern "C" fn timedwait_cp(addr: *mut c_int, val: c_int, clk: clockid_t, at: *const timespec, lock_priv: c_int) -> c_int
{
    let mut r: c_int;
    let mut to: timespec = timespec {tv_sec: 0, tv_nsec: 0};
    let mut top: *mut timespec = ptr::null_mut();

    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { 0 };

    if at != ptr::null_mut() {
        if unsafe {(*at).tv_nsec} as u64 > 1000000000u64 {return EINVAL;}   
        if __clock_gettime(clk, &mut to) != 0 {return EINVAL;}
        to.tv_sec = unsafe {(*at).tv_sec} - to.tv_sec;
        to.tv_nsec = unsafe {(*at).tv_nsec} - to.tv_nsec;
        if to.tv_nsec < 0 {
            to.tv_sec -= 1;
            to.tv_nsec += 1000000000;
        }
        if to.tv_sec < 0 {return ETIMEDOUT;}
        top = &mut to;
    }

    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    r = -futex4_cp(addr as *mut c_void, FUTEX_WAIT|lock_priv, val, top);

    if r != EINTR && r!= ETIMEDOUT && r != ECANCELED {r = 0;}
    if r == EINTR && unsafe {ptr::read_volatile(ptr::addr_of!(sigaction::__eintr_valid_flag))} == 0 {r = 0;}

    r
}
