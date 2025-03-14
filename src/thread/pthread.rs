use libc::PTHREAD_MUTEX_NORMAL;
use libc::{uintptr_t, c_int, c_uchar, c_void, size_t, c_long, c_char, c_ulong, sigset_t, c_uint};
use core::arch::asm;
use core::ptr;

use crate::arch::syscall_arch::*;
use crate::arch::atomic_arch::*;
use crate::thread::syscall_cp::*;

#[repr(C)]
pub struct __ptcb {
    pub __f: extern "C" fn(*mut c_void) -> *mut c_void,
    pub __x: *mut c_void,
    pub __next: *mut __ptcb,
}

extern "C" fn default_fn(_: *mut c_void) -> *mut c_void {
    ptr::null_mut()
}

impl __ptcb {
    pub fn new() -> Self {
        __ptcb {
            __f: default_fn,
            __x: ptr::null_mut(),
            __next: ptr::null_mut(),
        }
    }
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

pub const _NSIG: usize = 65;
#[cfg(target_pointer_width = "64")]
pub const SIGPT_SET_VALUE: [c_ulong; _NSIG/8/8] = [3u64 << 32];
#[cfg(target_pointer_width = "32")]
pub static SIGPT_SET_VALUE: [c_ulong; _NSIG/8/4] = [0, 3u64];
pub const SIGPT_SET: *const sigset_t = SIGPT_SET_VALUE.as_ptr() as *const sigset_t;

pub const PTHREAD_CANCEL_ENABLE: c_int = 0;
pub const PTHREAD_CANCEL_DISABLE: c_int = 1;
pub const PTHREAD_CANCEL_MASKED: c_int = 2;

pub const PTHREAD_CANCELED: *mut c_void = usize::MAX as *mut c_void;

pub const FUTEX_PRIVATE: c_int = 128;

unsafe extern "C" {
    #[link_name = "__eintr_valid_flag"]
    unsafe static mut __eintr_valid_flag: i32;
}

extern "C" {
    pub static __vmlock_lockptr: *const c_int;
}

pub static mut vmlock: [c_int; 2] = [0; 2];

impl pthread_attr_t {
    pub fn _a_stacksize(&self) -> c_ulong {unsafe {self.__u.__s[0]}}
    pub fn _a_guardsize(&self) -> c_ulong {unsafe {self.__u.__s[1]}}
    pub fn _a_stackaddr(&self) -> c_ulong {unsafe {self.__u.__s[2]}}
    pub fn _a_detach(&self) -> c_int { unsafe {self.__u.__i[3*__SU+0]}}
    pub fn _a_sched(&self) -> c_int { unsafe {self.__u.__i[3*__SU+1]}}
    pub fn _a_policy(&self) -> c_int { unsafe {self.__u.__i[3*__SU+2]}}
    pub fn _a_prio(&self) -> c_int { unsafe {self.__u.__i[3*__SU+3]}}
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


#[inline(always)]
pub fn __get_up() -> uintptr_t
{
    let tp: uintptr_t;
    unsafe {
        asm!("mrs {}, TPIDR_EL0", out(reg) tp);
    }
    tp
}

#[no_mangle]
pub extern "C" fn pthread_self() -> pthread_t
{
    (__get_up() - core::mem::size_of::<pthread>() as uintptr_t - TP_OFFSET as uintptr_t) as pthread_t
}

#[no_mangle]
pub extern "C" fn get_tid(t: pthread_t) -> c_int
{
    unsafe {(*t).tid}
}

pub type syscall_arg_t = c_long;

#[no_mangle]
pub unsafe fn __syscall_cp_c(nr: syscall_arg_t,
                             u: syscall_arg_t, v: syscall_arg_t, w: syscall_arg_t,
                             x: syscall_arg_t, y: syscall_arg_t, z: syscall_arg_t) -> c_long 
{
    let _self: pthread_t = pthread_self();
    let mut r: c_long;
    let st: c_int;

    st = unsafe {(*_self).canceldisable as c_int};
    if st != 0 && (st==PTHREAD_CANCEL_DISABLE || nr==libc::SYS_close) {
        return __syscall6(nr, u, v, w, x, y, z);
    }

    unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    r = __syscall_cp_asm(ptr::addr_of!((*_self).cancel), nr, u, v, w, x, y, z);
    if r==-libc::EINTR as c_long && nr!=libc::SYS_close && (*_self).cancel!=0 && (*_self).canceldisable != PTHREAD_CANCEL_DISABLE as u8{
        r = cancel();
    }

    r
}

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

#[no_mangle]
pub extern "C" fn pthread_mutex_init(m: *mut pthread_mutex_t, a: *const pthread_mutexattr_t) -> c_int
{
    if m.is_null() {
        return -1;
    }

    unsafe {
        ptr::write(m, core::mem::zeroed::<pthread_mutex_t>());
        assert_eq!((*m).__u.__i[0], 0);
        assert_eq!(ptr::read_volatile(&(*m).__u.__vi[1]), 0);
        assert_eq!(ptr::read_volatile(&(*m).__u.__vi[2]), 0);
        assert_eq!(ptr::read_volatile(&(*m).__u.__p[3]), ptr::null_mut());
        assert_eq!(ptr::read_volatile(&(*m).__u.__p[4]), ptr::null_mut());
        assert_eq!((*m).__u.__i[5], 0);

        if !a.is_null() {
            (*m).__u.__i[0] = (*a).__attr as c_int;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn pthread_mutex_lock(m: *mut pthread_mutex_t) -> c_int
{
    if m.is_null() {
        return -1;
    }

    if ((unsafe{(*m)._m_type()} & 15) == libc::PTHREAD_MUTEX_NORMAL)
     && a_cas(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, libc::EBUSY) == 0 {
        return 0;
    }

    pthread_mutex_timedlock(m, 0 as *const libc::timespec)
}

#[no_mangle]
pub extern "C" fn pthread_mutex_timedlock(m: *mut pthread_mutex_t, at: *const libc::timespec) -> c_int
{
    if m.is_null() {
        return -1;
    }

    if ((unsafe{(*m)._m_type()} & 15) == libc::PTHREAD_MUTEX_NORMAL)
     && a_cas(unsafe {ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, libc::EBUSY) == 0 {
        return 0;
    }

    let lock_type = unsafe {(*m)._m_type()};
    let mut r: c_int ;
    let mut t: c_int;
    let lock_priv = (lock_type & 128) ^ 128;
    let _self: pthread_t = pthread_self();

    r = pthread_mutex_trylock(m);
    if r != libc::EBUSY {return r;}

    if lock_type&8 != 0 {return pthread_mutex_timedlock_pi(m, at);}

    let mut spins: c_int = 100;
    while spins != 0 {
        if unsafe { (*m)._m_lock() } == 0 || unsafe { (*m)._m_waiters() } != 0 {
            break;
        }
        a_barrier();
        spins -= 1;
    }

    r = pthread_mutex_trylock(m);
    while r == libc::EBUSY {
        r = unsafe {(*m)._m_lock()};
        let own = r & 0x3fffffff;
        if own == 0 && (r ==0 || lock_type&4 != 0) {
            r = pthread_mutex_trylock(m);
            continue;
        }
        if lock_type &3 == libc::PTHREAD_MUTEX_ERRORCHECK && own == unsafe {(*_self).tid} {return libc::EDEADLK;}

        a_inc(unsafe {ptr::addr_of_mut!((*m).__u.__vi[2])});
        t = r | libc::INT_MIN;
        a_cas(unsafe {ptr::addr_of_mut!((*m).__u.__vi[1])}, r, t);
        r = timedwait(unsafe {ptr::addr_of_mut!((*m).__u.__vi[1])}, t, libc::CLOCK_REALTIME, at, lock_priv);
        a_dec(unsafe {ptr::addr_of_mut!((*m).__u.__vi[2])});
        if r != 0 && r != libc::EINTR {break;}
        r = pthread_mutex_trylock(m);
    }

    r
}

#[no_mangle]
pub extern "C" fn pthread_mutex_trylock(m: *mut pthread_mutex_t) -> c_int
{
    if m.is_null() {
        return -1;
    }

    if (unsafe{(*m)._m_type()} & 15) == libc::PTHREAD_MUTEX_NORMAL{
        return a_cas(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, libc::EBUSY) & libc::EBUSY;
    }

    return pthread_mutex_trylock_owner(m);
}

#[no_mangle]
pub extern "C" fn pthread_mutex_trylock_owner(m: *mut pthread_mutex_t) -> c_int
{
    if m.is_null() {
        return -1;
    }

    let mut old: c_int;
    let own: c_int;
    let lock_type = unsafe {(*m)._m_type()};
    let mut _self: pthread_t = pthread_self();
    let mut tid = unsafe {(*_self).tid};

    old = unsafe {(*m)._m_lock()};
    own = old & 0x3fffffff;
    if own == tid {
        if ((lock_type & 8) != 0) && (unsafe {(*m)._m_count()} < 0) {
            old &= 0x40000000;
            unsafe {(*m).__u.__i[5] = 0};
            return success(m, old, lock_type, ptr::addr_of_mut!(_self));
        }

        if (lock_type & 3) == libc::PTHREAD_MUTEX_RECURSIVE {
            if unsafe{(*m)._m_count() as c_uint >= 0x7fffffff as c_uint} {
                return libc::EAGAIN;
            }
            unsafe {(*m).__u.__i[5] += 1};
            return 0;
        }
    }

    if own == 0x3fffffff {return libc::ENOTRECOVERABLE;}
    if own != 0 || (old != 0 && (lock_type & 4) == 0) {return libc::EBUSY;}

    if (lock_type & 128) != 0 {
        if unsafe {(*_self).robust_list.off == 0} {
            unsafe {
                // (*_self).robust_list.off = ((ptr::addr_of_mut!((*m).__u.__vi[1]) as usize) - (ptr::addr_of_mut!((*m).__u.__p[4]) as usize)) as c_long;
                (*_self).robust_list.off = ((ptr::addr_of!((*m).__u.__vi) as *const i32).add(1) as *const u8).offset_from((ptr::addr_of!((*m).__u.__p) as *const *mut u8).add(4) as *const u8) as c_long;
                __syscall2(libc::SYS_set_robust_list, ptr::addr_of_mut!((*_self).robust_list) as c_long, (3* core::mem::size_of::<c_long>()) as c_long);
            }
        }
        if unsafe {(*m)._m_waiters()} != 0 {tid |= libc::INT_MIN;}
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), &mut (*m).__u.__p[4] as *mut _ as *mut c_void)};
    }
    tid |= old & 0x40000000;

    if a_cas(unsafe{ ptr::addr_of_mut!((*m).__u.__vi[1]) }, old, tid) != old {
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::null_mut())};
        if (lock_type & 12) == 12 && unsafe {(*m)._m_waiters() != 0} { return libc::ENOTRECOVERABLE; }
        return libc::EBUSY;
    }

