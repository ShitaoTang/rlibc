use crate::include::ctype::*;
use crate::stdio::__uflow::*;
use crate::include::stdio::*;

#[no_mangle]
pub extern "C" fn __shlim(f: &mut FILE, lim: off_t) -> ()
{
    f.shlim = lim;
    unsafe {
        f.shcnt = f.buf.offset_from(f.rpos) as off_t;
        if lim!=0 && f.rend.offset_from(f.rpos) > lim as isize {
            f.shend = f.rpos.offset(lim as isize);
        } else {
            f.shend = f.rend;
        }
    }
}

#[no_mangle]
pub extern "C" fn __shgetc(f: &mut FILE) -> c_int
{
    let c: c_int = __uflow(f);
    let mut cnt = shcnt(f) as c_int;
    if f.shlim!=0 && cnt>=f.shlim as c_int || c<0 {
        f.shcnt = unsafe { f.buf.offset_from(f.rpos) } as off_t + cnt as off_t;
        f.shend = f.rpos;
        f.shlim = -1;
        return EOF;
    }
    cnt += 1;

    unsafe {
        if f.shlim!=0 && f.rend.offset_from(f.rpos) > (f.shlim-cnt as off_t) as isize {
            f.shend = f.rpos.offset((f.shlim-cnt as off_t) as isize);
        } else {
            f.shend = f.rend;
        }
        f.shcnt = f.buf.offset_from(f.rpos) as off_t + cnt as off_t;
        if f.rpos <= f.buf {
            *f.rpos.sub(1) = c as c_uchar;
        }
    }

    c   
}

#[no_mangle]
pub extern "C" fn shcnt(f: &mut FILE) -> off_t
{
    unsafe{
        f.shcnt + (f.rpos.offset_from(f.buf)) as off_t
    }
}

#[no_mangle]
pub extern "C" fn shlim(f: &mut FILE, lim: off_t) -> ()
{
    __shlim(f, lim)
}

#[no_mangle]
pub extern "C" fn shgetc(f: &mut FILE) -> c_int
{
    if f.rpos != f.shend {
        let c = unsafe { *f.rpos };
        f.rpos = unsafe { f.rpos.add(1) };
        return c as c_int;
    } else {
        __shgetc(f)
    }
}

#[no_mangle]
pub extern "C" fn shunget(f: &mut FILE) -> ()
{
    if f.shlim >= 0 {
        f.rpos = unsafe { f.rpos.sub(1) };
    }
}

#[no_mangle]
pub extern "C" fn sh_fromstring(f: &mut FILE, s: *const c_char) -> ()
{
    f.rpos = s as *mut c_uchar;
    f.buf = s as *mut c_uchar;
    f.rend = core::ptr::null_mut::<c_uchar>().wrapping_sub(1);
}