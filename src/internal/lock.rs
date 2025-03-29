use crate::thread::__lock::*;
use crate::include::ctype::c_int;

#[no_mangle]
pub unsafe extern "C" fn LOCK(x: *mut c_int)
{
    __lock(x)
}

#[no_mangle]
pub unsafe extern "C" fn UNLOCK(x: *mut c_int)
{
    __unlock(x)
}
