use libc::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void, size_t, c_long};

pub mod socket;
pub mod bind;
pub mod listen;
pub mod connect;
pub mod sendto;
pub mod send;
pub mod sendmsg;
pub mod recvfrom;
pub mod recv;
pub mod recvmsg;

type sa_family_t = c_ushort;
type socklen_t = c_uint;

// 2B+14B=16B, better for alignment
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "big"))]
    pub __pad1: c_int,
    pub msg_iovlen: c_int,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "little"))]
    pub __pad1: c_int,
    pub msg_control: *mut c_void,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "big"))]
    pub __pad2: c_int,
    pub msg_controllen: size_t,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "little"))]
    pub __pad2: c_int,
    pub msg_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsghdr {
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "big"))]
    pub __pad1: c_int,
    pub cmsg_len: socklen_t,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "little"))]
    pub __pad1: c_int,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

// #[cfg(target_os = "linux")]
// pub const SOCK_CLOEXEC: i32 = 0o2000000;
// #[cfg(target_os = "linux")]
// pub const SOCK_NONBLOCK: i32 = 0o4000;

#[cfg(target_pointer_width = "64")]
pub const fn CMSG_ALIGN(len: usize) -> usize
{
    (len + 8 - 1) & (!(8-1))
}

#[cfg(target_pointer_width = "32")]
pub const fn CMSG_ALIGN(len: usize) -> usize
{
    (len + 4 - 1) & (!(4-1))
}

pub const fn CMSG_SPACE(len: usize) -> usize
{
    CMSG_ALIGN(len) + CMSG_ALIGN(core::mem::size_of::<cmsghdr>())
}

pub fn __CMSG_LEN(cmsg: *const cmsghdr) -> size_t
{
    // (len + alignmeng) & !alignment
    unsafe {
        ((*cmsg).cmsg_len as usize + core::mem::size_of::<c_long>() as usize - 1) & !((core::mem::size_of::<c_long>() - 1) as c_long) as usize
    }
}

pub fn __CMSG_NEXT(cmsg: *const cmsghdr) -> *mut c_uchar
{
    unsafe {
        (cmsg as *mut c_uchar).add(__CMSG_LEN(cmsg) as usize)
    }
}

pub fn __MHDR_END(mhdr: *const msghdr) -> *mut c_uchar
{
    unsafe {
        ((*mhdr).msg_control as *mut c_uchar).add((*mhdr).msg_controllen as usize)
    }
}

pub fn CMSG_FIRSTHDR(msg: *const msghdr) -> *mut cmsghdr
{
    if unsafe { (*msg).msg_controllen } >= core::mem::size_of::<cmsghdr>() as size_t {
        unsafe { (*msg).msg_control as *mut cmsghdr }
    } else {
        core::ptr::null_mut()
    }
}

pub fn CMSG_NEXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr
{
    unsafe {
        if (*cmsg).cmsg_len < core::mem::size_of::<cmsghdr>() as u32 {
            return core::ptr::null_mut();
        }
        let remaining = __MHDR_END(mhdr).offset_from(cmsg as *mut c_uchar) as usize;

        if ((*cmsg).cmsg_len < core::mem::size_of::<cmsghdr>() as u32) ||
            (__CMSG_LEN(cmsg) as usize + core::mem::size_of::<cmsghdr>() >= remaining)  {
            return core::ptr::null_mut();
        } else {
            return __CMSG_NEXT(cmsg) as *mut cmsghdr;
        }
    }
}