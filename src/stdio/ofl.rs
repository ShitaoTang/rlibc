use crate::include::ctype::*;
use core::sync::atomic::AtomicI32;
use crate::internal::lock::*;
use core::ptr;

pub static mut ofl_head: *mut FILE = core::ptr::null_mut();
pub static mut ofl_lock: [AtomicI32; 1] = [AtomicI32::new(0)];   // volatile
pub static __stdio_ofl_lockptr: &AtomicI32 = unsafe { &mut ofl_lock[0] };

#[no_mangle]
pub unsafe extern "C" fn __ofl_lock() -> *mut *mut FILE
{
    LOCK(ptr::addr_of_mut!(ofl_lock) as *mut _ as *mut c_int);
    ptr::addr_of_mut!(ofl_lock) as *const _ as *mut *mut FILE
}

#[no_mangle]
pub unsafe extern "C" fn __ofl_unlock()
{
    UNLOCK(ptr::addr_of_mut!(ofl_lock) as *mut _ as *mut c_int);
}