    success(m, old, lock_type, ptr::addr_of_mut!(_self));

    0
}

fn success(m: *mut pthread_mutex_t, old: c_int, lock_type: c_int, _self: *mut pthread_t) -> c_int
{
    if (lock_type & 8) != 0 && unsafe { (*m)._m_waiters() } != 0 {
        let priv_flag = (lock_type & 128) ^ 128;
        unsafe {__syscall2(libc::SYS_futex, ptr::addr_of!((*m).__u.__vi[1]) as c_long, (libc::FUTEX_UNLOCK_PI | priv_flag) as c_long);}
        unsafe { ptr::write_volatile(ptr::addr_of_mut!((*(*_self)).robust_list.pending), ptr::null_mut()); }
        return if (lock_type & 4) != 0 {
            libc::ENOTRECOVERABLE
        } else {
            libc::EBUSY
        };
    }

    let next: *mut c_void = unsafe { ptr::read_volatile(ptr::addr_of_mut!((*(*_self)).robust_list.head)) };    // volatile
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!((*m).__u.__p[4]), ptr::read_volatile(ptr::addr_of!(next)));
        ptr::write_volatile(ptr::addr_of_mut!((*m).__u.__p[3]), ptr::read_volatile(ptr::addr_of_mut!((*(*_self)).robust_list.head)));
    }
    if next != unsafe { ptr::read_volatile(ptr::addr_of_mut!((*(*_self)).robust_list.head)) } {
        unsafe {
            *((next as *mut u8).offset(-(core::mem::size_of::<*mut c_void>() as isize))
            as *mut *mut c_void) = ptr::read_volatile(ptr::addr_of_mut!((*m).__u.__p[4]));
        }
    }
    unsafe { ptr::write_volatile(ptr::addr_of_mut!((*(*_self)).robust_list.head), (*m)._m_next()) };
    unsafe { ptr::write_volatile(ptr::addr_of_mut!((*(*_self)).robust_list.pending), ptr::null_mut()) };

    if old != 0 {
        unsafe { (*m).__u.__i[5] = 0 };
        return libc::EOWNERDEAD;
    }

    0
}

pub extern "C" fn futex4(addr: *mut c_void, op: c_int, val: c_int, to: *const libc::timespec) -> c_int
{
    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    let res = unsafe {__syscall4(libc::SYS_futex, addr as c_long, op as c_long, val as c_long, to as c_long)};
    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    res as c_int
}

