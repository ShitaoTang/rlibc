use libc::{uintptr_t, c_int, c_uchar, c_void, size_t, c_long, c_char, c_ulong, sigset_t, c_uint};
use core::arch::asm;
use core::sync::atomic::{AtomicI32, AtomicU8, AtomicPtr, Ordering};
use core::option::Option;
use core::ptr;


#[repr(C)]
pub struct __ptcb {
    pub __f: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    pub __x: *mut c_void,
    pub __next: *mut __ptcb,
}

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
    pub unsafe fn set_pending(&mut self, value: *mut c_void) { ptr::write_volatile(&mut self.pending, value);}}

#[repr(C)]
pub struct __locale_map;

#[repr(C)]
pub struct __locale_struct {
    pub cat: [*const __locale_map; 6],
}

#[allow(non_camel_case_types)]
pub type locale_t = *mut __locale_struct;

#[repr(C)]
pub struct pthread {
    pub _self:  *mut pthread,
    pub prev:   *mut pthread,
    pub next:   *mut pthread,
    pub sysinfo:    uintptr_t,

    pub tid:            c_int,
    pub errno_val:      c_int,
    pub detach_state:   c_int,
    pub cancel:         c_int,
    pub canceldisable:  c_uchar,
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

    pub canary:     uintptr_t,
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

#[allow(non_camel_case_types)]
pub type pthread_t = *mut pthread;

#[repr(C)]
pub struct pthread_attr_t {
    pub __u: ptau,
}

#[repr(C)]
pub union ptau {
    #[cfg(target_pointer_width = "64")]
    pub __i: [c_int; 14],
    #[cfg(target_pointer_width = "32")]
    pub __i: [c_int; 9],
    #[cfg(target_pointer_width = "64")]
    pub __vi: [c_int; 14],                  // volatile int
    #[cfg(target_pointer_width = "32")]
    pub __vi: [c_int; 9],                   // volatile int
    #[cfg(target_pointer_width = "64")]
    pub __s: [c_ulong; 7],
    #[cfg(target_pointer_width = "32")]
    pub __s: [c_ulong; 9],
}

// pub const __SU: usize = core::mem::size_of::<size_t>() / core::mem::size_of::<c_int>();
#[cfg(target_pointer_width = "64")]
pub const __SU: usize = 2;
#[cfg(target_pointer_width = "32")]
pub const __SU: usize = 1;

pub const TP_OFFSET: usize = 0;

impl pthread_attr_t {
    pub fn _a_stacksize(&self) -> c_ulong {unsafe {self.__u.__s[0]}}
    pub fn _a_guardsize(&self) -> c_ulong {unsafe {self.__u.__s[1]}}
    pub fn _a_stackaddr(&self) -> c_ulong {unsafe {self.__u.__s[2]}}
    pub fn _a_detach(&self) -> c_int { unsafe {self.__u.__i[3*__SU+0]}}
    pub fn _a_sched(&self) -> c_int { unsafe {self.__u.__i[3*__SU+1]}}
    pub fn _a_policy(&self) -> c_int { unsafe {self.__u.__i[3*__SU+2]}}
    pub fn _a_prio(&self) -> c_int { unsafe {self.__u.__i[3*__SU+3]}}

    pub fn _m_type(&self) -> c_int { unsafe {self.__u.__i[0]}}
    pub fn _m_lock(&self) -> c_int { unsafe {ptr::read_volatile(&self.__u.__vi[1])}}
    pub fn _m_waiters(&self) -> c_int { unsafe {ptr::read_volatile(&self.__u.__vi[2])}}
    pub fn _m_counter(&self) -> c_int { unsafe {self.__u.__i[5]}}

    pub fn _c_seq(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[2])}}
    pub fn _c_waiters(&self) -> c_int { unsafe {ptr::read_volatile(&self.__u.__vi[3])}}
    pub fn _c_clock(&self) -> c_int {unsafe {self.__u.__i[4]}}
    pub fn _c_lock(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[8])}}

    pub fn _rw_lock(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[0])}}
    pub fn _rw_waiters(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[1])}}
    pub fn _rw_shared(&self) -> c_int {unsafe {self.__u.__i[2]}}

    pub fn _b_lock(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[0])}}
    pub fn _b_waiters(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[1])}}
    pub fn _b_limit(&self) -> c_int {unsafe {self.__u.__i[2]}}
    pub fn _b_count(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[3])}}
    pub fn _b_waiters2(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[4])}}
}

