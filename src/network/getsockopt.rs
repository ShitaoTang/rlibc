use crate::include::ctype::*;
use crate::internal::{syscall::{socketcall, SO_RCVTIMEO_OLD, SO_SNDTIMEO_OLD, SO_TIMESTAMP_OLD, SO_TIMESTAMPNS_OLD}, syscall_ret::__syscall_ret};
use super::socklen_t;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use super::*;
use crate::include::time::*;

#[no_mangle]
pub extern "C" fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int
{
    let mut tv32: [c_long; 2] = [0; 2];
    let tv: *mut timeval;
    let mut optname: c_int = optname;
    
    let mut r: c_int = socketcall(SYS_getsockopt as c_int, fd as c_long, level as c_long,
         optname as c_long, optval as c_long, optlen as c_long, 0) as c_int;

    if r == -ENOPROTOOPT as c_int { match level {
    SOL_SOCKET => { 
        match optname {
        SO_RCVTIMEO | SO_SNDTIMEO => {
            if SO_RCVTIMEO == SO_RCVTIMEO_OLD { return __syscall_ret(r as u64) as c_int; }
            if unsafe { *optlen } < size_of::<timeval>() as socklen_t { return __syscall_ret(-EINVAL as u64) as c_int; }
            if optname == SO_RCVTIMEO { optname = SO_RCVTIMEO_OLD; }
            if optname == SO_SNDTIMEO { optname = SO_SNDTIMEO_OLD; }
            r = socketcall(SYS_getsockopt as c_int, fd as c_long, level as c_long,
                 optname as c_long, tv32.as_mut_ptr() as c_long,
                  (size_of::<c_long>() * 2).try_into().unwrap(), 0
                ) as c_int;
            if r < 0 { return __syscall_ret(r as u64) as c_int; }
            tv = optval as *mut timeval;
            unsafe { (*tv).tv_sec = tv32[0]; }
            unsafe { (*tv).tv_usec = tv32[1]; }
            unsafe { (*optlen) = size_of::<timeval>() as socklen_t; }
            return __syscall_ret(r as u64) as c_int;
        }
        SO_TIMESTAMP | SO_TIMESTAMPNS => {
            if SO_TIMESTAMP == SO_TIMESTAMP_OLD { return __syscall_ret(r as u64) as c_int; }
            if optname == SO_TIMESTAMP { optname = SO_TIMESTAMP_OLD; }
            if optname == SO_TIMESTAMPNS { optname = SO_TIMESTAMPNS_OLD; }
            r = socketcall(SYS_getsockopt as c_int, fd as c_long, level as c_long,
                 optname as c_long, optval as c_long, optlen as c_long, 0) as c_int;
            return __syscall_ret(r as u64) as c_int;
        }
        _ => {}
        }
    }
    _ => {}
    }}

    __syscall_ret(r as u64) as c_int
}