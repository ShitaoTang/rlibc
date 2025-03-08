use libc::PTHREAD_MUTEX_NORMAL;
use libc::{uintptr_t, c_int, c_uchar, c_void, size_t, c_long, c_char, c_ulong, sigset_t, c_uint};
use core::arch::asm;
use core::option::Option;
use core::ptr;
use crate::arch::aarch64::syscall_arch::*;
use crate::arch::aarch64::atomic_arch::*;

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

pub const PTHREAD_CANCEL_ENABLE:c_int = 0;
pub const PTHREAD_CANCEL_DISABLE:c_int = 1;
pub const PTHREAD_CANCEL_MASKED:c_int = 2;

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
pub extern "C" fn get_tid(t: pthread_t) -> c_int {
    unsafe {(*t).tid}
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
pub extern "C" fn pthread_mutex_init(m: *mut pthread_mutex_t, a: *const pthread_mutexattr_t) -> c_int {
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
pub extern "C" fn pthread_mutex_lock(m: *mut pthread_mutex_t) -> c_int {
    if m.is_null() {
        return -1;
    }

    if ((unsafe{(*m)._m_type()} & 0xFFFF) == libc::PTHREAD_MUTEX_NORMAL)
     && a_cas(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, libc::EBUSY) == 0 {
        return 0;
    }

    pthread_mutex_timedlock(m, 0 as *const libc::timespec)
}

#[no_mangle]
pub extern "C" fn pthread_mutex_timedlock(m: *mut pthread_mutex_t, at: *const libc::timespec) -> c_int {
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
pub extern "C" fn pthread_mutex_trylock(m: *mut pthread_mutex_t) -> c_int {
    if m.is_null() {
        return -1;
    }

    if (unsafe{(*m)._m_type()} & 15) == libc::PTHREAD_MUTEX_NORMAL{
        return a_cas(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, libc::EBUSY) & libc::EBUSY;
    }

    return pthread_mutex_trylock_owner(m);
}

#[no_mangle]
pub extern "C" fn pthread_mutex_trylock_owner(m: *mut pthread_mutex_t) -> c_int {
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

fn success(m: *mut pthread_mutex_t, old: c_int, lock_type: c_int, _self: *mut pthread_t) -> c_int {
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

pub extern "C" fn futex4(uaddr: *mut c_void, op: c_int, val: c_int, to: *const libc::timespec) -> c_int {
    let res = unsafe {__syscall4(libc::SYS_futex, uaddr as c_long, op as c_long, val as c_long, to as c_long)};
    res as c_int
}

pub extern "C" fn timedwait(uaddr: *mut c_int, val: c_int, clk: libc::clockid_t, at: *const libc::timespec, lock_priv: c_int) -> c_int {
    let mut cs: c_int = 0;
    let r: c_int;

    pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, &mut cs);
    r = timewait_cp(uaddr, val, clk, at, lock_priv);
    pthread_setcancelstate(cs, ptr::null_mut());
    r
}

#[no_mangle]
pub extern "C" fn pthread_setcancelstate(new: c_int, old: *mut c_int) -> c_int {
    if new as c_uint > 2u32 {return libc::EINVAL;}   // trick, only when 0<=new<=2, it's valid (negatives are invalid)
    let mut _self: pthread_t = pthread_self();
    if old != ptr::null_mut() { unsafe  {*old = ptr::read_volatile(ptr::addr_of_mut!((*_self).canceldisable)) as c_int};}
    unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).canceldisable), new as c_uchar)};
    0
}

