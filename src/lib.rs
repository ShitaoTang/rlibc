#![no_std]

use core::panic::PanicInfo;
use libc;

pub mod thread;
pub mod arch;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}