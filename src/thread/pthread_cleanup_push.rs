use crate::include::ctype::*;
use super::*;
use super::pthread_create::*;

#[no_mangle]
pub unsafe extern "C" fn _pthread_cleanup_push(
    cb: *mut __ptcb,
    f: Option<unsafe extern "C" fn(*mut c_void)>,
    x: *mut c_void)
{
    (*cb).__f = f;
    (*cb).__x = x;
    __do_cleanup_push(cb);
}

#[no_mangle]
pub unsafe extern "C" fn _pthread_cleanup_pop(cb: *mut __ptcb, run: c_int)
{
    __do_cleanup_pop(cb);
    if run!=0 {
        if let Some(f) = (*cb).__f {
            f((*cb).__x);
        }
    }
}