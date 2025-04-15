use crate::include::ctype::*;
use crate::include::byteswap::*;

/* network to host short */
#[no_mangle]
pub extern "C" fn ntohs(n: uint16_t) -> uint16_t
{
    if cfg!(target_endian = "little") { bswap_16(n) }
    else { n }
}