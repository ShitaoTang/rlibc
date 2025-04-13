use crate::include::ctype::*;

pub fn __year_to_secs(year: c_longlong, is_leap: &mut bool) -> c_longlong
{
    const secs_per_year: c_longlong = 365 * 86400;
    const UNIX_OFFSET: c_longlong = 946684800;

    if year-2 <= 136 {
        let mut leaps: c_int = ((year-68)>>2) as c_int;
        if ((year-68)&3) == 0 {
            leaps -= 1;
            *is_leap = true;
        } else {
            *is_leap = false;
        }
        return (year-70) * secs_per_year + (leaps * 86400) as c_longlong;
    }

    let mut cycles: c_int;
    let centuries: c_int;
    let mut leaps: c_int;
    let mut remyears: c_int;

    cycles = ((year-100) / 400) as c_int;
    remyears = ((year-100) % 400) as c_int;
    if remyears < 0 {
        cycles -= 1;
        remyears += 400;
    }
    if remyears == 0 {
        *is_leap = true;
        centuries = 0;
        leaps = 0;
    } else {
        if remyears >= 200 {
            if remyears >= 300 { centuries = 3; remyears -= 300; }
            else { centuries = 2; remyears -= 200; }
        } else {
            if remyears >= 100 { centuries = 1; remyears -= 100; }
            else { centuries = 0; }
        }
        if remyears == 0 {
            *is_leap = false;
            leaps = 0;
        } else {
            leaps = (remyears as c_ulonglong / 4) as c_int;
            remyears = (remyears as c_ulonglong % 4) as c_int;
            *is_leap = remyears == 0;
        }
    }

    leaps += 97*cycles + 24*centuries - if *is_leap {1} else {0};

    (year-100) * secs_per_year
        + (leaps * 86400) as c_longlong
        + UNIX_OFFSET
        + 86400
}