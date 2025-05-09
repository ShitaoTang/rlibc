use crate::include::ctype::*;
use core::ptr;
use crate::__syscall;
use crate::arch::atomic_arch::*;
use crate::arch::syscall_arch::*;
use super::__wait::*;
use super::pthread_impl::*;
use super::vmlock::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_bits::*;
use crate::internal::futex::*;
use super::*;

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
pub extern "C" fn pshared_barrier_wait(b: *mut pthread_barrier_t) -> c_int {
    let limit: c_int = (unsafe{(*b)._b_limit()} & c_int::MAX) + 1;
    let mut ret: c_int = 0;
    let mut v: c_int;
    let mut w: c_int;

    if limit == 1 {
        return PTHREAD_BARRIER_SERIAL_THREAD;
    }

    v = a_cas(unsafe{ptr::addr_of_mut!((*b).__u.__vi[0])}, 0, limit);
    while v != 0 {
        wait(unsafe{ptr::addr_of_mut!((*b).__u.__vi[0])},
         unsafe{ptr::addr_of_mut!((*b).__u.__vi[1])}, v, 0);
        v = a_cas(unsafe{ptr::addr_of_mut!((*b).__u.__vi[0])}, 0, limit);
    }

    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!((*b).__u.__vi[3]),
         ptr::read_volatile(ptr::addr_of_mut!((*b).__u.__vi[3])) + 1);
    
        if (*b)._b_count() == limit {
            a_store(ptr::addr_of_mut!((*b).__u.__vi[3]), 0);
            ret = PTHREAD_BARRIER_SERIAL_THREAD;
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
            if a_cas(ptr::addr_of_mut!((*b).__u.__vi[0]), v, if v==c_int::MIN+1 {0} else {v-1}) == v {break;}
        }

        if v==c_int::MIN+1 || (v==1 && w!=0) {
            wake(ptr::addr_of_mut!((*b).__u.__vi[0]), 1, 0);
        }

        vm_unlock();
    }

    ret
}

#[no_mangle]
pub extern "C" fn pthread_barrier_wait(b: *mut pthread_barrier_t) -> c_int {
    let limit = unsafe{(*b)._b_limit()};
    let mut inst: *mut instance;

    if limit == 0 {return PTHREAD_BARRIER_SERIAL_THREAD;}

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
                // let _ = __syscall4(SYS_futex as c_long, ptr::addr_of_mut!((*inst).finished) as c_long, 
                //             (FUTEX_WAIT | FUTEX_PRIVATE) as c_long, 1 as c_long, 0 as c_long) != -ENOSYS as c_long
                //      || __syscall4(SYS_futex as c_long, ptr::addr_of_mut!((*inst).finished) as c_long, 
                //             FUTEX_WAIT as c_long, 1 as c_long, 0 as c_long) != 0;
                let _ = __syscall!(SYS_futex, ptr::addr_of_mut!((*inst).finished), (FUTEX_WAIT | FUTEX_PRIVATE), 1, 0) != -ENOSYS as c_long
                     || __syscall!(SYS_futex, ptr::addr_of_mut!((*inst).finished), FUTEX_WAIT, 1, 0) != 0;
            }
            return PTHREAD_BARRIER_SERIAL_THREAD;
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

        if a_fetch_add(ptr::addr_of_mut!((*inst).count), -1)==1
         && a_fetch_add(ptr::addr_of_mut!((*inst).finished), 1)!=0 {
            wake(ptr::addr_of_mut!((*inst).finished), 1, 1);
        }
    }

    0
}
