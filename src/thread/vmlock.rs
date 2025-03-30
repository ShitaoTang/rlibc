use core::ptr;
use crate::arch::atomic_arch::*;
use crate::include::ctype::*;
use super::__wait::*;
use super::pthread_impl::*;

pub static mut vmlock: [c_int; 2] = [0; 2];
extern "C" {
    pub static __vmlock_lockptr: *const c_int;
}

#[no_mangle]
pub unsafe extern "C" fn vm_wait() -> ()
{
    let mut tmp = vmlock[0];
    loop {
        if tmp == 0 {break;}
        wait(ptr::addr_of_mut!(vmlock) as *mut c_int, (ptr::addr_of_mut!(vmlock) as *mut c_int).add(1), tmp, 1);
        tmp = vmlock[0];
    }
}

#[no_mangle]
pub unsafe extern "C" fn vm_lock() -> ()
{
    a_inc(ptr::addr_of_mut!(vmlock) as *mut c_int);
}

#[no_mangle]
pub unsafe extern "C" fn vm_unlock() -> ()
{
    if a_fetch_add(ptr::addr_of_mut!(vmlock) as *mut c_int, -1) == 1 && vmlock[1] != 0{
        wake(ptr::addr_of!(vmlock) as *mut c_int, -1, 1);
    }
}