use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
use crate::__syscall;
use crate::include::ctype::*;
use crate::internal::stdio_impl::*;
use crate::internal::syscall_ret::__syscall_ret;

#[no_mangle]
pub extern "C" fn __stdio_read(f: *mut FILE, buf: *mut c_uchar, len: size_t) -> size_t
{
unsafe {
    let iov: [iovec; 2] = [
        iovec {
            iov_base: buf as *mut c_void,
            iov_len: len - if (*f).buf_size==0 {0} else {1},
        },
        iovec {
            iov_base: (*f).buf as *mut c_void,
            iov_len: (*f).buf_size,
        },
    ];

    let mut cnt = if iov[0].iov_len!=0 {
        // __syscall_ret(__syscall3(SYS_readv as c_long, (*f).fd as c_long, iov.as_ptr() as c_long, 2) as c_ulong) as ssize_t
        __syscall_ret(__syscall!(SYS_readv, (*f).fd, iov.as_ptr(), 2) as c_ulong) as ssize_t
    } else {
        // __syscall_ret(__syscall3(SYS_read as c_long, (*f).fd as c_long, iov[1].iov_base as c_long, iov[1].iov_len as c_long) as c_ulong) as ssize_t
        __syscall_ret(__syscall!(SYS_read, (*f).fd, iov[1].iov_base, iov[1].iov_len) as c_ulong) as ssize_t
    };
    if cnt <= 0 {
        (*f).flags |= if cnt!=0 {F_ERR} else {F_EOF};
        return 0;
    }
    if cnt >= 0 && (cnt as size_t) <= iov[0].iov_len { return cnt as size_t; }
    cnt -= iov[0].iov_len as ssize_t;
    (*f).rpos = (*f).buf as *mut c_uchar;
    (*f).rend = ((*f).buf as *mut c_uchar).add(cnt as size_t);
    if (*f).buf_size != 0 {
        buf.add(len-1).write((*f).rpos.read());
        (*f).rpos = (*f).rpos.add(1);
    }
}
    len
}