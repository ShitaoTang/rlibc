use crate::include::ctype::*;
use core::ptr;
use crate::__syscall;
use crate::arch::atomic_arch::*;
use crate::arch::syscall_arch::*;
use super::*;
use super::__wait::*;
use super::__timedwait::*;
use super::pthread_impl::*;
use super::pthread_self::*;
use super::pthread_mutex_lock::*;
use super::pthread_mutex_unlock::*;
use super::pthread_setcancelstate::*;
use super::pthread_testcancel::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::internal::futex::*;
use crate::include::time::*;

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
        // unsafe {
        //     let _ = 
        //     __syscall5(SYS_futex as c_long, l as c_long, (FUTEX_REQUEUE | FUTEX_PRIVATE) as c_long,
        //      0 as c_long, 1 as c_long, r as c_long) != ENOSYS as c_long
        //     || __syscall5(SYS_futex as c_long, l as c_long, FUTEX_REQUEUE as c_long,
        //      0 as c_long, 1 as c_long, r as c_long) != 0;
        // };
        let _ = __syscall!(SYS_futex, l, (FUTEX_REQUEUE | FUTEX_PRIVATE), 0, 1, r) != -ENOSYS as c_long
            || __syscall!(SYS_futex, l, FUTEX_REQUEUE, 0, 1, r) != 0;
    }
}

#[repr(C)]
pub enum THREAD_STATE {
    WAITING,
    SIGNALED,
    LEAVING,
}

#[no_mangle]
pub extern "C" fn pthread_cond_timedwait(c: *mut pthread_cond_t, m: *mut pthread_mutex_t, ts: *const timespec) -> c_int
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
        return EINVAL;
    }

    if unsafe{(*m)._m_type()} & 15 != 0 && unsafe{(*m)._m_lock()&c_int::MAX != (*_self).tid} {return EPERM;}

    if !ts.is_null() && (unsafe{(*ts).tv_nsec} as u64 >= 1000000000u64) {return EINVAL;}

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
        if !(unsafe{*(fut)}==seq && (e==0 || e==EINTR)) {
            break;
        }
    }
    if e == EINTR {
        e = 0;
    }

    if shared != 0 {
        if e == ECANCELED && unsafe{(*c)._c_seq() != seq} {e = 0;}
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
                a_cas(ptr::addr_of_mut!((*m).__u.__vi[1]), val, val | c_int::MIN);
            }
            unlock_requeue(ptr::addr_of_mut!((*(*node).prev).barrier), ptr::addr_of_mut!((*m).__u.__vi[1]), (*m)._m_type()&(8|128));
        } else if ((*m)._m_type() & 8) == 0 {
            a_dec(ptr::addr_of_mut!((*m).__u.__vi[2]));
        }
    }

    if *e == ECANCELED {*e = 0;}

    done(*e, cs)
}

fn done(e: c_int, cs: c_int) -> c_int
{
    pthread_setcancelstate(cs, ptr::null_mut());

    if e == ECANCELED {
        pthread_testcancel();
        pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, ptr::null_mut());
    }

    e
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
