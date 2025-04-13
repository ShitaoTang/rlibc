use crate::include::ctype::*;

#[inline(always)]
pub fn major(x: uint32_t) -> uint32_t
{
    (((x >> 31 >> 1) & 0xfffff000) | ((x >> 8) & 0x00000fff)) as uint32_t
}

#[inline(always)]
pub fn minor(x: uint32_t) -> uint32_t
{
    (((x >> 12) & 0xffffff00) | (x & 0x000000ff)) as uint32_t
}

#[inline(always)]
pub fn makedev(x: uint32_t, y: uint32_t) -> dev_t
{
    ((((x & 0xfffff000) as dev_t) << 32) |
     (((x & 0x00000fff) as dev_t) << 8) |
     (((y & 0xffffff00) as dev_t) << 12) |
     ((y & 0x000000ff) as dev_t)) as dev_t
}