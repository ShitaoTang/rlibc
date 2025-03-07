#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::panic::PanicInfo;

pub mod thread;
pub mod arch;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // unsafe {
    //     libc::abort();
    // }
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}