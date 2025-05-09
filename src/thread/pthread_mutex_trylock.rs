use crate::include::ctype::*;
use core::ptr;
use crate::arch::atomic_arch::*;
use super::pthread_self::*;
use crate::__syscall;
use crate::arch::syscall_arch::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use super::*;
use crate::internal::futex::*;

#[no_mangle]
pub extern "C" fn pthread_mutex_trylock(m: *mut pthread_mutex_t) -> c_int
{
    if m.is_null() {
        return -1;
    }

    if (unsafe{(*m)._m_type()} & 15) == PTHREAD_MUTEX_NORMAL{
        return a_cas(unsafe{ptr::addr_of_mut!((*m).__u.__vi[1])}, 0, EBUSY) & EBUSY;
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

        if (lock_type & 3) == PTHREAD_MUTEX_RECURSIVE {
            if unsafe{(*m)._m_count() as c_uint >= 0x7fffffff as c_uint} {
                return EAGAIN;
            }
            unsafe {(*m).__u.__i[5] += 1};
            return 0;
        }
    }

    if own == 0x3fffffff {return ENOTRECOVERABLE;}
    if own != 0 || (old != 0 && (lock_type & 4) == 0) {return EBUSY;}

    if (lock_type & 128) != 0 {
        if unsafe {(*_self).robust_list.off == 0} {
            unsafe {
                // (*_self).robust_list.off = ((ptr::addr_of_mut!((*m).__u.__vi[1]) as usize) - (ptr::addr_of_mut!((*m).__u.__p[4]) as usize)) as c_long;
                (*_self).robust_list.off = ((ptr::addr_of!((*m).__u.__vi) as *const i32).add(1) as *const u8)
                            .offset_from((ptr::addr_of!((*m).__u.__p) as *const *mut u8).add(4) as *const u8) as c_long;
                // __syscall2(SYS_set_robust_list as c_long,
                //      ptr::addr_of_mut!((*_self).robust_list) as c_long,
                //  (3* core::mem::size_of::<c_long>()) as c_long);
                __syscall!(
                    SYS_set_robust_list,
                    ptr::addr_of_mut!((*_self).robust_list),
                    (3 * core::mem::size_of::<c_long>())
                );
            }
        }
        if unsafe {(*m)._m_waiters()} != 0 {tid |= c_int::MIN;}
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending),
             &mut (*m).__u.__p[4] as *mut _ as *mut c_void)};
    }
    tid |= old & 0x40000000;

    if a_cas(unsafe{ ptr::addr_of_mut!((*m).__u.__vi[1]) }, old, tid) != old {
        unsafe {ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending), ptr::null_mut())};
        if (lock_type & 12) == 12 && unsafe {(*m)._m_waiters() != 0} { return ENOTRECOVERABLE; }
        return EBUSY;
    }

    success(m, old, lock_type, ptr::addr_of_mut!(_self));

    0
}

fn success(m: *mut pthread_mutex_t, old: c_int, lock_type: c_int, _self: *mut pthread_t) -> c_int
{
    if (lock_type & 8) != 0 && unsafe { (*m)._m_waiters() } != 0 {
        let priv_flag = (lock_type & 128) ^ 128;
        // unsafe { __syscall2(SYS_futex as c_long,
        //      ptr::addr_of!((*m).__u.__vi[1]) as c_long,
        //      (FUTEX_UNLOCK_PI | priv_flag) as c_long); }
        __syscall!(
            SYS_futex,
            ptr::addr_of!((*m).__u.__vi[1]),
            (FUTEX_UNLOCK_PI | priv_flag)
        );
        unsafe { ptr::write_volatile(ptr::addr_of_mut!((*(*_self)).robust_list.pending), ptr::null_mut()); }
        return if (lock_type & 4) != 0 {
            ENOTRECOVERABLE
        } else {
            EBUSY
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
        return EOWNERDEAD;
    }

    0
}