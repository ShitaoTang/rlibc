use crate::include::ctype::*;
use crate::include::time::*;
use super::__year_to_secs::*;
use super::__month_to_secs::*;

pub fn __tm_to_secs(tm: &tm) -> c_longlong
{
    let mut is_leap: bool = false;
    let mut year = tm.tm_year;
    let mut month = tm.tm_mon;
    if month>=12 || month<0 {
        let mut adj = month/12;
        month %= 12;
        if month < 0 {
            month += 12;
            adj -= 1;
        }
        year += adj;
    }
    let mut t = __year_to_secs(year as c_longlong, &mut is_leap);
    t += __month_to_secs(month, is_leap) as c_longlong;
    t += (tm.tm_mday-1) as c_longlong * 86400;
    t += (tm.tm_hour as c_longlong) * 3600;
    t += (tm.tm_min as c_longlong) * 60;
    t += tm.tm_sec as c_longlong;

    t
}