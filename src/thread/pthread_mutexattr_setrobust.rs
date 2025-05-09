use crate::arch::atomic_arch::a_store;
use crate::__syscall;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::SYS_get_robust_list;
use crate::include::ctype::*;
use crate::arch::generic::bits::errno::EINVAL;
use super::*;

static mut check_robust_result: c_int = 0; // volatile

#[no_mangle]
pub unsafe extern "C" fn pthread_mutexattr_setrobust(a: *mut pthread_mutexattr_t, robust: c_int) -> c_int
{
    if robust as c_uint > 1 { return EINVAL; }
    if robust != 0 {
        let mut r = ptr::read_volatile(ptr::addr_of_mut!(check_robust_result));
        if r < 0 {
            let mut p: *mut c_void = ptr::null_mut();
            let mut l: size_t = 0;
            // r = -__syscall3(SYS_get_robust_list as c_long, 0,
            //     ptr::addr_of_mut!(p) as c_long, ptr::addr_of_mut!(l) as c_long) as c_int;
            r = __syscall!(SYS_get_robust_list, 0,
                ptr::addr_of_mut!(p), ptr::addr_of_mut!(l)
            ) as c_int;
            a_store(ptr::addr_of_mut!(check_robust_result), r);
        }
        if r != 0 { return r; }
        (*a).__attr |= 4;
        return 0;
    }

    (*a).__attr &= !4;
    0
}