use super::{in_addr, in_addr_t};
use crate::include::ctype::*;
use super::inet_aton::inet_aton;

// string in dotted-decimal notation -> 32-bit integer in network byte order
// "192.168.1.1" -> 0xC0A80101
#[no_mangle]
pub extern "C" fn inet_addr(p: *const c_char) -> in_addr_t
{
    let mut a: in_addr = in_addr { s_addr: 0 };
    if inet_aton(p, &mut a) == 0 { return u32::MAX; }
    a.s_addr
}