use crate::include::ctype::c_int;

pub const FUTEX_WAIT: c_int = 0;
pub const FUTEX_WAKE: c_int = 1;
pub const FUTEX_FD:   c_int = 2;
pub const FUTEX_REQUEUE:     c_int = 3;
pub const FUTEX_CMP_REQUEUE: c_int = 4;
pub const FUTEX_WAKE_OP:     c_int = 5;
pub const FUTEX_LOCK_PI:     c_int = 6;
pub const FUTEX_UNLOCK_PI:   c_int = 7;
pub const FUTEX_TRYLOCK_PI:  c_int = 8;
pub const FUTEX_WAIT_BITSET: c_int = 9;

pub const FUTEX_PRIVATE: c_int = 128;

pub const FUTEX_CLOCK_REALTIME: c_int = 256;