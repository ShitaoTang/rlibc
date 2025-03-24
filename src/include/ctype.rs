use core::ptr;
use crate::thread::pthread_impl::pthread;
use crate::internal::stdio_impl::*;
use crate::cfg_if;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;

pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub type c_float = f32;
pub type c_double = f64;

pub type size_t = usize;
pub type ssize_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;

pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type c_void = core::ffi::c_void;

cfg_if! {
    if #[cfg(all(
        not(windows),
        // FIXME(ctest): just use `target_vendor` = "apple"` once `ctest` supports it
        not(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "watchos",
            target_os = "visionos",
        )),
        not(target_os = "vita"),
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "csky",
            target_arch = "hexagon",
            target_arch = "msp430",
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "xtensa",
        )
    ))] {
        pub type c_char = u8;
    } else {
        // On every other target, c_char is signed.
        // c_char is signed on x86_64
        pub type c_char = i8;
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "avr", target_arch = "msp430"))] {
        pub type c_int = i16;
        pub type c_uint = u16;
    } else {
        pub type c_int = i32;
        pub type c_uint = u32;
    }
}

cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        pub type c_long = i64;
        pub type c_ulong = u64;
    } else {
        // The minimal size of `long` in the C standard is 32 bits
        pub type c_long = i32;
        pub type c_ulong = u32;
    }
}

// #[deprecated(since = "0.2.55", note = "Use i8 instead.")]
pub type int8_t = i8;
// #[deprecated(since = "0.2.55", note = "Use i16 instead.")]
pub type int16_t = i16;
// #[deprecated(since = "0.2.55", note = "Use i32 instead.")]
pub type int32_t = i32;
// #[deprecated(since = "0.2.55", note = "Use i64 instead.")]
pub type int64_t = i64;
// #[deprecated(since = "0.2.55", note = "Use u8 instead.")]
pub type uint8_t = u8;
// #[deprecated(since = "0.2.55", note = "Use u16 instead.")]
pub type uint16_t = u16;
// #[deprecated(since = "0.2.55", note = "Use u32 instead.")]
pub type uint32_t = u32;
// #[deprecated(since = "0.2.55", note = "Use u64 instead.")]
pub type uint64_t = u64;

cfg_if! {
    if #[cfg(all(target_arch = "aarch64", not(target_os = "windows")))] {
        pub type __int128 = i128;

        pub type __uint128 = u128;
        
        pub type __int128_t = i128;
        
        pub type __uint128_t = u128;

        // NOTE: if you add more platforms to here, you may need to cfg
        // these consts. They should always match the platform's values
        // for `sizeof(__int128)` and `_Alignof(__int128)`.
        const _SIZE_128: usize = 16;
        const _ALIGN_128: usize = 16;

        // FIXME(ctest): ctest doesn't handle `_` as an identifier so these tests are temporarily
        // disabled.
        // macro_rules! static_assert_eq {
        //     ($a:expr, $b:expr) => {
        //         const _: [(); $a] = [(); $b];
        //     };
        // }
        //
        // // Since Rust doesn't officially guarantee that these types
        // // have compatible ABIs, we const assert that these values have the
        // // known size/align of the target platform's libc. If rustc ever
        // // tries to regress things, it will cause a compilation error.
        // //
        // // This isn't a bullet-proof solution because e.g. it doesn't
        // // catch the fact that llvm and gcc disagree on how x64 __int128
        // // is actually *passed* on the stack (clang underaligns it for
        // // the same reason that rustc *never* properly aligns it).
        // static_assert_eq!(core::mem::size_of::<__int128>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__int128>(), _ALIGN_128);

        // static_assert_eq!(core::mem::size_of::<__uint128>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__uint128>(), _ALIGN_128);

        // static_assert_eq!(core::mem::size_of::<__int128_t>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__int128_t>(), _ALIGN_128);

        // static_assert_eq!(core::mem::size_of::<__uint128_t>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__uint128_t>(), _ALIGN_128);
    }
}

