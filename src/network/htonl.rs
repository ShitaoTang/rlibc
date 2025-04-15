use crate::include::ctype::*;
use crate::include::byteswap::*;

/* host to network long */
#[no_mangle]
pub extern "C" fn htonl(n: uint32_t) -> uint32_t
{
    if cfg!(target_endian = "little") { bswap_32(n) }
    else { n }
}