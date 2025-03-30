use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::SYS_mmap;
use crate::internal::syscall_ret::*;
use crate::include::ctype::*;
use crate::include::sys::mman::*;
use crate::internal::syscall::*;
use crate::thread::pthread_self::pthread_self;
use crate::arch::bits::stdint::*;
use crate::thread::vmlock::vm_wait;

pub const UINT: c_ulonglong = SYSCALL_MMAP2_UINT;
pub const OFF_MASK: off_t = (((0x2000 as c_ulonglong).wrapping_neg()
    << (8*core::mem::size_of::<syscall_arg_t>() as c_ulonglong - 1)) | (UINT-1)) as off_t;

#[no_mangle]
pub unsafe extern "C" fn __mmap(
    start: *mut c_void,
    len: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    off: off_t,
) -> *mut c_void {
    let mut ret: c_long;
    if (off & OFF_MASK) != 0 {
        (*pthread_self()).errno_val = EINVAL;
        return MAP_FAILED;
    }
    if len > PTRDIFF_MAX {
        (*pthread_self()).errno_val = EINVAL;
        return MAP_FAILED;
    }
    if ((flags as c_long) & MAP_FIXED) != 0 {
        vm_wait();
    }

    #[cfg(target_arch = "x86_64")]
    {
        ret = __syscall6(
            SYS_mmap as syscall_arg_t,
            start as syscall_arg_t,
            len as syscall_arg_t,
            prot as syscall_arg_t,
            flags as syscall_arg_t,
            fd as syscall_arg_t,
            off as syscall_arg_t,
        );
    }
    #[cfg(target_arch = "aarch64")]
    {
        ret = __syscall6(
            SYS_mmap as syscall_arg_t,
            start as syscall_arg_t,
            len as syscall_arg_t,
            prot as syscall_arg_t,
            flags as syscall_arg_t,
            fd as syscall_arg_t,
            off as syscall_arg_t,
        );
    }
    if ret == -EPERM as c_long && start==core::ptr::null_mut()
        && ((flags as c_long)&MAP_ANNO)!=0 && ((flags as c_long)&MAP_FIXED)==0 {
        ret = -ENOMEM as c_long;
    }

    __syscall_ret(ret as c_ulong) as *mut c_void
}