#[repr(C)]
pub struct sigset_t {
    #[cfg(target_pointer_width = "32")]
    __val: [c_ulong; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [c_ulong; 16],
}

pub type syscall_arg_t = c_long;

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

impl pthread_attr_t {
    pub fn _a_stacksize(&self) -> c_ulong {unsafe {self.__u.__s[0]}}
    pub fn _a_guardsize(&self) -> c_ulong {unsafe {self.__u.__s[1]}}
    pub fn _a_stackaddr(&self) -> c_ulong {unsafe {self.__u.__s[2]}}
    pub fn _a_detach(&self) -> c_int { unsafe {self.__u.__i[3*__SU+0]}}
    pub fn _a_sched(&self) -> c_int { unsafe {self.__u.__i[3*__SU+1]}}
    pub fn _a_policy(&self) -> c_int { unsafe {self.__u.__i[3*__SU+2]}}
    pub fn _a_prio(&self) -> c_int { unsafe {self.__u.__i[3*__SU+3]}}
}

// pub const __SU: usize = core::mem::size_of::<size_t>() / core::mem::size_of::<c_int>();
#[cfg(target_pointer_width = "64")]
pub const __SU: usize = 2;
#[cfg(target_pointer_width = "32")]
pub const __SU: usize = 1;

pub const TP_OFFSET: usize = 0;

pub const _NSIG: usize = 65;
#[cfg(target_pointer_width = "64")]
pub const SIGPT_SET_VALUE: [c_ulong; _NSIG/8/8] = [3u64 << 32];
#[cfg(target_pointer_width = "32")]
pub static SIGPT_SET_VALUE: [c_ulong; _NSIG/8/4] = [0, 3u64];
pub const SIGPT_SET: *const sigset_t = SIGPT_SET_VALUE.as_ptr() as *const sigset_t;

#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: ptmu,
}

#[repr(C)]
pub union ptmu {
    #[cfg(target_pointer_width = "64")]
    pub __i: [c_int; 10],
    #[cfg(target_pointer_width = "32")]
    pub __i: [c_int; 6],
    #[cfg(target_pointer_width = "64")]
    pub __vi: [c_int; 10],                  // volatile int
    #[cfg(target_pointer_width = "32")]
    pub __vi: [c_int; 6],                   // volatile int
    #[cfg(target_pointer_width = "64")]
    pub __p: [*mut c_void; 5],              // volatile void *
    #[cfg(target_pointer_width = "32")]
    pub __p: [*mut c_void; 6],              // volatile void *
}

impl pthread_mutex_t {
    pub fn _m_type(&self) -> c_int { unsafe {self.__u.__i[0]}}
    pub fn _m_lock(&self) -> c_int { unsafe {ptr::read_volatile(&self.__u.__vi[1])}}
    pub fn _m_waiters(&self) -> c_int { unsafe {ptr::read_volatile(&self.__u.__vi[2])}}
    pub fn _m_prev(&self) -> *mut c_void {unsafe {ptr::read_volatile(&self.__u.__p[3])}}
    pub fn _m_next(&self) -> *mut c_void {unsafe {ptr::read_volatile(&self.__u.__p[4])}}
    pub fn _m_count(&self) -> c_int { unsafe {self.__u.__i[5]}}
}

#[repr(C)]
pub struct pthread_mutexattr_t {
    pub __attr: c_uint,
}

pub type pthread_spinlock_t = c_int;

#[repr(C)]
pub struct pthread_rwlock_t {
    pub __u: ptrwu,
}

#[repr(C)]
pub union ptrwu {
    #[cfg(target_pointer_width = "64")]
    pub __i: [c_int; 14],
    #[cfg(target_pointer_width = "32")]
    pub __i: [c_int; 8],
    #[cfg(target_pointer_width = "64")]
    pub __vi: [c_int; 14],                  // volatile int
    #[cfg(target_pointer_width = "32")]
    pub __vi: [c_int; 8],                   // volatile int
    #[cfg(target_pointer_width = "64")]
    pub __p: [*mut c_void; 7],              // volatile void *
    #[cfg(target_pointer_width = "32")]
    pub __p: [*mut c_void; 8],              // volatile void *
}

