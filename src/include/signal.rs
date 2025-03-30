use crate::include::ctype::*;

pub const SIG_BLOCK:   c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const SIG_SETMASK: c_int = 2;

pub const SI_ASYNCNL:  c_int = -60;
pub const SI_TKILL:    c_int = -6;
pub const SI_SIGIO:    c_int = -5;
pub const SI_ASYNCIO:  c_int = -4;
pub const SI_MESGQ:    c_int = -3;
pub const SI_TIMER:    c_int = -2;
pub const SI_QUEUE:    c_int = -1;
pub const SI_USER:     c_int = 0;
pub const SI_KERNEL:   c_int = 128;