pub extern "C" fn timedwait(addr: *mut c_int, val: c_int, clk: libc::clockid_t, at: *const libc::timespec, lock_priv: c_int) -> c_int
{
    let mut cs: c_int = 0;
    let r: c_int;

    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, &mut cs);
    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    r = timedwait_cp(addr, val, clk, at, lock_priv);
    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    pthread_setcancelstate(cs, ptr::null_mut());
    r
}

#[no_mangle]
pub extern "C" fn pthread_setcancelstate(new: c_int, old: *mut c_int) -> c_int
{
    if new as c_uint > 2u32 {return libc::EINVAL;}   // trick, only when 0<=new<=2, it's valid (negatives are invalid)
    let mut _self: pthread_t = pthread_self();
    unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    if old != ptr::null_mut() { unsafe  {*old = ptr::read_volatile(ptr::addr_of_mut!((*_self).canceldisable)) as c_int};}
    unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).canceldisable), new as c_uchar)};
    0
}

#[no_mangle]
pub extern "C" fn timedwait_cp(addr: *mut c_int, val: c_int, clk: libc::clockid_t, at: *const libc::timespec, lock_priv: c_int) -> c_int
{
    let mut r: c_int;
    let mut to: libc::timespec = libc::timespec {tv_sec: 0, tv_nsec: 0};
    let mut top: *mut libc::timespec = ptr::null_mut();

    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { 0 };

    if at != ptr::null_mut() {
        if unsafe {(*at).tv_nsec} as u64 > 1000000000u64 {return libc::EINVAL;}   
        if clock_gettime(clk, &mut to) != 0 {return libc::EINVAL;}
        to.tv_sec = unsafe {(*at).tv_sec} - to.tv_sec;
        to.tv_nsec = unsafe {(*at).tv_nsec} - to.tv_nsec;
        if to.tv_nsec < 0 {
            to.tv_sec -= 1;
            to.tv_nsec += 1000000000;
        }
        if to.tv_sec < 0 {return libc::ETIMEDOUT;}
        top = &mut to;
    }

    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    r = -futex4_cp(addr as *mut c_void, libc::FUTEX_WAIT|lock_priv, val, top);

    if r != libc::EINTR && r!= libc::ETIMEDOUT && r != libc::ECANCELED {r = 0;}
    if r == libc::EINTR && unsafe {ptr::read_volatile(ptr::addr_of!(__eintr_valid_flag))} == 0 {r = 0;}

    r
}

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

#[no_mangle]
pub extern "C" fn futex4_cp(addr: *mut c_void, op: c_int, val: c_int, to: *const libc::timespec) -> c_int
{
    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    let r: c_int = unsafe {
        __syscall6(libc::SYS_futex, addr as c_long, op as c_long, val as c_long, to as c_long, 0 as c_long, 0 as c_long) as c_int
    };
    if r != -libc::ENOSYS {return r;}
    let tmp = (op as c_int) & !(FUTEX_PRIVATE as c_int);
    unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    unsafe {__syscall6(libc::SYS_futex, addr as c_long, tmp as c_long, val as c_long, to as c_long, 0 as c_long, 0 as c_long) as c_int}
}

#[no_mangle]
pub extern "C" fn pthread_mutex_timedlock_pi(m: *mut pthread_mutex_t, at: *const libc::timespec) -> c_int
{
    let lock_type = unsafe {(*m)._m_type()};
    let lock_priv = (lock_type & 128) ^ 128;
    let mut _self: pthread_t = pthread_self();
    let mut e: c_int;

    unsafe{if m.is_null() { asm!("brk #0", options(noreturn)); }}
    if lock_priv == 0 {
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::read_volatile(ptr::addr_of_mut!((*m).__u.__p[4])))};
    }

    loop {
        e = -futex4(unsafe{ptr::addr_of!((*m).__u.__vi[1]) as *mut i32 as *mut c_void}, libc::FUTEX_LOCK_PI | lock_priv, 0, at);
        if e != libc::EINTR {break;}
    }
    if e != 0 {
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::null_mut())};
    }

    'block:{
        match e {
            0 => {
                if (lock_type&4 == 0) && (unsafe{((*m)._m_lock() & 0x40000000 != 0) || (*m)._m_waiters() != 0}) {
                    a_store(unsafe{ptr::addr_of_mut!((*m).__u.__vi[2])}, -1);
                    unsafe {__syscall2(libc::SYS_futex, ptr::addr_of_mut!((*m).__u.__vi[1]) as *mut _ as c_long, (libc::FUTEX_UNLOCK_PI | lock_priv) as c_long);}
                    unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::null_mut())};
                    break 'block;
                }
                unsafe {(*m).__u.__i[5] = -1};
                return pthread_mutex_trylock(m);
            }
            libc::ETIMEDOUT => {return e}
            libc::EDEADLK => {
                if lock_type&3 == libc::PTHREAD_MUTEX_ERRORCHECK {return e}
            }
            _ => {}
        }
        loop {
            e = timedwait(ptr::null_mut(), 0, libc::CLOCK_REALTIME, at, 1);
            if e == libc::ETIMEDOUT {break;}
        }
    }

    e
}

