use crate::arch::syscall_arch::*;
use crate::include::ctype::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::include::time::*;

#[no_mangle]
pub extern "C" fn clock_gettime(clk: clockid_t, ts: *mut timespec) -> c_int
{
    let mut r: c_int;
    r = unsafe {__syscall2(SYS_clock_gettime as c_long, clk as c_long, ts as c_long) as c_int};
    if r == -ENOSYS {
        if clk == CLOCK_REALTIME {
            unsafe {
                __syscall2(SYS_gettimeofday as c_long, ts as c_long, 0 as c_long);
                (*ts).tv_nsec = (*ts).tv_nsec * 1000;
            }
            return 0;
        }
        r = -EINVAL;
    }
    if (r as c_ulong) > (-4096i32 as c_ulong) {return -1;}
    r
}