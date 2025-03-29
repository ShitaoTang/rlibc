use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn __stdio_seek(f: *mut FILE, off: off_t, whence: c_int) -> off_t
{
    0
}