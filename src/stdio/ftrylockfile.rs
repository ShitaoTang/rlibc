use core::ptr;
use crate::arch::atomic_arch::*;
use crate::include::ctype::*;
use crate::thread::pthread_self::*;

#[no_mangle]
pub unsafe extern "C" fn __do_orphaned_stdio_locks() {
    let mut f: *mut FILE = (*pthread_self()).stdio_locks as *mut FILE;
    while !f.is_null() {
        a_store(ptr::addr_of_mut!((*f).lock), 0x40000000);
        f = (*f).next_locked as *mut FILE;
    }
}