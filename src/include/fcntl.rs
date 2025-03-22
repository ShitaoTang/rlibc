use super::ctype::*;
use crate::arch::generic::bits::fcntl::*;

#[repr(C)]
pub struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: off_t,
    pub l_len: off_t,
    pub l_pid: pid_t,
}

pub const O_SEARCH:     c_int = O_PATH;
pub const O_EXEC:       c_int = O_PATH;
pub const O_TTY_INIT:   c_int = 0;

pub const O_ACCMODE: c_int = 0o3|O_SEARCH;
pub const O_RDONLY:  c_int = 0o0;
pub const O_WRONLY:  c_int = 0o1;
pub const O_RDWR:    c_int = 0o2;

pub const F_OFD_GETLK:  c_int = 36;
pub const F_OFD_SETLK:  c_int = 37;
pub const F_OFD_SETLKW: c_int = 38;

pub const F_DUPFD_CLOEXEC: c_int = 1030;

pub const F_RDLCK: c_int = 0;
pub const F_WRLCK: c_int = 1;
pub const F_UNLCK: c_int = 2;

pub const FD_CLOEXEC: c_int = 1;

pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_EACCESS: c_int = 0x200;

pub const POSIX_FADV_NORMAL:     c_int = 0;
pub const POSIX_FADV_RANDOM:     c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: c_int = 2;
pub const POSIX_FADV_WILLNEED:   c_int = 3;
pub const POSIX_FADV_DONTNEED:   c_int = 4;
pub const POSIX_FADV_NOREUSE:    c_int = 5;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const S_ISUID:  mode_t = 0o4000;
pub const S_ISGID:  mode_t = 0o2000;
pub const S_ISVTX:  mode_t = 0o1000;
pub const S_IRUSR:  mode_t = 0o400;
pub const S_IWUSR:  mode_t = 0o200;
pub const S_IXUSR:  mode_t = 0o100;
pub const S_IRWXU:  mode_t = 0o700;
pub const S_IRGRP:  mode_t = 0o040;
pub const S_IWGRP:  mode_t = 0o020;
pub const S_IXGRP:  mode_t = 0o010;
pub const S_IRWXG:  mode_t = 0o070;
pub const S_IROTH:  mode_t = 0o004;
pub const S_IWOTH:  mode_t = 0o002;
pub const S_IXOTH:  mode_t = 0o001;
pub const S_IRWXO:  mode_t = 0o007;