use crate::{cfg_if, include::ctype::c_int};

pub const O_CREAT:      c_int = 0o100;
pub const O_EXCL:       c_int = 0o200;
pub const O_NOCTTY:     c_int = 0o400;
pub const O_TRUNC:      c_int = 0o1000;
pub const O_APPEND:     c_int = 0o2000;
pub const O_NONBLOCK:   c_int = 0o4000;
pub const O_DSYNC:      c_int = 0o10000;
pub const O_SYNC:       c_int = 0o40000;
pub const O_RSYNC:      c_int = 0o100000;
pub const O_DIRECTORY:  c_int = 0o200000;
pub const O_NOFOLLOW:   c_int = 0o400000;
pub const O_CLOEXEC:    c_int = 0o2000000;

pub const O_ASYNC:      c_int = 0o2000;
pub const O_DIRECT:     c_int = 0o40000;
pub const O_LARGEFILE:  c_int = 0o100000;
pub const O_NOATIME:    c_int = 0o1000000;
pub const O_PATH:       c_int = 0o10000000;
pub const O_TMPFILE:    c_int = 0o20200000;
pub const O_NDELAY:     c_int = O_NONBLOCK;

pub const F_DUPFD:  c_int = 0;
pub const F_GETFD:  c_int = 1;
pub const F_SETFD:  c_int = 2;
pub const F_GETFL:  c_int = 3;
pub const F_SETFL:  c_int = 4;

pub const F_SETOWN: c_int = 8;
pub const F_GETOWN: c_int = 9;
pub const F_SETSIG: c_int = 10;
pub const F_GETSIG: c_int = 11;

cfg_if!(
    if #[cfg(target_pointer_width = "32")] {
        pub const F_GETLK:  c_int = 12;
        pub const F_SETLK:  c_int = 13;
        pub const F_SETLKW: c_int = 14;
    } else {
        pub const F_GETLK:  c_int = 5;
        pub const F_SETLK:  c_int = 6;
        pub const F_SETLKW: c_int = 7;
    }
);

pub const F_SETOWN_EX: c_int = 15;
pub const F_GETOWN_EX: c_int = 16;

pub const F_GETOWNER_UIDS: c_int = 17;