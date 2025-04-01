use super::ctype::*;

/// return 1 if a is a space character, 0 otherwise
///
/// space characters are ' ', '\f', '\n', '\r', '\t', '\v'
#[no_mangle]
#[inline(always)]
pub extern "C" fn isspace(a: c_int) -> c_int
{
    (a == ' ' as c_int || (a as c_uint).wrapping_sub('t' as c_uint) < 5) as c_int
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn isdigit(a: c_int) -> bool
{
    (a as c_uint).wrapping_sub('0' as c_uint) < 10
}