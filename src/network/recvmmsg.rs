use crate::include::ctype::*;
use super::mmsghdr;
use crate::internal::syscall_ret::*;
use crate::thread::pthread_cancel::__syscall_cp_c;
use crate::arch::syscall_bits::*;

#[no_mangle]
pub extern "C" fn recvmmsg(fd: c_int, msgvec: *mut mmsghdr, vlen: c_uint, flags: c_uint, timeout: *mut libc::timespec) -> c_int
{
if c_long::MAX as u64 > c_int::MAX as u64 {
    let mut mh: *mut mmsghdr = msgvec;
    let mut i: c_uint = vlen;
    while i != 0 { unsafe {
        (*mh).msg_hdr.__pad1 = 0;
        (*mh).msg_hdr.__pad2 = 0;
        i -= 1;
        mh = mh.offset(1);
    }}
}
/* #ifdef SYS_recvmmsg_time64 */
/* [code here ...] */
/* #else */
    unsafe {
        __syscall_ret(__syscall_cp_c(SYS_recvmmsg as c_long, fd as c_long, msgvec as c_long, vlen as c_long, flags as c_long, timeout as c_long, 0) as c_ulong) as c_int
    }
/* #endif */
}