use crate::include::ctype::*;
use crate::internal::locale_impl::locale_t;
use super::*;
use crate::arch::syscall_arch::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::internal::futex::*;

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

pub enum DT_STATUS {
    DT_EXITED = 0,
    DT_EXITING,
    DT_JOINABLE,
    DT_DETACHED,
}

pub const DTP_OFFSET: size_t = 0;

#[no_mangle]
#[inline(always)]
pub extern "C" fn wake(addr: *mut c_int, cnt: c_int, lock_priv: c_int) -> ()
{
    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { lock_priv };
    let cnt = if cnt < 0 { c_int::MAX } else { cnt };
    unsafe {
        let _ = __syscall3(SYS_futex as c_long, addr as c_long, 
            (FUTEX_WAKE|lock_priv) as c_long, cnt as c_long) != -ENOSYS as c_long
        || __syscall3(SYS_futex as c_long, addr as c_long, FUTEX_WAKE as c_long, cnt as c_long) != 0;
    };
    
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
#[inline(always)]
pub extern "C" fn TP_ADJ(p: *mut c_void) -> *mut c_void
{
    p
}

#[cfg(target_arch = "aarch64")]
#[no_mangle]
#[inline(always)]
pub extern "C" fn TP_ADJ(p: *mut c_void) -> *mut c_void
{
    (p as *mut c_char as uintptr_t + core::mem::size_of::<pthread>() + TP_OFFSET) as *mut c_void
}

pub const DEFAULT_STACK_SIZE: c_uint = 131072;
pub const DEFAULT_GUARD_SIZE: c_uint = 8192;

pub const __ATTRP_C11_THREAD: *const c_void = usize::MAX as *const c_void;

#[no_mangle]
#[inline(always)]
pub unsafe extern "C" fn __wake(addr: *mut c_void, cnt: c_int, _priv: c_int)
{
    let _priv = if _priv != 0 { FUTEX_PRIVATE } else { _priv };
    let cnt = if cnt < 0 { c_int::MAX } else { cnt };
    let _ = __syscall3(SYS_futex as c_long, addr as c_long, (FUTEX_WAKE|_priv) as c_long, cnt as c_long) != -ENOSYS as c_long
        || __syscall3(SYS_futex as c_long, addr as c_long, FUTEX_WAKE as c_long, cnt as c_long) != 0;
}

#[no_mangle]
#[inline(always)]
pub unsafe extern "C" fn __futexwait(addr: *mut c_void, val: c_int, _priv: c_int)
{
    let _priv = if _priv != 0 { FUTEX_PRIVATE } else { _priv };
    let _ = __syscall4(SYS_futex as c_long, addr as c_long, (FUTEX_WAIT|_priv) as c_long, val as c_long, 0) != -ENOSYS as c_long
        || __syscall4(SYS_futex as c_long, addr as c_long, FUTEX_WAIT as c_long, val as c_long, 0) != 0;
}

pub const SIGTIMER: size_t = 32;
pub const SIGCANCEL: size_t = 33;
pub const SIGSYNCCALL: size_t = 34;