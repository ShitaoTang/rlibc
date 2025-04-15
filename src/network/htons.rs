use crate::include::ctype::*;
use crate::include::byteswap::*;

/* host to network short */
#[no_mangle]
pub extern "C" fn htons(n: uint16_t) -> uint16_t
{
    if cfg!(target_endian = "little") { bswap_16(n) }
    else { n }
}