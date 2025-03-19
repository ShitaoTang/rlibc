use crate::internal::syscall::{SCM_TIMESTAMPNS_OLD, SCM_TIMESTAMP_OLD, socketcall_cp};
use super::{cmsghdr, msghdr, socklen_t, CMSG_DATA, CMSG_SPACE, CMSG_LEN};
use libc::{c_void, c_long, c_longlong, c_int, ssize_t};

// SCM: Socket Control Message
fn __convert_scm_timestamps(msg: *mut msghdr, csize: socklen_t) -> ()
{
    if libc::SCM_TIMESTAMP == SCM_TIMESTAMP_OLD {return;}
    if unsafe {(*msg).msg_control == core::ptr::null_mut() || (*msg).msg_controllen == 0} {return;}

    let mut cmsg: *mut cmsghdr;
    let mut last: *mut cmsghdr = core::ptr::null_mut();
    let mut tmp: c_long;
    let mut tvts: [c_longlong; 2] = [0; 2];
    let mut c_type: c_int = 0;

    cmsg = super::CMSG_FIRSTHDR(msg);
    while !cmsg.is_null() {
        unsafe {
            if (*cmsg).cmsg_level == libc::SOL_SOCKET { match (*cmsg).cmsg_type {
            SCM_TIMESTAMP_OLD => {
                if c_type != 0 {}
                else {
                    c_type = libc::SCM_TIMESTAMP;
                    tmp = *(CMSG_DATA(cmsg) as *const c_longlong);
                    tvts[0] = tmp;
                    tmp = *(CMSG_DATA(cmsg) as *const c_longlong).add(1);
                    tvts[1] = tmp;
                }
            }
            SCM_TIMESTAMPNS_OLD => {
                c_type = libc::SCM_TIMESTAMPNS;
                tmp = *(CMSG_DATA(cmsg) as *const c_longlong);
                tvts[0] = tmp;
                tmp = *(CMSG_DATA(cmsg) as *const c_longlong).add(1);
                tvts[1] = tmp;
            }
            _ => {}
            }
            last = cmsg;
            cmsg = super::CMSG_NEXTHDR(msg, cmsg);
            }
        }
    }

    if last.is_null() || c_type == 0 {return;}
    unsafe {
        if CMSG_SPACE(size_of::<c_longlong>() as usize * 2) > csize as usize-(*msg).msg_controllen {
            (*msg).msg_flags |= libc::MSG_CTRUNC;
            return;
        }

        (*msg).msg_controllen += CMSG_SPACE(size_of::<c_longlong>() as usize * 2) as usize;
        cmsg = super::CMSG_NEXTHDR(msg, last);
        (*cmsg).cmsg_level = libc::SOL_SOCKET;
        (*cmsg).cmsg_type = c_type;
        (*cmsg).cmsg_len = CMSG_LEN(size_of::<c_longlong>() * 2) as u32;
        libc::memcpy(CMSG_DATA(cmsg) as *mut c_void, tvts.as_ptr() as *const c_void, size_of::<c_longlong>() as usize * 2);
    }
}

#[no_mangle]
pub extern "C" fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t
{
    let mut msg = msg;
    let r: ssize_t;
    let orig_controllen: socklen_t = unsafe {(*msg).msg_controllen as socklen_t};
    let mut h: msghdr = msghdr{
        msg_name: core::ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: core::ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: core::ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
        __pad1: 0,
        __pad2: 0,
    };
    let orig: *mut msghdr = msg;
if c_long::MAX as u64 > c_int::MAX as u64 {
    if !msg.is_null() {
        h = unsafe { *msg };
        h.__pad1 = 0;
        h.__pad2 = 0;
        msg = &mut h;
    }
}
    r = socketcall_cp(libc::SYS_recvmsg as c_int, fd as c_long, msg as c_long, flags as c_long, 0, 0, 0) as ssize_t;
    if r > 0 {
        __convert_scm_timestamps(msg, orig_controllen);
    }
if c_long::MAX as u64 > c_int::MAX as u64 {
    if !orig.is_null() {
        unsafe { *orig = h; }
    }
}
    r
}