use libc::{c_int, c_long, c_uint, c_ulong, ssize_t};
use crate::{network::{mmsghdr, sendmsg::sendmsg}, thread::pthread::__syscall_cp_c};
use super::IOV_MAX;
use crate::internal::syscall_ret::*;

#[no_mangle]
pub extern "C" fn sendmmsg(fd: c_int, msgvec: *mut mmsghdr, vlen: c_uint, flags: c_uint) -> c_int
{
    let mut i: c_int = 0;
    let mut vlen = vlen;
if c_long::MAX as u64 > c_int::MAX as u64 {
    if vlen > IOV_MAX as c_uint { vlen = IOV_MAX as c_uint; }
    if vlen == 0 { return 0; }
    while i < vlen as c_int {
        let r: ssize_t = unsafe {
            sendmsg(fd, &mut (*msgvec.offset(i as isize)).msg_hdr, flags as c_int)
        };
        if r < 0 { return if i != 0 { i } else { -1 }; }
        unsafe { (*msgvec.offset(i as isize)).msg_len = r as c_uint; }
        i += 1;
    }
    return if i != 0 { i } else { -1 };
} else {
    unsafe {
        __syscall_ret(__syscall_cp_c(libc::SYS_sendmmsg as c_long, fd as c_long, msgvec as c_long, vlen as c_long, flags as c_long, 0, 0) as c_ulong) as c_int
    }
}
}