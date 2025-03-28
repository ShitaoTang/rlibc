use crate::include::ctype::size_t;
pub mod generic;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

#[cfg(target_arch = "x86_64")]
pub const TLS_ABOVE_TP: bool = false;
#[cfg(target_arch = "aarch64")]
pub const TLS_ABOVE_TP: bool = true;

#[cfg(target_arch = "x86_64")]
pub const GAP_ABOVE_TP: size_t = 0;
#[cfg(target_arch = "aarch64")]
pub const GAP_ABOVE_TP: size_t = 16;