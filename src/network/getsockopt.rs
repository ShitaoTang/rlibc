use libc::{c_int, c_long, c_void};
use crate::internal::{syscall::{socketcall, SO_RCVTIMEO_OLD, SO_SNDTIMEO_OLD, SO_TIMESTAMP_OLD, SO_TIMESTAMPNS_OLD}, syscall_ret::__syscall_ret};
use super::socklen_t;

#[no_mangle]
pub extern "C" fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int
{
    let mut tv32: [c_long; 2] = [0; 2];
    let tv: *mut libc::timeval;
    let mut optname: c_int = optname;
    
    let mut r: c_int = socketcall(libc::SYS_getsockopt as c_int, fd as c_long, level as c_long, optname as c_long, optval as c_long, optlen as c_long, 0) as c_int;

    if r == -libc::ENOPROTOOPT as c_int { match level {
    libc::SOL_SOCKET => { 
        match optname {
        libc::SO_RCVTIMEO | libc::SO_SNDTIMEO => {
            if libc::SO_RCVTIMEO == SO_RCVTIMEO_OLD { return __syscall_ret(r as u64) as c_int; }
            if unsafe { *optlen } < size_of::<libc::timeval>() as socklen_t { return __syscall_ret(-libc::EINVAL as u64) as c_int; }
            if optname == libc::SO_RCVTIMEO { optname = SO_RCVTIMEO_OLD; }
            if optname == libc::SO_SNDTIMEO { optname = SO_SNDTIMEO_OLD; }
            r = socketcall(libc::SYS_getsockopt as c_int, fd as c_long, level as c_long, optname as c_long, tv32.as_mut_ptr() as c_long, (size_of::<c_long>() * 2).try_into().unwrap(), 0) as c_int;
            if r < 0 { return __syscall_ret(r as u64) as c_int; }
            tv = optval as *mut libc::timeval;
            unsafe { (*tv).tv_sec = tv32[0]; }
            unsafe { (*tv).tv_usec = tv32[1]; }
            unsafe { (*optlen) = size_of::<libc::timeval>() as socklen_t; }
            return __syscall_ret(r as u64) as c_int;
        }
        libc::SO_TIMESTAMP | libc::SO_TIMESTAMPNS => {
            if libc::SO_TIMESTAMP == SO_TIMESTAMP_OLD { return __syscall_ret(r as u64) as c_int; }
            if optname == libc::SO_TIMESTAMP { optname = SO_TIMESTAMP_OLD; }
            if optname == libc::SO_TIMESTAMPNS { optname = SO_TIMESTAMPNS_OLD; }
            r = socketcall(libc::SYS_getsockopt as c_int, fd as c_long, level as c_long, optname as c_long, optval as c_long, optlen as c_long, 0) as c_int;
            return __syscall_ret(r as u64) as c_int;
        }
        _ => {}
        }
    }
    _ => {}
    }}

    __syscall_ret(r as u64) as c_int
}