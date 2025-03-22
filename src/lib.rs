#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#[allow(unused_imports)]
use core::panic::PanicInfo;

pub mod include;
pub mod thread;
pub mod arch;
pub mod internal;
pub mod network;

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