use crate::include::ctype::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::SYS_membarrier;
use crate::include::sys::membarier::*;

pub unsafe fn __membarrier_init()
{
    let _ = __syscall2(SYS_membarrier as c_long,
        MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED as c_long, 0);
}