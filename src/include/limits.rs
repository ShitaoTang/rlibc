use crate::include::ctype::*;

pub const PTHREAD_KEYS_MAX: size_t = 128;
pub const PTHREAD_STACK_MIN: size_t = 2048;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: size_t = 4;

#[cfg(target_arch = "x86_64")]
pub const PAGE_SIZE: size_t = 4096;
#[cfg(target_arch = "aarch64")]
pub const PAGE_SIZE: size_t = 4096;

pub const NAME_MAX: size_t = 255;
pub const PATH_MAX: size_t = 4096;
pub const TZNAME_MAX: size_t = 6;