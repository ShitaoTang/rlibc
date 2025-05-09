use crate::include::ctype::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::SYS_membarrier;
use crate::__syscall;
use crate::include::sys::membarier::*;

pub unsafe fn __membarrier_init()
{
    // let _ = __syscall2(SYS_membarrier as c_long,
    //     MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED as c_long, 0);
    let _ = __syscall!(SYS_membarrier, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED, 0);
}