use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn difftime(time1: time_t, time2: time_t) -> c_double
{
    (time1 - time2) as c_double
}