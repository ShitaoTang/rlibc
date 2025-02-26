use libc::{uintptr_t, c_int, c_uchar, c_void, size_t, c_long, c_char, c_ulong, sigset_t};
use core::sync::atomic::{AtomicI32, AtomicU8};
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
    pub dtv:    *mut uintptr_t,
    pub sysinfo:    uintptr_t,
    pub canary:     uintptr_t,

    pub tid:            c_int,
    pub errno_val:      c_int,
    pub detach_state:   AtomicI32,
    pub cancel:         AtomicI32,
    pub canceldisable:  AtomicU8,
    pub cancelasync:    AtomicU8,
    pub tsd_used:       c_uchar,
    pub dlerror_flag:   c_uchar,
    pub map_base:       *mut c_uchar,
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

#[no_mangle]
pub extern "C" fn pthread_create(ret: *mut pthread_t, attrp: *const pthread_attr_t, entry: extern "C" fn(*mut c_void), arg: *mut c_void) -> c_int {
    let mut ret: c_int = 0;
    let mut size: size_t = 0;
    let mut guard: size_t = 0;
    let mut _self: *mut pthread_t = ptr::null_mut();
    let mut new: *mut pthread_t = ptr::null_mut();
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
    
    0
}