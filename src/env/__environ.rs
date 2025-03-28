use crate::include::ctype::c_char;

#[no_mangle]
pub static mut environ: *mut *mut c_char = core::ptr::null_mut();