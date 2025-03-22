use crate::include::ctype::*;
use core::ptr;
use crate::arch::atomic_arch::*;
use crate::arch::syscall_arch::*;

#[no_mangle]
pub extern "C" fn wait(addr: *mut c_int, waiters: *mut c_int, val: c_int, lock_priv: c_int) -> ()
{
    let mut spins: c_int = 100;
    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { lock_priv };
    while spins != 0 && (waiters.is_null() || unsafe {ptr::read_volatile(waiters)} == 0) {
        spins -= 1;
        // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
        if unsafe {ptr::read_volatile(addr)} == val {
            a_spin();
        } else {
            return;
        }
    }
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    if !waiters.is_null() {
        a_inc(waiters);
    }
    while unsafe {ptr::read_volatile(addr)} == val {
        unsafe {
            let _ = __syscall4(libc::SYS_futex, addr as c_long, (libc::FUTEX_WAIT|lock_priv) as c_long, val as c_long, 0 as c_long) != -libc::ENOSYS as c_long
            || __syscall4(libc::SYS_futex, addr as c_long, libc::FUTEX_WAIT as c_long, val as c_long, 0 as c_long) != 0;
        }
    }
    if !waiters.is_null(){
        a_dec(waiters);
    }
}