#[no_mangle]
pub extern "C" fn pthread_mutex_unlock(m: *mut pthread_mutex_t) -> c_int
{
    let mut _self: pthread_t = pthread_self();
    let mut waiters: c_int = unsafe {(*m)._m_waiters()};
    let cont: c_int;
    let lock_type: c_int = unsafe {(*m)._m_type()} & 15;
    let lock_priv: c_int = (unsafe {(*m)._m_type()} & 128) ^ 128;
    let mut new: c_int = 0;
    let mut old: c_int = 0;

    if lock_type != libc::PTHREAD_MUTEX_NORMAL {
        old = unsafe {(*m)._m_lock()};
        let own = old & 0x3fffffff;
        if own != unsafe {(*_self).tid} { return libc::EPERM; }
        if lock_type&3 == libc::PTHREAD_MUTEX_RECURSIVE && unsafe {(*m)._m_count()} != 0 {
            unsafe {(*m).__u.__i[5] -= 1;}
            return 0;
        }
        if lock_type&4 != 0 && (old&0x40000000) != 0 {new = 0x7fffffff;}
        if lock_priv == 0 {
            unsafe{if m.is_null() { asm!("brk #0", options(noreturn)); }}
            unsafe {
                ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.head), ptr::read_volatile(ptr::addr_of_mut!((*m).__u.__p[4])));
                vm_lock();
            }
        }
        let mut prev: *mut c_void = unsafe {(*m).__u.__p[3]};    
        let next: *mut c_void = unsafe {(*m).__u.__p[4]};
        unsafe {
            ptr::write_volatile(ptr::addr_of_mut!(prev), next);
        }

        unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
        unsafe{if next.is_null() { asm!("brk #0", options(noreturn)); }}
        unsafe{if prev.is_null() { asm!("brk #0", options(noreturn)); }}
        if next != unsafe { ptr::read_volatile(ptr::addr_of_mut!((*_self).robust_list.head)) } {
            unsafe {
                *((next as *mut u8).offset(-(core::mem::size_of::<*mut c_void>() as isize))
                as *mut *mut c_void) = ptr::read_volatile(ptr::addr_of_mut!(prev));
            }
        }
    }

    if lock_type&8 != 0 {
        if old<0 || a_cas(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, old, new) != old {
            if new != 0 {a_store(unsafe{ptr::addr_of_mut!((*m).__u.__vi[2])}, -1);}
            unsafe {__syscall2(libc::SYS_futex, ptr::addr_of_mut!((*m).__u.__vi[1]) as c_long, (libc::FUTEX_UNLOCK_PI | lock_priv) as c_long);}
        }
        cont = 0;
        waiters = 0;
    } else {
        cont = a_swap(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, new);
    }
    if lock_priv != PTHREAD_MUTEX_NORMAL && lock_priv == 0 {
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::null_mut())};
        unsafe {vm_unlock()};
    }
    if waiters != 0 || cont < 0 {
        wake(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, 1, lock_priv);
    }

    0
} 

#[no_mangle]
pub unsafe extern "C" fn vm_wait() -> ()
{
    let mut tmp = vmlock[0];
    loop {
        wait(ptr::addr_of_mut!(vmlock) as *mut c_int, (ptr::addr_of_mut!(vmlock) as *mut c_int).add(1), tmp, 1);
        tmp = vmlock[0];
        if tmp == 0 {break;}
    }
}

#[no_mangle]
pub unsafe extern "C" fn vm_lock() -> ()
{
    a_inc(ptr::addr_of_mut!(vmlock) as *mut c_int);
}

#[no_mangle]
pub unsafe extern "C" fn vm_unlock() -> ()
{
    if a_fetch_add(ptr::addr_of_mut!(vmlock) as *mut c_int, -1) == 1 && vmlock[1] != 0{
        wake(ptr::addr_of!(vmlock) as *mut c_int, -1, 1);
    }
}

