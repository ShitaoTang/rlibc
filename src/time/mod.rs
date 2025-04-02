use crate::include::ctype::*;
pub mod clock_gettime;
pub mod nanosleep;
pub mod sleep;
pub mod usleep;
pub mod clock_nanosleep;
pub mod clock;
pub mod difftime;
pub mod strftime;
pub mod time;
pub mod __secs_to_tm;
pub mod gmtime;
pub mod gmtime_r;
pub mod localtime;
pub mod localtime_r;
pub mod mktime;

pub const CLOCKS_PER_SEC: c_long = 1000000;

pub const __utc: [c_char; 4] = [b'U' as c_char, b'T' as c_char, b'C' as c_char, 0];