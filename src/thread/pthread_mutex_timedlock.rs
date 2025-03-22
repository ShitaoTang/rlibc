use crate::include::ctype::*;
use core::ptr;
use crate::arch::atomic_arch::*;
use crate::arch::syscall_arch::*;
use super::pthread_self::*;
use super::pthread_mutex_trylock::*;
use super::__timedwait::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::internal::futex::*;
use crate::include::time::*;
use super::*;

#[no_mangle]
pub extern "C" fn futex4(addr: *mut c_void, op: c_int, val: c_int, to: *const timespec) -> c_int
{
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    let res = unsafe {__syscall4(SYS_futex as c_long, addr as c_long, op as c_long, val as c_long, to as c_long)};
    // unsafe{if addr.is_null() { asm!("brk #0", options(noreturn)); }}
    res as c_int
}

#[no_mangle]
pub extern "C" fn pthread_mutex_timedlock_pi(m: *mut pthread_mutex_t, at: *const timespec) -> c_int
{
    let lock_type = unsafe {(*m)._m_type()};
    let lock_priv = (lock_type & 128) ^ 128;
    let mut _self: pthread_t = pthread_self();
    let mut e: c_int;

    // unsafe{if m.is_null() { asm!("brk #0", options(noreturn)); }}
    if lock_priv == 0 {
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending),
             ptr::read_volatile(ptr::addr_of_mut!((*m).__u.__p[4])))};
    }

    loop {
        e = -futex4(unsafe{ptr::addr_of!((*m).__u.__vi[1]) as *mut i32 as *mut c_void},
         FUTEX_LOCK_PI | lock_priv, 0, at);
        if e != EINTR {break;}
    }
    if e != 0 {
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::null_mut())};
    }

    'block:{
        match e {
            0 => {
                if (lock_type&4 == 0) && (unsafe{((*m)._m_lock() & 0x40000000 != 0) || (*m)._m_waiters() != 0}) {
                    a_store(unsafe{ptr::addr_of_mut!((*m).__u.__vi[2])}, -1);
                    unsafe {__syscall2(SYS_futex as c_long, ptr::addr_of_mut!((*m).__u.__vi[1]) as *mut _ as c_long,
                         (FUTEX_UNLOCK_PI | lock_priv) as c_long);}
                    unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::null_mut())};
                    break 'block;
                }
                unsafe {(*m).__u.__i[5] = -1};
                return pthread_mutex_trylock(m);
            }
            ETIMEDOUT => {return e}
            EDEADLK => {
                if lock_type&3 == PTHREAD_MUTEX_ERRORCHECK {return e}
            }
            _ => {}
        }
        loop {
            e = timedwait(ptr::null_mut(), 0, CLOCK_REALTIME, at, 1);
            if e == ETIMEDOUT {break;}
        }
    }

    e
}

#[no_mangle]
pub extern "C" fn pthread_mutex_timedlock(m: *mut pthread_mutex_t, at: *const timespec) -> c_int
{
    if m.is_null() {
        return -1;
    }

    if ((unsafe{(*m)._m_type()} & 15) == PTHREAD_MUTEX_NORMAL)
     && a_cas(unsafe {ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, EBUSY) == 0 {
        return 0;
    }

    let lock_type = unsafe {(*m)._m_type()};
    let mut r: c_int ;
    let mut t: c_int;
    let lock_priv = (lock_type & 128) ^ 128;
    let _self: pthread_t = pthread_self();

    r = pthread_mutex_trylock(m);
    if r != EBUSY {return r;}

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
    while r == EBUSY {
        r = unsafe {(*m)._m_lock()};
        let own = r & 0x3fffffff;
        if own == 0 && (r ==0 || lock_type&4 != 0) {
            r = pthread_mutex_trylock(m);
            continue;
        }
        if lock_type &3 == PTHREAD_MUTEX_ERRORCHECK && own == unsafe {(*_self).tid} {return EDEADLK;}

        a_inc(unsafe {ptr::addr_of_mut!((*m).__u.__vi[2])});
        t = r | c_int::MIN;
        a_cas(unsafe {ptr::addr_of_mut!((*m).__u.__vi[1])}, r, t);
        r = timedwait(unsafe {ptr::addr_of_mut!((*m).__u.__vi[1])}, t, CLOCK_REALTIME, at, lock_priv);
        a_dec(unsafe {ptr::addr_of_mut!((*m).__u.__vi[2])});
        if r != 0 && r != EINTR {break;}
        r = pthread_mutex_trylock(m);
    }

    r
}