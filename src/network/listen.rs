use libc::{c_int, c_long};
use crate::internal::syscall::socketcall;

#[no_mangle]
pub extern "C" fn listen(fd: c_int, backlog: c_int) -> c_int
{
    socketcall(libc::SYS_listen as c_int, fd as c_long, backlog as c_long, 0, 0, 0, 0) as c_int
}