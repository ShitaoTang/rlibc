use crate::arch::syscall_arch::*;
use crate::include::ctype::*;

#[no_mangle]
pub extern "C" fn clock_gettime(clk: libc::clockid_t, ts: *mut libc::timespec) -> c_int
{
    let mut r: c_int;
    r = unsafe {__syscall2(libc::SYS_clock_gettime, clk as c_long, ts as c_long) as c_int};
    if r == -libc::ENOSYS {
        if clk == libc::CLOCK_REALTIME {
            unsafe {
                __syscall2(libc::SYS_gettimeofday, ts as c_long, 0 as c_long);
                (*ts).tv_nsec = (*ts).tv_nsec * 1000;
            }
            return 0;
        }
        r = -libc::EINVAL;
    }
    if (r as libc::c_ulong) > (-4096i32 as libc::c_ulong) {return -1;}
    r
}