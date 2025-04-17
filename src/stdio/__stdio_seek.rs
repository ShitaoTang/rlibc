use crate::cfg_if;
use crate::arch::syscall_bits::*;
use crate::arch::syscall_arch::*;
use crate::include::ctype::*;
use crate::internal::syscall_ret::__syscall_ret;

cfg_if! {
    if #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] {
        const def_SYS__llseek: bool = false;
    } else {
        const def_SYS__llseek: bool = true;
    }
}

unsafe fn __lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t
{
    if !def_SYS__llseek {
        __syscall_ret(
            __syscall3(SYS_lseek as c_long, fd as c_long, offset as c_long, whence as c_long) as c_ulong
        ) as off_t
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn __stdio_seek(f: *mut FILE, off: off_t, whence: c_int) -> off_t
{
    unsafe {
        __lseek((*f).fd, off, whence)
    }
}