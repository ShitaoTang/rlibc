use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn __stdio_read(f: *mut FILE, buf: *mut c_uchar, len: size_t) -> size_t
{
    len
}