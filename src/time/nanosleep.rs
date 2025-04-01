use crate::internal::syscall_ret::__syscall_ret;
use crate::include::ctype::*;
use crate::include::time::*;
use super::clock_nanosleep::*;

#[no_mangle]
pub unsafe extern "C" fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int
{
    __syscall_ret((clock_nanosleep(CLOCK_REALTIME, 0, req, rem) as c_ulong).wrapping_neg()) as c_int
}