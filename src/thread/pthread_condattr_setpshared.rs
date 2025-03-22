use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn pthread_condattr_setpshared(a: *mut pthread_condattr_t, pshared: c_int) -> c_int
{
    if pshared as u32 > 1u32 {return libc::EINVAL;}
    unsafe {
        (*a).__attr &= 0x7fffffff;
        (*a).__attr |= (pshared << 31) as u32;
    }
    0
}