#[no_mangle]
pub extern "C" fn wait(addr: *mut c_int, waiters: *mut c_int, val: c_int, lock_priv: c_int) -> ()
{
    let mut spins: c_int = 100;
    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { lock_priv };
    while spins != 0 && (waiters.is_null() || unsafe {ptr::read_volatile(waiters)} == 0) {
        spins -= 1;
        // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
        if unsafe {ptr::read_volatile(addr)} == val {
            a_spin();
        } else {
            return;
        }
    }
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    if !waiters.is_null() {
        a_inc(waiters);
    }
    while unsafe {ptr::read_volatile(addr)} == val {
        unsafe {
            let _ = __syscall4(libc::SYS_futex, addr as c_long, (libc::FUTEX_WAIT|lock_priv) as c_long, val as c_long, 0 as c_long) != -libc::ENOSYS as c_long
            || __syscall4(libc::SYS_futex, addr as c_long, libc::FUTEX_WAIT as c_long, val as c_long, 0 as c_long) != 0;
        }
    }
    if !waiters.is_null(){
        a_dec(waiters);
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

type pthread_spinlock_t = c_int;

#[no_mangle]
pub extern "C" fn pthread_spin_init(s: *mut pthread_spinlock_t, _pshared: c_int) -> c_int
{
    unsafe {*s = 0;}
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_lock(s: *mut pthread_spinlock_t) -> c_int
{
    unsafe {
        while ptr::read_volatile(s) != 0 || a_cas(s, 0, libc::EBUSY) != 0 {
            a_barrier();
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_trylock(s: *mut pthread_spinlock_t) -> c_int
{
    a_cas(s, 0, libc::EBUSY);
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_unlock(s: *mut pthread_spinlock_t) -> c_int
{
    a_store(s, 0);
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_destroy(_s: *mut pthread_spinlock_t) -> c_int
{
    0
}

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

#[no_mangle]
pub extern "C" fn pthread_rwlock_init(rw: *mut pthread_rwlock_t, a: *const pthread_rwlockattr_t) -> c_int
{
    if rw.is_null() {return -1;}

    unsafe {
        ptr::write(rw, core::mem::zeroed::<pthread_rwlock_t>());
        assert_eq!(ptr::read_volatile(&(*rw).__u.__vi[0]), 0);
        assert_eq!(ptr::read_volatile(&(*rw).__u.__vi[1]), 0);
        assert_eq!((*rw).__u.__i[2], 0);
    }

    if !a.is_null() {
        unsafe {(*rw).__u.__i[2] = ((*a).__attr[0] * 128) as c_int;}
    }

    0
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_rdlock(rw: *mut pthread_rwlock_t) -> c_int
{
    pthread_rwlock_timedrdlock(rw, 0 as *const libc::timespec)
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_timedrdlock(rw: *mut pthread_rwlock_t, at: *const libc::timespec) -> c_int
{
    let mut r: c_int = pthread_rwlock_tryrdlock(rw);
    if r != libc::EBUSY {return r;}

    let mut spins: c_int = 100;
    while spins != 0 {
        if unsafe {(*rw)._rw_lock()} == 0 || unsafe {(*rw)._rw_waiters()} != 0 {
            break;
        }
        a_barrier();
        spins -= 1;
    }

    r = pthread_rwlock_tryrdlock(rw);
    while r == libc::EBUSY {
        r = unsafe {(*rw)._rw_lock()};
        if r == 0 || (r & 0x7fffffff)!=0x7fffffff {
            r = pthread_rwlock_tryrdlock(rw);
            continue;
        }
        let t = r | libc::INT_MIN;
        a_inc(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[1])});
        a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, r, t);
        r = timedwait(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, t, libc::CLOCK_REALTIME, at, unsafe{(*rw)._rw_shared()}^128);
        a_dec(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[1])});
        if r != 0 && r != libc::EINTR {return r;} 
        r = pthread_rwlock_tryrdlock(rw);
    }

    r
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_tryrdlock(rw: *mut pthread_rwlock_t) -> c_int
{
    let mut val: c_int;
    let mut cnt: c_int;
    loop {
        val = unsafe {(*rw)._rw_lock()};
        cnt = val & 0x7fffffff;
        if cnt == 0x7fffffff {return libc::EBUSY;}
        if cnt == 0x7ffffffe {return libc::EAGAIN;}
        if a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, val, val+1) == val {break;}
    }
    0
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_wrlock(rw: *mut pthread_rwlock_t) -> c_int
{
    pthread_rwlock_timedwrlock(rw, 0 as *const libc::timespec)
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_timedwrlock(rw: *mut pthread_rwlock_t, at: *const libc::timespec) -> c_int
{
    let mut r: c_int = pthread_rwlock_trywrlock(rw);
    if r != libc::EBUSY {return r;}

    let mut spins: c_int = 100;
    while spins != 0 {
        if unsafe {(*rw)._rw_lock()} == 0 || unsafe {(*rw)._rw_waiters()} != 0 {
            break;
        }
        a_barrier();
        spins -= 1;
    }

    r = pthread_rwlock_trywrlock(rw);
    while r == libc::EBUSY {
        r = unsafe {(*rw)._rw_lock()};
        if r == 0 {
            r = pthread_rwlock_trywrlock(rw);
            continue;
        }
        let t = r | libc::INT_MIN;
        a_inc(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[1])});
        a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, r, t);
        r = timedwait(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, t, libc::CLOCK_REALTIME, at, unsafe{(*rw)._rw_shared()}^128);
        a_dec(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[1])});
        if r != 0 && r != libc::EINTR {return r;}
        r = pthread_rwlock_trywrlock(rw);
    }

    r
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_trywrlock(rw: *mut pthread_rwlock_t) -> c_int
{
    if a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, 0, 0x7fffffff) != 0 {return libc::EBUSY;}
    0
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_unlock(rw: *mut pthread_rwlock_t) -> c_int
{
    let mut val: c_int;
    let mut cnt: c_int;
    let mut waiters: c_int;
    let mut new: c_int;
    let lock_priv: c_int = unsafe{(*rw)._rw_shared()} ^ 128;

    loop {
        val = unsafe {(*rw)._rw_lock()};
        cnt = val & 0x7fffffff;
        waiters = unsafe {(*rw)._rw_waiters()};
        if cnt == 0x7fffffff || cnt == 1 {
            new = 0;
        } else {
            new = val - 1;
        }
        if a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, val, new) == val {break;}
    }

    if new == 0 && (waiters != 0 || val < 0) {
        wake(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, cnt, lock_priv);
    }

    0
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

#[no_mangle]
pub extern "C" fn pthread_cond_init(c: *mut pthread_cond_t, a: *const pthread_condattr_t) -> c_int
{
    unsafe {
        ptr::write(c, core::mem::zeroed::<pthread_cond_t>());
        assert_eq!(ptr::read_volatile(&(*c).__u.__p[0]), ptr::null_mut());
        assert_eq!(ptr::read_volatile(&(*c).__u.__vi[2]), 0);
        assert_eq!(ptr::read_volatile(&(*c).__u.__vi[3]), 0);
        assert_eq!((*c).__u.__i[4], 0);
        assert_eq!(ptr::read_volatile(&(*c).__u.__vi[8]), 0);
        assert_eq!(ptr::read_volatile(&(*c).__u.__p[1]), ptr::null_mut());
        assert_eq!(ptr::read_volatile(&(*c).__u.__p[5]), ptr::null_mut());

        if !a.is_null() {
            (*c).__u.__i[4] = ((*a).__attr & 0x7fffffff) as c_int;
            if (*a).__attr >> 31 != 0 {
                ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[0]), usize::MAX as *mut c_void);
            }
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn pthread_cond_wait(c: *mut pthread_cond_t, m: *mut pthread_mutex_t) -> c_int
{
    pthread_cond_timedwait(c, m, 0 as *const libc::timespec)
}

#[repr(C)]
pub struct waiter {
    next: *mut waiter,
    prev: *mut waiter,
    state: c_int,           // volatile
    barrier: c_int,         // volatile
    notify: *mut c_int,     // volatile
}

impl waiter {
    pub fn new() -> Self {
        Self {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            state: 0,
            barrier: 0,
            notify: ptr::null_mut(),
        }
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn _lock(l: *mut c_int) -> ()
{
    if a_cas(l, 0, 1) != 0 {
        a_cas(l, 1, 2);
        loop {
            wait(l, ptr::null_mut(), 2, 1);
            if a_cas(l, 0, 2) == 0 {break;}
        }
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn _unlock(l: *mut c_int) -> ()
{
    if a_swap(l, 0) == 2 {
        wake(l, 1, 1);
    }
}

#[inline(always)]
#[no_mangle]
pub extern "C" fn unlock_requeue(l: *mut c_int, r: *mut c_int, w: c_int) -> ()
{
    a_store(l, 0);
    if w != 0 {
        wake(l, 1, 1);
    } else {
        unsafe {
            let _ = 
            __syscall5(libc::SYS_futex, l as c_long, (libc::FUTEX_REQUEUE | FUTEX_PRIVATE) as c_long, 0 as c_long, 1 as c_long, r as c_long) != libc::ENOSYS as c_long
            || __syscall5(libc::SYS_futex, l as c_long, libc::FUTEX_REQUEUE as c_long, 0 as c_long, 1 as c_long, r as c_long) != 0;
        };
    }
}

#[repr(C)]
pub enum THREAD_STATE {
    WAITING,
    SIGNALED,
    LEAVING,
}

#[no_mangle]
pub extern "C" fn pthread_cond_timedwait(c: *mut pthread_cond_t, m: *mut pthread_mutex_t, ts: *const libc::timespec) -> c_int
{
    let mut node: waiter = waiter::new();
    let mut e: c_int;
    let seq: c_int;
    let clock: c_int = unsafe{(*c)._c_clock()};
    let mut cs: c_int = 0;
    let mut shared: c_int = 0;
    let oldstate: c_int;
    let mut tmp:c_int = 0;
    let fut: *mut c_int;    // volatile
    let _self: pthread_t = pthread_self();

    if m.is_null() {
        return libc::EINVAL;
    }

    if unsafe{(*m)._m_type()} & 15 != 0 && unsafe{(*m)._m_lock()&libc::INT_MAX != (*_self).tid} {return libc::EPERM;}

    if !ts.is_null() && (unsafe{(*ts).tv_nsec} as u64 >= 1000000000u64) {return libc::EINVAL;}

    pthread_testcancel();

    if unsafe{(*c)._c_shared()} != ptr::null_mut() {
        shared = 1;
        fut = unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])};
        seq = unsafe{(*c)._c_seq()};
        a_inc(unsafe{ptr::addr_of_mut!((*c).__u.__vi[3])});
    } else {
        _lock(unsafe{ptr::addr_of_mut!((*c).__u.__vi[8])});

        node.barrier = 2;
        seq = 2;
        fut = ptr::addr_of_mut!(node.barrier);
        node.state = THREAD_STATE::WAITING as c_int;
        node.next = unsafe{(*c)._c_head()} as *mut waiter;
        unsafe{ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[1]), ptr::addr_of_mut!(node) as *mut c_void);}
        unsafe {
            if (*c)._c_tail() == ptr::null_mut() {
                ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[5]), ptr::addr_of_mut!(node) as *mut c_void);
            } else {
                (*node.next).prev = ptr::addr_of_mut!(node);
            }
        }
        _unlock(unsafe{ptr::addr_of_mut!((*c).__u.__vi[8])});
    }

    pthread_mutex_unlock(m);

    pthread_setcancelstate(PTHREAD_CANCEL_MASKED, ptr::addr_of_mut!(cs));
    if cs == PTHREAD_CANCEL_DISABLE {
        pthread_setcancelstate(cs, ptr::null_mut());
    }

    loop {
        e = timedwait_cp(fut, seq, clock, ts, (shared==0) as c_int);
        if !(unsafe{*(fut)}==seq && (e==0 || e==libc::EINTR)) {
            break;
        }
    }
    if e == libc::EINTR {
        e = 0;
    }

    if shared != 0 {
        if e == libc::ECANCELED && unsafe{(*c)._c_seq() != seq} {e = 0;}
        if a_fetch_add(unsafe{ptr::addr_of_mut!((*c).__u.__vi[3])}, -1) == -0x7fffffff {
            wake(unsafe{ptr::addr_of_mut!((*c).__u.__vi[3])}, 1, 0);
        }
        oldstate = THREAD_STATE::WAITING as c_int;
        return relock(&mut e, m, &mut tmp, oldstate, ptr::addr_of_mut!(node), cs);
    }

    oldstate = a_cas(ptr::addr_of_mut!(node.state), THREAD_STATE::WAITING as c_int, THREAD_STATE::LEAVING as c_int);

    if oldstate == THREAD_STATE::WAITING as c_int {
        _lock(unsafe{ptr::addr_of_mut!((*c).__u.__vi[8])});

        if unsafe{(*c)._c_head()} as *mut waiter == ptr::addr_of_mut!(node) {
            unsafe{ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[1]), node.next as *mut c_void);}
        } else if node.prev != ptr::null_mut() {
            unsafe{(*node.prev).next = node.next;}
        }

        if unsafe{(*c)._c_tail()} as *mut waiter == ptr::addr_of_mut!(node) {
            unsafe{ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[5]), node.prev as *mut c_void);}
        } else if node.next != ptr::null_mut() {
            unsafe{(*node.next).prev = node.prev;}
        }

        _unlock(unsafe{ptr::addr_of_mut!((*c).__u.__vi[8])});

        if node.notify != ptr::null_mut() {
            if a_fetch_add(node.notify, -1) == 1 {
                wake(node.notify, 1, 1);
            }
        }
    } else {
        _lock(ptr::addr_of_mut!(node.barrier));
    }

    relock(&mut e, m, &mut tmp, oldstate, ptr::addr_of_mut!(node), cs)
}

