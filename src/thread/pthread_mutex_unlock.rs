use crate::include::ctype::*;
use core::ptr;
use super::pthread_self::*;
use crate::arch::atomic_arch::*;
use crate::__syscall;
use crate::arch::syscall_arch::*;
use super::vmlock::*;
use super::pthread_impl::*;
use super::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::internal::futex::*;

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

    if lock_type != PTHREAD_MUTEX_NORMAL {
        old = unsafe {(*m)._m_lock()};
        let own = old & 0x3fffffff;
        if own != unsafe {(*_self).tid} { return EPERM; }
        if lock_type&3 == PTHREAD_MUTEX_RECURSIVE && unsafe {(*m)._m_count()} != 0 {
            unsafe {(*m).__u.__i[5] -= 1;}
            return 0;
        }
        if lock_type&4 != 0 && (old&0x40000000) != 0 {new = 0x7fffffff;}
        if lock_priv == 0 {
            // unsafe{if m.is_null() { asm!("brk #0", options(noreturn)); }}
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

        // unsafe{if _self.is_null() { asm!("brk #0", options(noreturn)); }}
        // unsafe{if next.is_null() { asm!("brk #0", options(noreturn)); }}
        // unsafe{if prev.is_null() { asm!("brk #0", options(noreturn)); }}
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
            // unsafe {__syscall2(SYS_futex as c_long,
            //      ptr::addr_of_mut!((*m).__u.__vi[1]) as c_long,
            //      (FUTEX_UNLOCK_PI | lock_priv) as c_long);}
            __syscall!(SYS_futex, ptr::addr_of_mut!((*m).__u.__vi[1]), (FUTEX_UNLOCK_PI | lock_priv));   
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