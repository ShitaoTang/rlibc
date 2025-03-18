use libc::{c_int, c_void, size_t, ssize_t, c_long};
use crate::{internal::syscall::socketcall_cp, network::{CMSG_FIRSTHDR, CMSG_NEXTHDR}, thread::pthread::pthread_self};
use super::{cmsghdr, msghdr, sockaddr, socklen_t, CMSG_SPACE};
use core::mem::size_of;

#[no_mangle]
pub extern "C" fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t
{
    let mut msg = msg;
if c_long::MAX as u64 > c_int::MAX as u64 {
    let mut h: msghdr;
    const chbuf_size: usize = CMSG_SPACE(255*size_of::<c_int>())/size_of::<cmsghdr>();
    let mut chbuf: [cmsghdr; chbuf_size] = 
        [cmsghdr{cmsg_len: 0, cmsg_level: 0, cmsg_type: 0, __pad1: 0}; chbuf_size];
    let mut c: *mut cmsghdr;

    if !msg.is_null() {
        h = unsafe { *msg };
        h.__pad1 = 0;
        h.__pad2 = 0;
        msg = &h;
        if h.msg_controllen != 0 {
            if h.msg_controllen > chbuf_size {
                let _self = pthread_self();
                unsafe {(*_self).errno_val = libc::ENOMEM};
                return -1;
            }
            unsafe {
                libc::memcpy(chbuf.as_mut_ptr() as *mut c_void, h.msg_control, h.msg_controllen);
            }
            h.msg_control = chbuf.as_mut_ptr() as *mut c_void;
            c = CMSG_FIRSTHDR(&h);
            while !c.is_null() {
                unsafe { (*c).__pad1 = 0; }
                c = CMSG_NEXTHDR(&h, c);
            }
        }
    }
}
    socketcall_cp(libc::SYS_sendmsg as c_int, fd as c_long, msg as c_long, flags as c_long, 0, 0, 0) as ssize_t
}