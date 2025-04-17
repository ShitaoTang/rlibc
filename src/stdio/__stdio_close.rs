use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn __stdio_close(_f: *mut FILE) -> c_int
{
    0
}