// #[repr(C)]
// pub struct _IO_FILE {
//     pub flags: c_uint,
//     pub rpos: *mut c_uchar,
//     pub rend: *mut c_uchar,
//     pub close: Option<extern "C" fn(*mut FILE) -> c_int>,
//     pub wend: *mut c_uchar,
//     pub wpos: *mut c_uchar,
//     pub mustbezero1: *mut c_uchar,
//     pub wbase: *mut c_uchar,
//     pub read: Option<extern "C" fn(*mut FILE, *mut c_uchar, size_t) -> size_t>,
//     pub write: Option<extern "C" fn(*mut FILE, *const c_uchar, size_t) -> size_t>,
//     pub seek: Option<extern "C" fn(*mut FILE, c_long, c_int) -> libc::off_t>,
//     pub buf: *mut c_uchar,
//     pub buf_size: size_t,
//     pub prev: *mut FILE,
//     pub next: *mut FILE,
//     pub fd: c_int,
//     pub pipe_pid: c_int,
//     pub lockcount: c_long,
//     pub mode: c_int,
//     pub lock: c_int,     // volatile
//     pub lbf: c_int,
//     pub cookie: *mut c_void,
//     pub off: libc::off_t,
//     pub getln_buf: *mut c_char,
//     pub mustbezero2: *mut c_void,
//     pub shend: *mut c_uchar,
//     pub shlim: libc::off_t,
//     pub shcnt: libc::off_t,
//     pub prev_locked: *mut FILE,
//     pub next_locked: *mut FILE,
//     pub locale: *mut __locale_struct,
// }

// pub type FILE = _IO_FILE;

// pub static mut ofl_head: *mut FILE = ptr::null_mut();
// pub static mut ofl_lock: [c_int; 1] = [0];

// #[no_mangle]
// pub extern "C" fn __ofl_lock() -> *mut *mut FILE {
//     // LOCK(ofl_lock);
//     unsafe {
//         &ofl_head;
//     }
// }

// #[no_mangle]
// pub extern "C" fn __ofl_unlock() {
//     UNLOCK(ofl_lock);
// }


#[inline]
pub fn __get_up() -> uintptr_t {
    let tp: uintptr_t;
    unsafe {
        asm!("mrs {}, TPIDR_EL0", out(reg) tp);
    }
    tp
}

#[no_mangle]
pub extern "C" fn pthread_self() -> pthread_t {
    (__get_up() - core::mem::size_of::<pthread>() as uintptr_t - TP_OFFSET as uintptr_t) as pthread_t
}

#[no_mangle]
pub extern "C" fn pthread_create(ret: *mut pthread_t, attrp: *const pthread_attr_t, entry: extern "C" fn(*mut c_void), arg: *mut c_void) -> c_int {
    let mut ret: c_int = 0;
    let mut size: size_t = 0;
    let mut guard: size_t = 0;
    let mut _self: pthread_t = ptr::null_mut();
    let mut new: pthread_t = ptr::null_mut();
    let mut map: *mut c_uchar = ptr::null_mut();
    let mut stack: *mut c_uchar = ptr::null_mut();
    let mut tsd: *mut c_uchar = ptr::null_mut();
    let mut stack_limit: *mut c_uchar = ptr::null_mut();
    let flags: u32 = (libc::CLONE_VM | libc::CLONE_FS | libc::CLONE_FILES | libc::CLONE_SIGHAND
         | libc::CLONE_THREAD | libc::CLONE_SYSVSEM | libc::CLONE_SETTLS 
         | libc::CLONE_PARENT_SETTID | libc::CLONE_CHILD_CLEARTID | libc::CLONE_DETACHED) as u32;
    #[cfg(target_pointer_width = "64")]
    let mut attr: pthread_attr_t = pthread_attr_t {__u: ptau {__s: [0; 7]}};
    #[cfg(target_pointer_width = "32")]
    let mut attr: pthread_attr_t = pthread_attr_t {__u: ptau {__s: [0; 9]}};
    let mut set: sigset_t;

    _self = pthread_self();

    
    
    0
}
