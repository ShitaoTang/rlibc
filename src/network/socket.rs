use crate::include::ctype::*;
use crate::internal::syscall::__socketcall;
use crate::internal::syscall_ret::__syscall_ret;
use crate::arch::syscall_arch::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
// use super::*;

#[no_mangle]
pub extern "C" fn socket(domain: c_int, stype: c_int, protocol: c_int) -> c_int
{
    let mut s: c_int = __socketcall(SYS_socket as c_int, 
                                 domain as c_long, stype as c_long, protocol as c_long,
                                 0, 0, 0) as c_int;
    if (s==-EINVAL || s==-EPROTONOSUPPORT)
       && (stype&(libc::SOCK_CLOEXEC|libc::SOCK_NONBLOCK))!=0 {
        s = __socketcall(SYS_socket as c_int, 
                         domain as c_long, stype as c_long&(!(libc::SOCK_CLOEXEC|libc::SOCK_NONBLOCK) as c_long), protocol as c_long,
                         0, 0, 0) as c_int;
        if s < 0 {return __syscall_ret(s as u64) as c_int;}
        if (stype&libc::SOCK_CLOEXEC)!=0 {
            unsafe {
                __syscall3(SYS_fcntl as c_long, s as c_long, libc::F_SETFD as c_long, libc::FD_CLOEXEC as c_long);
            }
        }
        if (stype&libc::SOCK_NONBLOCK)!=0 {
            unsafe {
                __syscall3(SYS_fcntl as c_long, s as c_long, libc::F_SETFL as c_long, libc::O_NONBLOCK as c_long);
            }
        }
    }

    __syscall_ret(s as u64) as c_int
}