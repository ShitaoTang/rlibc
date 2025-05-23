use crate::include::ctype::*;
use super::*;
use core::mem::size_of;
use crate::internal::{syscall::{socketcall, SO_RCVTIMEO_OLD, SO_SNDTIMEO_OLD, SO_TIMESTAMP_OLD, SO_TIMESTAMPNS_OLD}, syscall_ret::__syscall_ret};
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::include::time::*;

fn IS_32BIT(x: i64) -> bool
{
    !(((x as u64).wrapping_add(0x80000000)) >> 32 != 0)
}

fn CLAMP(x: i64) -> i32
{
    if IS_32BIT(x) { x as i32 }
    else { 0x7fffffff + ((x as u64) >> 63) as i32 }
}

#[no_mangle]
pub extern "C" fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int
{
    let tv: *const timeval;
    let s: time_t;
    let us: suseconds_t;
    let mut optname: c_int = optname;

    let mut r = socketcall(SYS_setsockopt as c_int, fd as c_long, level as c_long,
         optname as c_long, optval as c_long, optlen as c_long, 0) as c_int;

    if r == -ENOPROTOOPT as c_int { match level {
    SOL_SOCKET => {
        match optname {
        SO_RCVTIMEO | SO_SNDTIMEO => {
            if SO_RCVTIMEO == SO_RCVTIMEO_OLD { return __syscall_ret(r as u64) as c_int; }
            if optlen < size_of::<timeval>() as socklen_t { return __syscall_ret(-EINVAL as u64) as c_int; }
            tv = optval as *const timeval;
            s = unsafe { (*tv).tv_sec };
            us = unsafe { (*tv).tv_usec };
            if  IS_32BIT(s) == false { return __syscall_ret(-ENOTSUP as u64) as c_int; }

            if optname == SO_RCVTIMEO { optname = SO_RCVTIMEO_OLD; }
            if optname == SO_SNDTIMEO { optname = SO_SNDTIMEO_OLD; }

            r = socketcall(SYS_setsockopt as c_int, fd as c_long, level as c_long, optname as c_long, 
                [s, CLAMP(us) as i64].as_ptr() as c_long,
                (size_of::<c_long>() * 2).try_into().unwrap(), 0) as c_int;
            return __syscall_ret(r as u64) as c_int;
        }
        SO_TIMESTAMP | SO_TIMESTAMPNS => {
            if SO_TIMESTAMP == SO_TIMESTAMP_OLD { return __syscall_ret(r as u64) as c_int; }
            if optname == SO_TIMESTAMP { optname = SO_TIMESTAMP_OLD; }
            if optname == SO_TIMESTAMPNS { optname = SO_TIMESTAMPNS_OLD; }
            r = socketcall(SYS_setsockopt as c_int, fd as c_long, level as c_long,
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