use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_barrierattr_setpshared(a: *mut pthread_barrierattr_t, pshared: c_int) -> c_int
{
    if pshared as u32 > 1u32 {return libc::EINVAL;}
    unsafe {
        (*a).__attr = if pshared!=0 {libc::INT_MIN as c_uint} else {0};
    }
    0
}