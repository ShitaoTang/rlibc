use core::ptr;
use crate::arch::syscall_arch::__syscall3;
use crate::arch::syscall_bits::*;
use crate::include::ctype::*;
use crate::internal::syscall_ret::__syscall_ret;
use crate::internal::stdio_impl::*;

#[no_mangle]
pub extern "C" fn __stdio_write(f: *mut FILE, buf: *const c_uchar, len: size_t) -> size_t
{
unsafe {
    let mut iovs: [iovec; 2] = [
        iovec {
            iov_base: (*f).wbase as *mut c_void,
            iov_len: (*f).wpos.offset_from((*f).wbase) as size_t,
        },
        iovec {
            iov_base: buf as *mut c_void,
            iov_len: len,
        },
    ];
    let mut iov = iovs.as_mut_ptr();
    let mut rem: size_t = iov.read().iov_len + iov.add(1).read().iov_len;
    let mut iovcnt = 2;
    let mut cnt: ssize_t;

    loop {
        cnt = __syscall_ret(
            __syscall3(SYS_writev as c_long, (*f).fd as c_long, iov as c_long, iovcnt as c_long) as c_ulong
        ) as ssize_t;
        if cnt == rem as ssize_t {
            (*f).wend = (*f).buf.add((*f).buf_size);
            (*f).wpos = (*f).buf;
            (*f).wbase = (*f).buf;
            return len;
        }
        if cnt < 0 {
            (*f).wend = ptr::null_mut();
            (*f).wbase = ptr::null_mut();
            (*f).wpos = ptr::null_mut();
            (*f).flags |= F_ERR;
            return if iovcnt==2 {0} else {len - iov.read().iov_len};
        }
        rem -= cnt as size_t;
        if cnt > iov.read().iov_len as ssize_t {
            cnt -= iov.read().iov_len as ssize_t;
            iov = iov.add(1);
            iovcnt -= 1;
        }
        iov.read().iov_base = (iov.read().iov_base as *mut c_char).add(cnt as size_t) as *mut c_void;
        iov.read().iov_len -= cnt as size_t;
    }
}
}