fn relock(e: &mut c_int, m: *mut pthread_mutex_t , tmp: &mut c_int, oldstate: c_int, node: *mut waiter, cs: c_int) -> c_int
{
    *tmp = pthread_mutex_lock(m);
    if *tmp != 0 {*e = *tmp;}

    if oldstate == THREAD_STATE::WAITING as c_int {done(*e, cs);}

    if unsafe{(*node).next == ptr::null_mut()} && unsafe{(*m)._m_type()&8 == 0} {
        a_inc(unsafe{ptr::addr_of_mut!((*m).__u.__vi[2])});
    }

    unsafe {
        if (*node).prev != ptr::null_mut() {
            let val = (*m)._m_lock();
            if val > 0 {
                a_cas(ptr::addr_of_mut!((*m).__u.__vi[1]), val, val | libc::INT_MIN);
            }
            unlock_requeue(ptr::addr_of_mut!((*(*node).prev).barrier), ptr::addr_of_mut!((*m).__u.__vi[1]), (*m)._m_type()&(8|128));
        } else if ((*m)._m_type() & 8) == 0 {
            a_dec(ptr::addr_of_mut!((*m).__u.__vi[2]));
        }
    }

    if *e == libc::ECANCELED {*e = 0;}

    done(*e, cs)
}

fn done(e: c_int, cs: c_int) -> c_int
{
    pthread_setcancelstate(cs, ptr::null_mut());

    if e == libc::ECANCELED {
        pthread_testcancel();
        pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, ptr::null_mut());
    }

    e
}

#[no_mangle]
pub extern "C" fn pthread_testcancel() -> ()
{
    testcancel();
}

#[no_mangle]
pub extern "C" fn testcancel() -> ()
{
    let _self: pthread_t = pthread_self();
    unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    if unsafe{ptr::read_volatile(ptr::addr_of_mut!((*_self).cancel))} != 0 && unsafe{ptr::read_volatile(ptr::addr_of!((*_self).canceldisable))} == 0 {
        cancel();
    } 
}

#[no_mangle]
pub extern "C" fn cancel() -> c_long
{
    let _self: pthread_t = pthread_self();
    unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
    unsafe {
        if {ptr::read_volatile(ptr::addr_of!((*_self).canceldisable))} == PTHREAD_CANCEL_ENABLE as u8
        || {ptr::read_volatile(ptr::addr_of!((*_self).cancelasync))} != 0 {
            libc::pthread_exit(PTHREAD_CANCELED);
        }
    }
    unsafe{ptr::write_volatile(ptr::addr_of_mut!((*_self).canceldisable), PTHREAD_CANCEL_DISABLE as u8);}
    
    -libc::ECANCELED as c_long
}

