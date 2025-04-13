use crate::arch::generic::bits::errno::*;
use crate::arch::generic::bits::fcntl::*;
use crate::include::ctype::*;
use crate::include::fcntl::*;
use crate::include::sys::sysmacros::*;
use crate::internal::syscall_ret::__syscall_ret;
use crate::internal::syscall::*;
use crate::internal::procfdname::*;
use crate::arch::syscall_bits::*;
use crate::arch::syscall_arch::*;
use crate::include::time::timespec;
use core::mem::{size_of, size_of_val};

#[repr(C)]
struct statx_time {
    tv_sec: int64_t,
    tv_nsec: uint32_t,
    pad: int32_t,
}

#[repr(C)]
struct statx {
    stx_mask: uint32_t,
    stx_blksize: uint32_t,
    stx_attributes: uint64_t,
    stx_nlink: uint32_t,
    stx_uid: uint32_t,
    stx_gid: uint32_t,
    stx_mode: uint16_t,
    pad1: uint16_t,
    stx_ino: uint64_t,
    stx_size: uint64_t,
    stx_blocks: uint64_t,
    stx_attributes_mask: uint64_t,
    stx_atime: statx_time,
    stx_btime: statx_time,
    stx_ctime: statx_time,
    stx_mtime: statx_time,
    stx_rdev_major: uint32_t,
    stx_rdev_minor: uint32_t,
    stx_dev_major: uint32_t,
    stx_dev_minor: uint32_t,
    spare: [uint64_t; 14],
}

impl statx {
    pub fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[no_mangle]
unsafe fn fstatat_statx(fd: c_int, path: *const c_char, st: *mut stat, flag: c_int) -> c_int
{
    let stx = statx::new();

    let mut flag = flag;
    flag |= AT_NO_AUTOMOUNT;
    let ret = __syscall5(SYS_statx as c_long,
        fd as c_long, path as c_long, flag as c_long,
        0x7ff as c_long, &stx as *const statx as c_long) as c_int;
    if ret!=0 { return ret; }

    *st = stat {
        st_dev: makedev(stx.stx_dev_major, stx.stx_dev_minor),
        st_ino: stx.stx_ino,
        st_mode: stx.stx_mode as mode_t,
        st_nlink: stx.stx_nlink as nlink_t,
        st_uid: stx.stx_uid,
        st_gid: stx.stx_gid,
        st_rdev: makedev(stx.stx_rdev_major, stx.stx_rdev_minor),
        st_size: stx.stx_size as off_t,
        st_blksize: stx.stx_blksize as blksize_t,
        st_blocks: stx.stx_blocks as blkcnt_t,
        st_atim: timespec {
            tv_sec: stx.stx_atime.tv_sec,
            tv_nsec: stx.stx_atime.tv_nsec as c_long,
        },
        st_mtim: timespec {
            tv_sec: stx.stx_mtime.tv_sec,
            tv_nsec: stx.stx_mtime.tv_nsec as c_long,
        },
        st_ctim: timespec {
            tv_sec: stx.stx_ctime.tv_sec,
            tv_nsec: stx.stx_ctime.tv_nsec as c_long,
        },
        ..Default::default()
    };

    0
}

#[no_mangle]
unsafe fn fstatat_kstat(fd: c_int, path: *const c_char, st: *mut stat, flag: c_int) -> c_int
{
    let mut ret;
    let mut kst = kstat::new();

    if flag==AT_EMPTY_PATH && fd>=0 && *path==0 {
        ret = __syscall2(SYS_fstat as c_long, fd as c_long, &mut kst as *mut kstat as c_long) as c_int;
        if ret==-EBADF && __syscall2(SYS_fcntl as c_long, fd as c_long, F_GETFD as c_long)>=0 {
            ret = __syscall4(SYS_fstatat as c_long, fd as c_long, path as c_long,
                &mut kst as *mut kstat as c_long, flag as c_long) as c_int;
            if ret == -EINVAL {
                let buf: [c_char; 15+3*size_of::<c_int>()] = [0; 15+3*size_of::<c_int>()];
                __procfdname(buf.as_ptr() as *mut c_char, fd as c_uint);
                #[cfg(target_arch = "x86_64")] {
                    ret = __syscall2(SYS_stat as c_long, buf.as_ptr() as c_long,
                        &mut kst as *mut kstat as c_long) as c_int;
                }
                #[cfg(target_arch = "aarch64")] {
                    ret = __syscall2(SYS_lstat as c_long, buf.as_ptr() as c_long,
                        &mut kst as *mut kstat as c_long) as c_int;
                }
            }
        }
    } else {
        #[cfg(target_arch = "x86_64")]
        if (fd==AT_FDCWD || *path==b'/' as c_char) && flag==AT_SYMLINK_NOFOLLOW {
            ret = __syscall2(SYS_lstat as c_long, path as c_long, &mut kst as *mut kstat as c_long) as c_int;
        } else if (fd==AT_FDCWD || *path==b'/' as c_char) && flag==0 {
            ret = __syscall2(SYS_stat as c_long, path as c_long, &mut kst as *mut kstat as c_long) as c_int;
        } else {
            ret = __syscall4(SYS_fstatat as c_long, fd as c_long, path as c_long,
                &mut kst as *mut kstat as c_long, flag as c_long) as c_int;
        }

        #[cfg(target_arch = "aarch64")] {
            ret = __syscall4(SYS_fstatat as c_long, fd as c_long, path as c_long,
                &mut kst as *mut kstat as c_long, flag as c_long) as c_int;
        }
    }

    if ret!=0 { return ret; }

    *st = stat {
        st_dev: kst.st_dev,
        st_ino: kst.st_ino,
        st_mode: kst.st_mode as mode_t,
        st_nlink: kst.st_nlink as nlink_t,
        st_uid: kst.st_uid,
        st_gid: kst.st_gid,
        st_rdev: kst.st_rdev,
        st_size: kst.st_size as off_t,
        st_blksize: kst.st_blksize as blksize_t,
        st_blocks: kst.st_blocks as blkcnt_t,
        st_atim: timespec {
            tv_sec: kst.st_atim_sec,
            tv_nsec: kst.st_atim_nsec as c_long,
        },
        st_mtim: timespec {
            tv_sec: kst.st_mtim_sec,
            tv_nsec: kst.st_mtim_nsec as c_long,
        },
        st_ctim: timespec {
            tv_sec: kst.st_ctim_sec,
            tv_nsec: kst.st_ctim_nsec as c_long,
        },
        ..Default::default()
    };

    0
}

#[no_mangle]
pub unsafe fn fstatat(fd: c_int, path: *const c_char, st: *mut stat, flag: c_int) -> c_int
{
    let mut ret: c_int;
    let tmp = kstat::new();
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
    if size_of_val(&tmp.st_atim_nsec) < size_of::<time_t>() {
        ret = fstatat_statx(fd, path, st, flag);
        if ret!=-ENOSYS as c_int {
            return __syscall_ret(ret as c_ulong) as c_int;
        }
    }
    ret = fstatat_kstat(fd, path, st, flag);
}

    __syscall_ret(ret as c_ulong) as c_int
}