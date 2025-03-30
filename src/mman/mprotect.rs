use crate::include::ctype::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::SYS_mprotect;
use crate::include::limits::*;
use crate::internal::syscall_ret::__syscall_ret;

#[no_mangle]
pub unsafe fn __mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int
{
    let start: size_t = (addr as size_t) & PAGE_SIZE.wrapping_neg();
    let end: size_t = ((addr as *mut c_char).wrapping_add(len + PAGE_SIZE - 1) as size_t) & PAGE_SIZE.wrapping_neg();

    __syscall_ret(__syscall3(
        SYS_mprotect as syscall_arg_t,
        start as syscall_arg_t,
        end.wrapping_sub(start) as syscall_arg_t,
        prot as syscall_arg_t,
    ) as c_ulong) as c_int

}