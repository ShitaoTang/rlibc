use super::ctype::*;

#[inline(always)]
pub extern "C" fn bswap_16(x: uint16_t) -> uint16_t
{
    x<<8 | x>>8
}

#[inline(always)]
pub extern "C" fn bswap_32(x: uint32_t) -> uint32_t
{
    x<<24 | x<<8&0xff0000 | x>>8&0xff00 | x>>24
}

#[inline(always)]
pub extern "C" fn bswap_64(x: uint64_t) -> uint64_t
{
    bswap_32(x as uint32_t) as uint64_t |
    bswap_32((x>>32) as uint32_t) as uint64_t
}