#[no_mangle]
pub extern "C" fn private_cond_signal(c: *mut pthread_cond_t, n: c_int) -> c_int
{
    let mut p: *mut waiter;
    let mut first: *mut waiter = ptr::null_mut();
    let mut _ref: c_int = 0;     // volatile
    let mut cur: c_int;
    let mut n = n;

    _lock(unsafe{ptr::addr_of_mut!((*c).__u.__vi[8])});
    unsafe{p = (*c)._c_tail() as *mut waiter};
    while n!=0 && p != ptr::null_mut() {
        if (a_cas(unsafe{ptr::addr_of_mut!((*p).state)}, THREAD_STATE::WAITING as c_int, THREAD_STATE::SIGNALED as c_int)) != THREAD_STATE::WAITING as c_int {
            unsafe { ptr::write_volatile(ptr::addr_of_mut!(_ref), ptr::read_volatile(&_ref) + 1); } // volatile +1
            unsafe {(*p).notify = ptr::addr_of_mut!(_ref);}
        } else {
            n -= 1;
            if first == ptr::null_mut() {first = p;}
        }
        p = unsafe{(*p).prev};
    }

    if p != ptr::null_mut() {
        if unsafe{(*p).next != ptr::null_mut()} {
            unsafe{(*(*p).next).prev = ptr::null_mut()};
        } 
        unsafe{(*p).next = ptr::null_mut()};
    } else {
        unsafe{ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[1]), ptr::null_mut())};
    }
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!((*c).__u.__p[5]), ptr::null_mut());
    }
    _unlock(unsafe{ptr::addr_of_mut!((*c).__u.__vi[8])});

    cur = _ref;
    loop {
        if cur == 0 {break;}
        wait(ptr::addr_of_mut!(cur), ptr::null_mut(), cur, 1);
        cur = _ref;
    }

    if first != ptr::null_mut() {
        unsafe{_unlock(ptr::addr_of_mut!((*first).barrier))};
    }

    0
}

#[no_mangle]
pub extern "C" fn pthread_cond_signal(c: *mut pthread_cond_t) -> c_int
{
    if unsafe{(*c)._c_shared()} == ptr::null_mut() {
        return private_cond_signal(c, 1);
    }
    if unsafe{(*c)._c_waiters()} == 0 {return 0;}
    a_inc(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])});
    wake(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])}, 1, 0);

    0
}

#[no_mangle]
pub extern "C" fn pthread_cond_broadcast(c: *mut pthread_cond_t) -> c_int
{
    if unsafe{(*c)._c_shared()} == ptr::null_mut() {
        return private_cond_signal(c, -1 as c_int);
    }
    if unsafe{(*c)._c_waiters()} == 0 {return 0;}
    a_inc(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])});
    wake(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])}, -1 as c_int, 0);

    0
}

#[no_mangle]
pub extern "C" fn pthread_cond_destory(c: *mut pthread_cond_t) -> c_int
{
    if unsafe{(*c)._c_shared()} != ptr::null_mut() && unsafe{(*c)._c_waiters()} != 0 {
        let mut cnt: c_int;
        a_or(unsafe{ptr::addr_of_mut!((*c).__u.__vi[3])}, libc::INT_MIN);
        a_inc(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])});
        wake(unsafe{ptr::addr_of_mut!((*c).__u.__vi[2])}, -1 as c_int, 0);
        cnt = unsafe{(*c)._c_waiters()};
        loop {
            if (cnt&0x7fffffff) == 0 {break;}
            wait(unsafe{ptr::addr_of_mut!((*c).__u.__vi[3])}, ptr::null_mut(), cnt, 0);
            cnt = unsafe{(*c)._c_waiters()};
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn pthread_condattr_init(a: *mut pthread_condattr_t) -> c_int
{
    unsafe {
        ptr::write(a, core::mem::zeroed::<pthread_condattr_t>());
    }
    0
}

#[no_mangle]
pub extern "C" fn pthread_condattr_destroy(_a: *mut pthread_condattr_t) -> c_int
{
    0
}

#[no_mangle]
pub extern "C" fn pthread_condattr_setclock(a: *mut pthread_condattr_t, clk: libc::clockid_t) -> c_int
{
    if (clk < 0) || (clk as u32).wrapping_sub(2) < 2 {return libc::EINVAL;}
    unsafe {
        (*a).__attr &= libc::INT_MIN as u32;
        (*a).__attr |= clk as u32;
    }
    0
}

#[no_mangle]
pub extern "C" fn pthread_condattr_setpshared(a: *mut pthread_condattr_t, pshared: c_int) -> c_int
{
    if pshared as u32 > 1u32 {return libc::EINVAL;}
    unsafe {
        (*a).__attr &= 0x7fffffff;
        (*a).__attr |= (pshared << 31) as u32;
    }
    0
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

#[no_mangle]
pub extern "C" fn pthread_barrier_init(b: *mut pthread_barrier_t, a: *const pthread_barrierattr_t, count: c_uint) -> c_int
{
    if count.wrapping_sub(1) > libc::INT_MAX as c_uint -1 {return libc::EINVAL;}
    unsafe {
        if b.is_null() {return libc::EINVAL;}
        let attr = if a.is_null() {0} else {(*a).__attr};
        ptr::write(b, core::mem::zeroed::<pthread_barrier_t>());
        (*b).__u.__i[2] = ((count-1) | attr) as c_int;
    }

    0
}

#[no_mangle]
pub extern "C" fn pthread_barrierattr_init(a: *mut pthread_barrierattr_t) -> c_int
{
    unsafe {
        ptr::write(a, core::mem::zeroed::<pthread_barrierattr_t>());
    }
    0
}

#[no_mangle]
pub extern "C" fn pthread_barrierattr_destroy(_a: *mut pthread_barrierattr_t) -> c_int
{
    0
}

#[no_mangle]
pub extern "C" fn pthread_barrierattr_setpshared(a: *mut pthread_barrierattr_t, pshared: c_int) -> c_int
{
    if pshared as u32 > 1u32 {return libc::EINVAL;}
    unsafe {
        (*a).__attr = if pshared!=0 {libc::INT_MIN as c_uint} else {0};
    }
    0
}

#[no_mangle]
pub extern "C" fn pshared_barrier_wait(b: *mut pthread_barrier_t) -> c_int {
    let limit: c_int = (unsafe{(*b)._b_limit()} & libc::INT_MAX) + 1;
    let mut ret: c_int = 0;
    let mut v: c_int;
    let mut w: c_int;

    if limit == 1 {
        return libc::PTHREAD_BARRIER_SERIAL_THREAD;
    }

    v = a_cas(unsafe{ptr::addr_of_mut!((*b).__u.__vi[0])}, 0, limit);
    while v != 0 {
        wait(unsafe{ptr::addr_of_mut!((*b).__u.__vi[0])}, unsafe{ptr::addr_of_mut!((*b).__u.__vi[1])}, v, 0);
        v = a_cas(unsafe{ptr::addr_of_mut!((*b).__u.__vi[0])}, 0, limit);
    }

    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!((*b).__u.__vi[3]), ptr::read_volatile(ptr::addr_of_mut!((*b).__u.__vi[3])) + 1);
    
        if (*b)._b_count() == limit {
            a_store(ptr::addr_of_mut!((*b).__u.__vi[3]), 0);
            ret = libc::PTHREAD_BARRIER_SERIAL_THREAD;
            if (*b)._b_waiters2() != 0 {
                wake(ptr::addr_of_mut!((*b).__u.__vi[3]), -1, 0);
            }
        } else {
            a_store(ptr::addr_of_mut!((*b).__u.__vi[0]), 0);
            if (*b)._b_waiters() != 0 {
                wake(ptr::addr_of_mut!((*b).__u.__vi[0]), 1, 0);
            }
            v = (*b)._b_count();
            while v > 0 {
                wait(ptr::addr_of_mut!((*b).__u.__vi[3]), ptr::addr_of_mut!((*b).__u.__vi[4]), v, 0);
                v = (*b)._b_count();
            }
        }
    
        vm_lock();

        if a_fetch_add(ptr::addr_of_mut!((*b).__u.__vi[3]), -1) == 1-limit {
            a_store(ptr::addr_of_mut!((*b).__u.__vi[3]), 0);
            if (*b)._b_waiters2() != 0 {
                wake(ptr::addr_of_mut!((*b).__u.__vi[3]), -1, 0);
            }
        } else {
            v = (*b)._b_count();
            while v != 0 {
                wait(ptr::addr_of_mut!((*b).__u.__vi[3]), ptr::addr_of_mut!((*b).__u.__vi[4]), v, 0);
                v = (*b)._b_count();
            }
        }

        loop {
            v = (*b)._b_lock();
            w = (*b)._b_waiters();
            if a_cas(ptr::addr_of_mut!((*b).__u.__vi[0]), v, if v==libc::INT_MIN+1 {0} else {v-1}) == v {break;}
        }

        if v==libc::INT_MIN+1 || (v==1 && w!=0) {
            wake(ptr::addr_of_mut!((*b).__u.__vi[0]), 1, 0);
        }

        vm_unlock();
    }

    ret
}

