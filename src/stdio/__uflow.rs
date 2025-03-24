use crate::include::ctype::*;
use super::__toread::*;
use crate::include::stdio::*;

#[no_mangle]
pub extern "C" fn __uflow(f: &mut FILE) -> c_int
{
    let mut c: c_uchar = 0;
    if __toread(f)==0 {
        if let Some(read_fn) = f.read {
            if read_fn(f, &mut c, 1) == 1 { return c as c_int; }
        }
    }
    EOF as c_int
}