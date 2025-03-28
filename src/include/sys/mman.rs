use crate::include::ctype::*;

pub const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;

pub const MAP_SHARED:     c_long = 0x01;
pub const MAP_PRIVATE:    c_long = 0x02;
pub const MAP_SHARED_VALIDATE: c_long = 0x03;
pub const MAP_TYPE:       c_long = 0x0f;
pub const MAP_FIXED:      c_long = 0x10;
pub const MAP_ANNO:       c_long = 0x20;
pub const MAP_ANONYMOUS:  c_long = MAP_ANNO;
pub const MAP_NORESERVE:  c_long = 0x4000;
pub const MAO_GROWSDOWN:  c_long = 0x0100;
pub const MAP_DENYWRITE:  c_long = 0x0800;
pub const MAP_EXECUTABLE: c_long = 0x1000;
pub const MAP_LOCKED:     c_long = 0x2000;
pub const MAP_POPULATE:   c_long = 0x8000;
pub const MAP_NONBLOCK:   c_long = 0x10000;
pub const MAP_STACK:      c_long = 0x20000;
pub const MAP_HUGETLB:    c_long = 0x40000;
pub const MAP_SYNC:       c_long = 0x80000;
pub const MAP_FIXED_NOREPLACE: c_long = 0x100000;
pub const MAP_FILE:       c_long = 0;

pub const MAP_HUGE_SHIFT: size_t = 26;
pub const MAP_HUGE_MASK:  size_t = 0x3f;
pub const MAP_HUGE_16KB:  size_t = 14 << 26;
pub const MAP_HUGE_64KB:  size_t = 16 << 26;
pub const MAP_HUGE_512KB: size_t = 19 << 26;
pub const MAP_HUGE_1MB:   size_t = 20 << 26;
pub const MAP_HUGE_2MB:   size_t = 21 << 26;
pub const MAP_HUGE_8MB:   size_t = 23 << 26;
pub const MAP_HUGE_16MB:  size_t = 24 << 26;
pub const MAP_HUGE_32MB:  size_t = 25 << 26;
pub const MAP_HUGE_256MB: size_t = 28 << 26;
pub const MAP_HUGE_512MB: size_t = 29 << 26;
pub const MAP_HUGE_1GB:   size_t = 30 << 26;
pub const MAP_HUGE_2GB:   size_t = 31 << 26;
pub const MAP_HUGE_16GB:  size_t = 34 << 26;

pub const PROT_NONE:      c_long = 0;
pub const PROT_READ:      c_long = 1;
pub const PROT_WRITE:     c_long = 2;
pub const PROT_EXEC:      c_long = 4;
pub const PROT_GROWSDOWN: c_long = 0x01000000;
pub const PROT_GROWSUP:   c_long = 0x02000000;

pub const MS_ASYNC:      c_long = 0x01;
pub const MS_INVALIDATE: c_long = 0x02;
pub const MS_SYNC:       c_long = 0x04;

pub const MCL_CURRENT: c_long = 0x01;
pub const MCL_FUTURE:  c_long = 0x02;
pub const MCL_ONFAULT: c_long = 0x04;

pub const POSIX_MADV_NORMAL:     c_long = 0;
pub const POSIX_MADV_RANDOM:     c_long = 1;
pub const POSIX_MADV_SEQUENTIAL: c_long = 2;
pub const POSIX_MADV_WILLNEED:   c_long = 3;
pub const POSIX_MADV_DONTNEED:   c_long = 4;