#[repr(C)]
struct instance {       // all members are volatile
    count: c_int,
    last: c_int,
    waiters: c_int,
    finished: c_int,
}

impl instance {
    pub fn new() -> Self {
        Self {
            count: 0,
            last: 0,
            waiters: 0,
            finished: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn pthread_barrier_wait(b: *mut pthread_barrier_t) -> c_int {
    let limit = unsafe{(*b)._b_limit()};
    let mut inst: *mut instance;

    if limit == 0 {return libc::PTHREAD_BARRIER_SERIAL_THREAD;}

    if limit < 0 {return pshared_barrier_wait(b);}

    unsafe {
        while a_swap(ptr::addr_of_mut!((*b).__u.__vi[0]), 1) != 0 {
            wait(ptr::addr_of_mut!((*b).__u.__vi[0]), ptr::addr_of_mut!((*b).__u.__vi[1]), 1, 1);
        }
        inst = (*b)._b_inst() as *mut instance;

        if inst == ptr::null_mut() {
            let mut new_inst: instance = instance::new();
            let mut spins: c_int = 200;
            inst = ptr::addr_of_mut!(new_inst);
            ptr::write_volatile(ptr::addr_of_mut!((*b).__u.__p[3]), ptr::addr_of_mut!(new_inst) as *mut c_void);
            a_store(ptr::addr_of_mut!((*b).__u.__vi[0]), 0);
            if (*b)._b_waiters() != 0 {
                wake(ptr::addr_of_mut!((*b).__u.__vi[0]), 1, 1);
            }
            while spins!=0 && ptr::read_volatile(&(*inst).finished) == 0  {
                a_spin();
                spins -= 1;
            }
            a_inc(ptr::addr_of_mut!((*inst).finished));
            while ptr::read_volatile(&(*inst).finished) == 1 {
                let _ = __syscall4(libc::SYS_futex, ptr::addr_of_mut!((*inst).finished) as c_long, (libc::FUTEX_WAIT | FUTEX_PRIVATE) as c_long, 1 as c_long, 0 as c_long) != -libc::ENOSYS as c_long
                    || __syscall4(libc::SYS_futex, ptr::addr_of_mut!((*inst).finished) as c_long, libc::FUTEX_WAIT as c_long, 1 as c_long, 0 as c_long) != 0;
            }
            return libc::PTHREAD_BARRIER_SERIAL_THREAD;
        }

        assert_ne!(inst, ptr::null_mut());
        (*inst).count += 1;
        if ptr::read_volatile(ptr::addr_of_mut!((*inst).count)) == limit {
            ptr::write_volatile(ptr::addr_of_mut!((*b).__u.__p[3]), ptr::null_mut());
            a_store(ptr::addr_of_mut!((*b).__u.__vi[0]), 0);
            if (*b)._b_waiters() != 0 {
                wake(ptr::addr_of_mut!((*b).__u.__vi[0]), 1, 1);
            }
            a_store(ptr::addr_of_mut!((*inst).last), 1);
            if ptr::read_volatile(&(*inst).waiters) != 0 {
                wake(ptr::addr_of_mut!((*inst).last), -1, 1);
            }
        } else {
            a_store(ptr::addr_of_mut!((*b).__u.__vi[0]), 0);
            if (*b)._b_waiters() != 0 {
                wake(ptr::addr_of_mut!((*b).__u.__vi[0]), 1, 1);
            }
            wait(ptr::addr_of_mut!((*inst).last), ptr::addr_of_mut!((*inst).waiters), 0, 1);
        }

        if a_fetch_add(ptr::addr_of_mut!((*inst).count), -1)==1 && a_fetch_add(ptr::addr_of_mut!((*inst).finished), 1)!=0 {
            wake(ptr::addr_of_mut!((*inst).finished), 1, 1);
        }
    }

    0
}

// #[no_mangle]
// pub extern "C" fn pthread_cleanup_push(f: extern "C" fn(*mut c_void) -> *mut c_void, x: *mut c_void) -> ()
// {
//     let mut __cb: __ptcb = __ptcb::new();
//     _pthread_cleanup_push(&mut __cb, f, x);
// }

// #[no_mangle]
// pub extern "C" fn _pthread_cleanup_push(cb: *mut __ptcb, f: extern "C" fn(*mut c_void) -> *mut c_void, x: *mut c_void) -> ()
// {
//     unsafe {
//         (*cb).__f = f;
//         (*cb).__x = x;
//         __do_cleanup_push(cb);
//     }
// }

// #[no_mangle]
// pub extern "C" fn __do_cleanup_push(cb: *mut __ptcb) -> ()
// {
//     let _self: pthread_t = pthread_self();
//     unsafe {
//         (*cb).__next = (*_self).cancelbuf;
//         (*_self).cancelbuf = cb;
//     }
// }
