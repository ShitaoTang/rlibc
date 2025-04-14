use crate::include::ctype::*;
use crate::string::strcmp::*;
use core::ptr;

#[inline(always)]
fn swapc(x: uint32_t, c: c_int) -> uint32_t
{
    if c != 0 {
        return x>>24 | x>>8&0xff00 | x<<8&0xff0000 | x<<24;
    } else {
        return x;
    }
}

#[no_mangle]
pub unsafe fn __mo_lookup(p: *const c_void, size: size_t, s: *const c_char) -> *const c_char
{
    let mo: *const uint32_t = p as *const uint32_t;
    /* if ``.mo` start with 0x950412de, it's big endian */
    let sw = (*mo - 0x950412de) as c_int;
    let mut b: uint32_t = 0;
    let mut n = swapc(mo.add(2).read(), sw);
    let mut o = swapc(mo.add(3).read(), sw);
    let mut t = swapc(mo.add(4).read(), sw);

    if (n as size_t)>=size/4
     ||(o as size_t)>=(size - 4*n as size_t)
     ||(t as size_t)>=(size - 4*n as size_t)
     ||((o|t)%4 != 0) {
        return ptr::null();
    }

    o/=4;
    t/=4;

    loop {
        let ol = swapc(mo.add((o+2*(b+n/2)) as usize).read(), sw);
        let os = swapc(mo.add((o+2*(b+n/2)+1) as usize).read(), sw);
        if (os as size_t) >= size
         ||(ol as size_t) >= size - os as size_t
         ||(p as *const c_char).add((os+ol) as usize).read()!=0 {
            return ptr::null();
        }

        let sign = strcmp(s, (p as *const c_char).add(os as usize));
        if sign==0 {
            let tl = swapc(mo.add((t+2*(b+n/2)) as usize).read(), sw);
            let ts = swapc(mo.add((t+2*(b+n/2)+1) as usize).read(), sw);
            if (ts as size_t) >= size
             ||(tl as size_t) >= size - ts as size_t
             ||(p as *const c_char).add((ts+tl) as usize).read()!=0 {
                return ptr::null();
            }
            return (p as *const c_char).add(ts as usize);
        }
        else if n == 1 { return ptr::null(); }
        else if sign < 0 {
            n /= 2;
        }
        else {
            b += n / 2;
            n -= n / 2;
        }
    }
}