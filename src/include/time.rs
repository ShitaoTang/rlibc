use super::ctype::*;

#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub __tm_gmtoff: c_long,
    pub __tm_zone: *const c_char,
}

pub const CLOCKS_PER_SEC: c_long = 1000000;

pub const TIME_UTC: c_int = 1;

#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    pub tv_nsec: i64,
    #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

pub const CLOCK_REALTIME:           c_int = 0;
pub const CLOCK_MONOTONIC:          c_int = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: c_int = 2;
pub const CLOCK_THREAD_CPUTIME_ID:  c_int = 3;
pub const CLOCK_MONOTONIC_RAW:      c_int = 4;
pub const CLOCK_REALTIME_COARSE:    c_int = 5;
pub const CLOCK_MONOTONIC_COARSE:   c_int = 6;
pub const CLOCK_BOOTTIME:           c_int = 7;
pub const CLOCK_REALTIME_ALARM:     c_int = 8;
pub const CLOCK_BOOTTIME_ALARM:     c_int = 9;
pub const CLOCK_SGI_CYCLE:          c_int = 10;
pub const CLOCK_TAI:                c_int = 11;

pub const TIMER_ABSTIME: c_int = 1;