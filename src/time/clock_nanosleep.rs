use crate::arch::generic::bits::errno::EINVAL;
use crate::arch::syscall_bits::*;
use crate::include::ctype::*;
use crate::include::time::*;
use crate::thread::pthread_cancel::__syscall_cp_c;

fn _IS_32BIT(x: i64) -> bool
{
    !(((x as u64).wrapping_add(0x80000000)) >> 32 != 0)
}

fn _CLAMP(x: i64) -> i32
{
    if _IS_32BIT(x) { x as i32 }
    else { 0x7fffffff + ((x as u64) >> 63) as i32 }
}

#[no_mangle]
pub unsafe extern "C" fn clock_nanosleep(clk: clockid_t, flags: c_int, req: *const timespec, rem: *mut timespec) -> c_int
{
    if clk == CLOCK_THREAD_CPUTIME_ID { return -EINVAL; }

    if clk==CLOCK_REALTIME && flags==0 {
        return -(__syscall_cp_c(SYS_nanosleep as c_long, req as c_long, rem as c_long, 0, 0, 0, 0) as c_int);
    }

    -(__syscall_cp_c(SYS_clock_nanosleep as c_long, clk as c_long, flags as c_long, req as c_long, rem as c_long, 0, 0)) as c_int
}