impl pthread_rwlock_t {
    pub fn _rw_lock(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[0])}}
    pub fn _rw_waiters(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[1])}}
    pub fn _rw_shared(&self) -> c_int {unsafe {self.__u.__i[2]}}
}

#[repr(C)]
pub struct pthread_rwlockattr_t {
    pub __attr: [c_uint; 2],
}

#[repr(C)]
pub struct pthread_cond_t {
    pub __u: ptcu,
}

#[repr(C)]
pub union ptcu {
    pub __i: [c_int; 12],
    pub __vi: [c_int; 12],                  // volatile int
    #[cfg(target_pointer_width = "64")]
    pub __p: [*mut c_void; 6],              // volatile void *
    #[cfg(target_pointer_width = "32")]
    pub __p: [*mut c_void; 12],              // volatile void *
}

impl pthread_cond_t {
    pub fn _c_shared(&self) -> *mut c_void {unsafe {ptr::read_volatile(ptr::addr_of!(self.__u.__p[0]))}}
    pub fn _c_seq(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[2])}}
    pub fn _c_waiters(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[3])}}
    pub fn _c_clock(&self) -> c_int {unsafe {self.__u.__i[4]}}
    pub fn _c_lock(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[8])}}
    pub fn _c_head(&self) -> *mut c_void {unsafe {ptr::read_volatile(&self.__u.__p[1])}}
    pub fn _c_tail(&self) -> *mut c_void {unsafe {ptr::read_volatile(&self.__u.__p[5])}}
}

#[repr(C)]
pub struct pthread_condattr_t {
    pub __attr: c_uint,
}

#[repr(C)]
pub struct pthread_barrier_t {
    pub __u: pbtu,
}

#[repr(C)]
pub union pbtu {
    #[cfg(target_pointer_width = "64")]
    pub __i: [c_int; 8],
    #[cfg(target_pointer_width = "32")]
    pub __i: [c_int; 5],
    #[cfg(target_pointer_width = "64")]
    pub __vi: [c_int; 8],                  // volatile int
    #[cfg(target_pointer_width = "32")]
    pub __vi: [c_int; 5],                   // volatile int
    #[cfg(target_pointer_width = "64")]
    pub __p: [*mut c_void; 4],              // volatile void *
    #[cfg(target_pointer_width = "32")]
    pub __p: [*mut c_void; 5],              // volatile void *
}

impl pthread_barrier_t {
    pub fn _b_lock(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[0])}}
    pub fn _b_waiters(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[1])}}
    pub fn _b_limit(&self) -> c_int {unsafe {self.__u.__i[2]}}
    pub fn _b_count(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[3])}}
    pub fn _b_waiters2(&self) -> c_int {unsafe {ptr::read_volatile(&self.__u.__vi[4])}}
    pub fn _b_inst(&self) -> *mut c_void {unsafe {ptr::read_volatile(&self.__u.__p[3])}}
}

#[repr(C)]
pub struct pthread_barrierattr_t {
    pub __attr: c_uint,
}

#[repr(C)]
pub struct __locale_map;

#[repr(C)]
pub struct __locale_struct {
    pub cat: [*const __locale_map; 6],
}

#[allow(non_camel_case_types)]
pub type locale_t = *mut __locale_struct;

pub type clockid_t = c_int;
pub type __time_t = c_long;
pub type time_t = __time_t;
pub type __suseconds_t = c_long;
pub type suseconds_t = __suseconds_t;

pub type off_t = c_long;

pub type pid_t = c_int;

pub type mode_t = c_uint;

pub type FILE = _IO_FILE;