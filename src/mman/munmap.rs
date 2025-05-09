use crate::include::ctype::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::SYS_munmap;
use crate::__syscall;
use crate::internal::syscall_ret::__syscall_ret;
use crate::thread::vmlock::vm_wait;

#[no_mangle]
pub unsafe fn __munmap(start: *mut c_void, len: size_t) -> c_int
{
    vm_wait();
    __syscall_ret(__syscall!(
        SYS_munmap as syscall_arg_t,
        start as syscall_arg_t,
        len as syscall_arg_t
    ) as c_ulong) as c_int
}