use crate::include::ctype::*;
use crate::internal::stdio_impl::*;
use crate::include::stdio::*;
use core::ptr;

pub extern "C" fn __toread(f: &mut FILE) -> c_int {
    f.mode |= f.mode - 1;
    if f.wpos != f.wbase {
        if let Some(write_fn) = f.write {
            write_fn(f, ptr::null(), 0);
        }
    }
    f.wend = ptr::null_mut();
    f.wbase = ptr::null_mut();
    f.wpos = ptr::null_mut();
    if f.flags & F_NORD != 0 {
        f.flags |= F_ERR;
        return EOF;
    }
    f.rend = unsafe { f.buf.add(f.buf_size) };
    f.rpos = f.rend;

    return if f.flags & F_ERR != 0 { EOF }
    else { 0 };
}