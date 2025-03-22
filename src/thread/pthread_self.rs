use crate::include::ctype::*;
#[allow(unused_imports)]
use super::pthread_impl::*;
use crate::thread::pthread_arch::*;

#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub extern "C" fn pthread_self() -> pthread_t
{
    (__get_tp() - core::mem::size_of::<pthread>() as uintptr_t - TP_OFFSET as uintptr_t) as pthread_t
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn pthread_self() -> pthread_t
{
    (__get_tp()) as pthread_t
}

#[no_mangle]
pub extern "C" fn get_tid(t: pthread_t) -> c_int
{
    unsafe {(*t).tid}
}