use crate::include::ctype::*;
use super::pthread_self::*;

#[no_mangle]
pub unsafe extern "C" fn pthread_getspecific(k: pthread_key_t) -> *mut c_void
{
    (*pthread_self()).tsd.add(k as usize).read()
}