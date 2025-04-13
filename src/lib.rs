#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(c_variadic)]

#[allow(unused_imports)]
use core::panic::PanicInfo;

pub mod crt;
pub mod env;
pub mod exit;
pub mod ldso;

pub mod include;
pub mod thread;
pub mod arch;
pub mod internal;
pub mod network;
pub mod time;
pub mod string;
pub mod stdio;
pub mod stdlib;
pub mod signal;
pub mod mman;
pub mod malloc;
pub mod stat;

pub mod linux;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}