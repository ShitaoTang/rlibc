#![no_std]

use core::panic::PanicInfo;
use libc;

pub mod thread;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}