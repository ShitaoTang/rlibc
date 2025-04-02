use crate::include::ctype::*;
use crate::include::time::*;
use super::*;

/// 1970-01-01 00:00:00 UTC -> 2000-03-01 00:00:00 UTC (mod 400 year)
/// 
/// (365×23 + 366×7)×24×3600 = 946684800
pub const LEAPOCH: c_longlong = 946684800 + SECS_PER_DAY*(31+29);

/// 400/4 - 400/100 + 400/400 = 97
pub const DAYS_PER_400Y: c_longlong = 365*400 + 97;
/// 100/4 - 100/100 = 24
pub const DAYS_PER_100Y: c_longlong = 365*100 + 24;
pub const DAYS_PER_4Y:   c_longlong = 365*4   + 1;

pub const SECS_PER_DAY: c_longlong = 24*3600;

pub const MAX_SEC_YEAR: c_longlong = 366*SECS_PER_DAY;

const days_in_month: [c_int; 12] = [
    31, 30, 31, 30, 31, 31,
    30, 31, 30, 31, 31, 29
];

#[no_mangle]
pub extern "C" fn __secs_to_tm(t: c_longlong, tm: *mut tm) -> c_int
{
    let mut days: c_longlong;
    let secs: c_longlong;
    let mut years: c_longlong;
    // rem for remaining
    let mut remdays: c_int;
    let mut remsecs: c_int;
    let mut remyears: c_int;
    // q for quarter, c for century
    let mut qc_cycles: c_int;
    let mut c_cycles: c_int;
    let mut q_cycles: c_int;
    let mut months: c_int;
    let mut wday: c_int;
    let mut yday: c_int;
    let leap: c_int;

    /* Reject time_t values whose year would overflow int */
    if t < c_int::MIN as c_longlong * MAX_SEC_YEAR
        || t > c_int::MAX as c_longlong * MAX_SEC_YEAR {
        return -1;
    }

    /* Reject time_t values befor 1970- */

    secs = t - LEAPOCH;
    days = secs / SECS_PER_DAY;
    remsecs = (secs % SECS_PER_DAY) as c_int;
    if remsecs < 0 {
        remsecs += SECS_PER_DAY as c_int;
        days -= 1;
    }

    /* 2000-03-01 is Wendesday */
    wday = ((days + 3) % 7) as c_int;
    if wday < 0 { wday += 7; }

    qc_cycles = (days / DAYS_PER_400Y) as c_int;
    remdays = (days % DAYS_PER_400Y) as c_int;
    if remdays < 0 {
        remdays += DAYS_PER_400Y as c_int;
        qc_cycles -= 1;
    }

    c_cycles = (remdays as c_longlong / DAYS_PER_100Y) as c_int;
    if c_cycles==4 { c_cycles -= 1; }
    remdays -= c_cycles * DAYS_PER_100Y as c_int;

    q_cycles = (remdays as c_longlong / DAYS_PER_4Y) as c_int;
    if q_cycles==25 { q_cycles -= 1; }
    remdays -= q_cycles * DAYS_PER_4Y as c_int;

    remyears = (remdays as c_longlong / 365) as c_int;
    if remyears==4 { remyears -= 1; }
    remdays -= remyears * 365;

    leap = if remyears==0 && (q_cycles!=0 || c_cycles==0) { 1 } else { 0 };
    yday = remdays + 31 + 28 + leap;
    if yday >= 365+leap { yday -= 365+leap; }

    years = (remyears + 4*q_cycles + 100*c_cycles + 400*qc_cycles) as c_longlong;

    months = 0;
    while days_in_month[months as size_t] <= remdays {
        remdays -= days_in_month[months as size_t];
        months += 1;
    }

    if months >= 10 {
        months -= 12;
        years += 1;
    }

    if ((years+100) as c_int) > c_int::MAX || ((years+100) as c_int) < c_int::MIN {
        return -1;
    }

    unsafe {
        (*tm).tm_year = (years + 100) as c_int;
        (*tm).tm_mon = months + 2;
        (*tm).tm_mday = remdays + 1;
        (*tm).tm_wday = wday;
        (*tm).tm_yday = yday;

        (*tm).tm_sec = remsecs % 60;
        (*tm).tm_min = remsecs / 60 % 60;
        (*tm).tm_hour = remsecs / 3600;
    }

    0
}