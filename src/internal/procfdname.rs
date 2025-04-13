use crate::include::ctype::*;

#[no_mangle]
pub unsafe fn __procfdname(buf: *mut c_char, fd: c_uint)
{
    let mut i: size_t = 0;
    let mut j: c_uint;
    let mut fd = fd;

    buf.add(i).write(b"/proc/self/fd/\0".as_ptr().add(i).read() as c_char);
    while buf.add(i).read() != 0 {
        i += 1;
        buf.add(i).write(b"/proc/self/fd/\0".as_ptr().add(i).read() as c_char);
    }

    if fd == 0 {
        buf.add(i).write(b'0' as c_char);
        buf.add(i + 1).write(0);
        return;
    }

    j = fd;
    while j != 0 {
        j /= 10;
        i += 1;
    }
    buf.add(i).write(0);
    while fd != 0 {
        i -= 1;
        buf.add(i).write((fd % 10) as c_char + b'0' as c_char);
        fd /= 10;
    }
}