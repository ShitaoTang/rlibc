use libc::{c_char, c_uint, c_ushort};

pub mod socket;
pub mod bind;
pub mod listen;
pub mod connect;

type sa_family_t = c_ushort;
type socklen_t = c_uint;

// 2B+14B=16B, better for alignment
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

// #[cfg(target_os = "linux")]
// pub const SOCK_CLOEXEC: i32 = 0o2000000;
// #[cfg(target_os = "linux")]
// pub const SOCK_NONBLOCK: i32 = 0o4000;