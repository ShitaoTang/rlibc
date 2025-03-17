use libc::{c_int, c_long};
use crate::arch::syscall_arch::*;
use crate::thread::pthread::*;
use super::syscall_ret::*;

type syscall_arg_t = c_long;

#[no_mangle]
pub extern "C" fn __alt_socketall(sys: c_int, cp: c_int,
                       a: syscall_arg_t, b: syscall_arg_t, c: syscall_arg_t,
                       d: syscall_arg_t, e: syscall_arg_t, f: syscall_arg_t) -> c_long
{
    let r: c_long;
    unsafe {
        if cp != 0 {
            r = __syscall_cp_c(sys as c_long, a, b, c, d, e, f);
        } else {
            r = __syscall6(sys as c_long, a, b, c, d, e, f);
        }
    }
    if r != -libc::ENOSYS as c_long {
        return r;
    }

    r
}

#[no_mangle]
pub extern "C" fn __socketcall(nm: c_int,
                    a: syscall_arg_t, b: syscall_arg_t, c: syscall_arg_t,
                    d: syscall_arg_t, e: syscall_arg_t, f: syscall_arg_t) -> c_long
{
    __alt_socketall(nm, 0, a, b, c, d, e, f)
}

#[no_mangle]
pub extern "C" fn __socketcall_cp(nm: c_int,
                       a: syscall_arg_t, b: syscall_arg_t, c: syscall_arg_t,
                       d: syscall_arg_t, e: syscall_arg_t, f: syscall_arg_t) -> c_long
{
    __alt_socketall(nm, 1, a, b, c, d, e, f)
}

#[no_mangle]
pub extern "C" fn socketcall(nm: c_int,
                  a: syscall_arg_t, b: syscall_arg_t, c: syscall_arg_t,
                  d: syscall_arg_t, e: syscall_arg_t, f: syscall_arg_t) -> c_long
{
    __syscall_ret(__socketcall(nm, a, b, c, d, e, f) as u64)
}

#[no_mangle]
pub extern "C" fn socketcall_cp(nm: c_int,
                     a: syscall_arg_t, b: syscall_arg_t, c: syscall_arg_t,
                     d: syscall_arg_t, e: syscall_arg_t, f: syscall_arg_t) -> c_long
{
    __syscall_ret(__socketcall_cp(nm, a, b, c, d, e, f) as u64)
}