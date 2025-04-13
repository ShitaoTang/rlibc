use crate::include::ctype::*;

pub fn __month_to_secs(month: c_int, is_leap: bool) -> c_int
{
    const secs_through_month: [c_int; 12] = [
        0, 31 * 86400, 59 * 86400, 90 * 86400, 120 * 86400,
        151 * 86400, 181 * 86400, 212 * 86400, 243 * 86400,
        273 * 86400, 304 * 86400, 334 * 86400
    ];
    let mut t = secs_through_month[month as usize];
    if is_leap && month >= 2 { t += 86400; }
    t
}