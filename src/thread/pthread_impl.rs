use crate::include::ctype::*;
use super::*;
use crate::arch::syscall_arch::*;

#[repr(C)]
pub struct RobustList {
    pub head: *mut c_void,
    pub off: c_long,
    pub pending: *mut c_void,
}

impl RobustList {
    #[inline]
    pub unsafe fn get_head(&self) -> *mut c_void { ptr::read_volatile(&self.head) }
    #[inline]
    pub unsafe fn set_head(&mut self, value: *mut c_void) { ptr::write_volatile(&mut self.head, value); }

    #[inline]
    pub unsafe fn get_pending(&self) -> *mut c_void { ptr::read_volatile(&self.pending) }
    #[inline]
    pub unsafe fn set_pending(&mut self, value: *mut c_void) { ptr::write_volatile(&mut self.pending, value); }
}

#[repr(C)]
pub struct pthread {
    pub _self:  *mut pthread,

    #[cfg(target_arch = "x86_64")]
    pub dtv:            *mut uintptr_t,

    pub prev:           *mut pthread,
    pub next:           *mut pthread,
    pub sysinfo:        uintptr_t,

    #[cfg(target_arch = "x86_64")]
    pub canary:         uintptr_t,

    /* Part 2  */

    pub tid:            c_int,
    pub errno_val:      c_int,
    pub detach_state:   c_int,
    pub cancel:         c_int,
    pub canceldisable:  c_uchar,        // volatile
    pub cancelasync:    c_uchar,
    pub tsd_used:       c_uchar,
    pub dlerror_flag:   c_uchar,
    pub map_base:       *mut c_uchar,
    pub map_size:       size_t,
    pub stack:          *mut c_void,
    pub stack_size:     size_t,
    pub guard_size:     size_t,
    pub result:         *mut c_void,
    pub cancelbuf:      *mut __ptcb,
    pub tsd:            *mut *mut c_void,
    pub robust_list:    RobustList,
    pub h_errno_val:    c_int,
    pub timer_id:       c_int,
    pub locale:         locale_t,
    pub killlock:       [c_int; 1],
    pub dlerror_buf:    *mut c_char,
    pub stdio_locks:    *mut c_void,

    #[cfg(target_arch = "aarch64")]
    pub canary:     uintptr_t,
    #[cfg(target_arch = "aarch64")]
    pub dtv:    *mut uintptr_t,
}

impl pthread {
    pub fn new() -> Self {
        pthread {
            _self: ptr::null_mut(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            dtv: ptr::null_mut(),
            sysinfo: 0,
            canary: 0,

            tid: 0,
            errno_val: 0,
            detach_state: 0,
            cancel: 0,
            canceldisable: 0,
            cancelasync: 0,
            tsd_used: 0,
            dlerror_flag: 0,
            map_base: ptr::null_mut(),
            map_size: 0,
            stack: ptr::null_mut(),
            stack_size: 0,
            guard_size: 0,
            result: ptr::null_mut(),
            cancelbuf: ptr::null_mut(),
            tsd: ptr::null_mut(),
            robust_list: RobustList {
                head: ptr::null_mut(),
                off: 0,
                pending: ptr::null_mut(),
            },
            h_errno_val: 0,
            timer_id: 0,
            locale: ptr::null_mut(),
            killlock: [0],
            dlerror_buf: ptr::null_mut(),
            stdio_locks: ptr::null_mut(),
        }
    }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn wake(addr: *mut c_int, cnt: c_int, lock_priv: c_int) -> ()
{
    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { lock_priv };
    let cnt = if cnt < 0 { libc::INT_MAX } else { cnt };
    unsafe {
        let _ = __syscall3(libc::SYS_futex, addr as c_long, (libc::FUTEX_WAKE|lock_priv) as c_long, cnt as c_long) != -libc::ENOSYS as c_long
        || __syscall3(libc::SYS_futex, addr as c_long, libc::FUTEX_WAKE as c_long, cnt as c_long) != 0;
    };
    
}