#[no_mangle]
pub extern "C" fn timewait_cp(uaddr: *mut c_int, val: c_int, clk: libc::clockid_t, at: *const libc::timespec, lock_priv: c_int) -> c_int {
    let mut r: c_int;
    let mut to: libc::timespec = libc::timespec {tv_sec: 0, tv_nsec: 0};
    let mut top: *mut libc::timespec = ptr::null_mut();
    let mut new_lock_priv = 0;

    if lock_priv != 0 {new_lock_priv = FUTEX_PRIVATE;}

    if at != ptr::null_mut() {
        if unsafe {(*at).tv_nsec} < 0 || unsafe {(*at).tv_nsec} >= 1000000000 {return libc::EINVAL;}   
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

    if lock_priv == 0 {
        r = -futex4_cp(uaddr, libc::FUTEX_WAIT|lock_priv, val, top)
    } else {
        r = -futex4_cp(uaddr, libc::FUTEX_WAIT|new_lock_priv, val, top);
    }
    if r != libc::EINTR && r!= libc::ETIMEDOUT && r != libc::ECANCELED {r = 0;}
    if r == libc::EINTR && unsafe {ptr::read(ptr::addr_of!(__eintr_valid_flag))} == 0 {r = 0;}

    r
}

#[no_mangle]
pub extern "C" fn clock_gettime(clk: libc::clockid_t, ts: *mut libc::timespec) -> c_int {
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
pub extern "C" fn futex4_cp(uaddr: *mut c_int, op: c_int, val: c_int, to: *const libc::timespec) -> c_int {
    let r: c_int = unsafe {
        __syscall4(libc::SYS_futex, uaddr as c_long, op as c_long, val as c_long, to as c_long) as c_int
    };
    if r != -libc::ENOSYS {return r;}
    let tmp = (op as c_long) & !(FUTEX_PRIVATE as c_long);
    unsafe {__syscall4(libc::SYS_futex, uaddr as c_long, tmp, val as c_long, to as c_long) as c_int}
}

#[no_mangle]
pub extern "C" fn pthread_mutex_timedlock_pi(m: *mut pthread_mutex_t, at: *const libc::timespec) -> c_int {
    let lock_type = unsafe {(*m)._m_type()};
    let lock_priv = (lock_type & 128) ^ 128;
    let mut _self: pthread_t = pthread_self();
    let mut e: c_int;

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
pub extern "C" fn pthread_mutex_unlock(m: *mut pthread_mutex_t) -> c_int {
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
pub unsafe extern "C" fn vm_wait() -> () {
    let mut tmp = vmlock[0];
    loop {
        wait(ptr::addr_of_mut!(vmlock) as *mut c_int, (ptr::addr_of_mut!(vmlock) as *mut c_int).add(1), tmp, 1);
        tmp = vmlock[0];
        if tmp == 0 {break;}
    }
}

#[no_mangle]
pub unsafe extern "C" fn vm_lock() -> () {
    a_inc(ptr::addr_of_mut!(vmlock) as *mut c_int);
}

#[no_mangle]
pub unsafe extern "C" fn vm_unlock() -> () {
    if a_fetch_add(ptr::addr_of_mut!(vmlock) as *mut c_int, -1) == 1 && vmlock[1] != 0 {
        wake(ptr::addr_of!(vmlock) as *mut c_int, -1, 1);
    }
}

#[no_mangle]
pub extern "C" fn wait(addr: *mut c_int, waiters: *mut c_int, val: c_int, lock_priv: c_int) -> () {
    let mut spins: c_int = 100;
    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { lock_priv };
    while spins != 0 && (waiters.is_null() || unsafe {ptr::read_volatile(waiters)} == 0) {
        spins -= 1;
        if unsafe {ptr::read_volatile(addr)} != val {
            a_barrier();
        } else {
            return;
        }
    }
    while unsafe {ptr::read_volatile(waiters)} == val {
        unsafe {
            let _ = __syscall4(libc::SYS_futex, addr as c_long, (libc::FUTEX_WAIT|lock_priv) as c_long, val as c_long, 0 as c_long) != -libc::ENOSYS as c_long
            || __syscall4(libc::SYS_futex, addr as c_long, libc::FUTEX_WAIT as c_long, val as c_long, 0 as c_long) != 0;
        }
    }
    if unsafe {ptr::read_volatile(waiters)} != 0 {
        a_dec(waiters);
    }
}

#[no_mangle]
#[inline(always)]
pub extern "C" fn wake(addr: *mut c_int, cnt: c_int, lock_priv: c_int) -> () {
    let lock_priv = if lock_priv != 0 { FUTEX_PRIVATE } else { lock_priv };
    let cnt = if cnt < 0 { libc::INT_MAX } else { cnt };
    unsafe {
        let _ = __syscall3(libc::SYS_futex, addr as c_long, (libc::FUTEX_WAKE|lock_priv) as c_long, cnt as c_long) != -libc::ENOSYS as c_long
        || __syscall3(libc::SYS_futex, addr as c_long, libc::FUTEX_WAKE as c_long, cnt as c_long) != 0;
    };
    
}

type pthread_spinlock_t = c_int;

#[no_mangle]
pub extern "C" fn pthread_spin_init(s: *mut pthread_spinlock_t, _pshared: c_int) -> c_int {
    unsafe {*s = 0;}
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_lock(s: *mut pthread_spinlock_t) -> c_int {
    unsafe {
        while ptr::read_volatile(s) != 0 || a_cas(s, 0, libc::EBUSY) != 0 {
            a_barrier();
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_trylock(s: *mut pthread_spinlock_t) -> c_int {
    a_cas(s, 0, libc::EBUSY);
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_unlock(s: *mut pthread_spinlock_t) -> c_int {
    a_store(s, 0);
    0
}

#[no_mangle]
pub extern "C" fn pthread_spin_destroy(_s: *mut pthread_spinlock_t) -> c_int {
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
pub extern "C" fn pthread_rwlock_init(rw: *mut pthread_rwlock_t, a: *const pthread_rwlockattr_t) -> c_int {
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
pub extern "C" fn pthread_rwlock_rdlock(rw: *mut pthread_rwlock_t) -> c_int {
    pthread_rwlock_timedrdlock(rw, 0 as *const libc::timespec)
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_timedrdlock(rw: *mut pthread_rwlock_t, at: *const libc::timespec) -> c_int {
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
pub extern "C" fn pthread_rwlock_tryrdlock(rw: *mut pthread_rwlock_t) -> c_int {
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
pub extern "C" fn pthread_rwlock_wrlock(rw: *mut pthread_rwlock_t) -> c_int {
    pthread_rwlock_timedwrlock(rw, 0 as *const libc::timespec)
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_timedwrlock(rw: *mut pthread_rwlock_t, at: *const libc::timespec) -> c_int {
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
    }

    r
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_trywrlock(rw: *mut pthread_rwlock_t) -> c_int {
    if a_cas(unsafe {ptr::addr_of_mut!((*rw).__u.__vi[0])}, 0, 0x7fffffff) != 0 {return libc::EBUSY;}
    0
}

#[no_mangle]
pub extern "C" fn pthread_rwlock_unlock(rw: *mut pthread_rwlock_t) -> c_int {
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