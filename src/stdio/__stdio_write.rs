use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn __stdio_write(f: *mut FILE, buf: *const c_uchar, len: size_t) -> size_t
{
    0
}