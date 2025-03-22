use crate::include::ctype::*;
use super::socklen_t;
use core::mem::size_of;
use crate::internal::{syscall::{socketcall, SO_RCVTIMEO_OLD, SO_SNDTIMEO_OLD, SO_TIMESTAMP_OLD, SO_TIMESTAMPNS_OLD}, syscall_ret::__syscall_ret};

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
    let tv: *const libc::timeval;
    let s: c_long;  // s: libc::time_t; time_t is long in musl-1.2.5
    let us: libc::suseconds_t;
    let mut optname: c_int = optname;

    let mut r = socketcall(libc::SYS_setsockopt as c_int, fd as c_long, level as c_long, optname as c_long, optval as c_long, optlen as c_long, 0) as c_int;

    if r == -libc::ENOPROTOOPT as c_int { match level {
    libc::SOL_SOCKET => {
        match optname {
        libc::SO_RCVTIMEO | libc::SO_SNDTIMEO => {
            if libc::SO_RCVTIMEO == SO_RCVTIMEO_OLD { return __syscall_ret(r as u64) as c_int; }
            if optlen < size_of::<libc::timeval>() as socklen_t { return __syscall_ret(-libc::EINVAL as u64) as c_int; }
            tv = optval as *const libc::timeval;
            s = unsafe { (*tv).tv_sec };
            us = unsafe { (*tv).tv_usec };
            if  IS_32BIT(s) == false { return __syscall_ret(-libc::ENOTSUP as u64) as c_int; }

            if optname == libc::SO_RCVTIMEO { optname = SO_RCVTIMEO_OLD; }
            if optname == libc::SO_SNDTIMEO { optname = SO_SNDTIMEO_OLD; }

            r = socketcall(libc::SYS_setsockopt as c_int, fd as c_long, level as c_long, optname as c_long, 
                [s, CLAMP(us) as i64].as_ptr() as c_long,
                (size_of::<c_long>() * 2).try_into().unwrap(), 0) as c_int;
            return __syscall_ret(r as u64) as c_int;
        }
        libc::SO_TIMESTAMP | libc::SO_TIMESTAMPNS => {
            if libc::SO_TIMESTAMP == SO_TIMESTAMP_OLD { return __syscall_ret(r as u64) as c_int; }
            if optname == libc::SO_TIMESTAMP { optname = SO_TIMESTAMP_OLD; }
            if optname == libc::SO_TIMESTAMPNS { optname = SO_TIMESTAMPNS_OLD; }
            r = socketcall(libc::SYS_setsockopt as c_int, fd as c_long, level as c_long, optname as c_long, optval as c_long, optlen as c_long, 0) as c_int;
            return __syscall_ret(r as u64) as c_int;
        }
        _ => {}
        }
    }
    _ => {}
    }}
    __syscall_ret(r as u64) as c_int
}