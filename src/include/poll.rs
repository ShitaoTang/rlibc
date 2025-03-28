use crate::include::ctype::*;

pub const POLLIN:     c_short = 0x001;
pub const POLLPRI:    c_short = 0x002;
pub const POLLOUT:    c_short = 0x004;
pub const POLLERR:    c_short = 0x008;
pub const POLLHUP:    c_short = 0x010;
pub const POLLNVAL:   c_short = 0x020;
pub const POLLRDNORM: c_short = 0x040;
pub const POLLRDBAND: c_short = 0x080;
pub const POLLWRNORM: c_short = 0x100;
pub const POLLWRBAND: c_short = 0x200;
pub const POLLMSG:    c_short = 0x400;
pub const POLLRDHUP:  c_short = 0x2000;

pub type nfds_t = c_ulong;

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}