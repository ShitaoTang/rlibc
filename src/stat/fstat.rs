use crate::arch::generic::bits::errno::EBADF;
use crate::include::ctype::*;
use crate::include::fcntl::*;
use crate::internal::syscall_ret::__syscall_ret;
use super::fstatat::fstatat;

#[no_mangle]
pub unsafe fn fstat(fd: c_int, st: *mut stat) -> c_int
{
    if fd < 0 { __syscall_ret(-EBADF as c_ulong) as c_int }
    else { fstatat(fd, b"\0".as_ptr() as *const c_char, st, AT_EMPTY_PATH as c_int) as c_int }
}