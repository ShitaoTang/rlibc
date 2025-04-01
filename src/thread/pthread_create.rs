use crate::arch::atomic_arch::*;
use crate::arch::generic::bits::errno::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
use crate::env::__init_tls;
use crate::env::__init_tls::__copy_tls;
use crate::exit::exit::*;
use crate::include::ctype::*;
use crate::include::limits::PAGE_SIZE;
use crate::include::sched::*;
use crate::include::libc;
use crate::include::signal::*;
use crate::include::sys::mman::*;
use crate::internal::lock::*;
use crate::mman::mmap::__mmap;
use crate::mman::munmap::__munmap;
use crate::mman::mprotect::__mprotect;
use crate::stdio::ofl::*;
use crate::stdio::stderr::__stderr_used;
use crate::stdio::stdin::__stdin_used;
use crate::stdio::stdout::__stdout_used;
use crate::stdio::ftrylockfile::*;
use crate::stdio::dlerror::*;
use crate::string::memcpy::*;
use crate::string::memset::*;
use crate::thread::vmlock::vm_wait;
use super::*;
use super::__wait::wait;
use super::default_attr::*;
use super::pthread_self::*;
use super::pthread_impl::*;
use super::vmlock::*;
use core::ptr;
use super::pthread_key_create::*;
#[cfg(target_os = "linux")]
use crate::linux::membarrier::__membarrier_init;
use super::lock_ptc::*;
use crate::arch::bits::signal::*;
use crate::signal::block::*;
use super::clone::__clone;
use super::__unmapself::*;

static mut tl_lock_count: c_int = 0;
static mut tl_lock_waiters: c_int = 0;

pub unsafe fn __tl_lock()
{
    let tid = (*pthread_self()).tid;
    let mut val = ptr::read_volatile(ptr::addr_of_mut!(__init_tls::__thread_list_lock));
    if val == tid {
        tl_lock_count += 1;
        return;
    }
    val = a_cas(ptr::addr_of_mut!(__init_tls::__thread_list_lock), 0, tid);
    while val != 0 {
        wait(
            ptr::addr_of_mut!(__init_tls::__thread_list_lock),
            ptr::addr_of_mut!(tl_lock_waiters),
            val,
            0);
        val = a_cas(ptr::addr_of_mut!(__init_tls::__thread_list_lock), 0, tid);
    }
}

pub unsafe fn __tl_unlock()
{
    if tl_lock_count != 0 {
        tl_lock_count -= 1;
        return;
    }
    a_store(ptr::addr_of_mut!(__init_tls::__thread_list_lock), 0);
    if tl_lock_waiters != 0 {
        wake(
            ptr::addr_of_mut!(__init_tls::__thread_list_lock),
            1,
            0,
        )
    }
}

pub unsafe fn __tl_sync(_td: pthread_t)
{
    a_barrier();
    let val = ptr::read_volatile(ptr::addr_of_mut!(__init_tls::__thread_list_lock));
    if val == 0 { return; }
    wait(
        ptr::addr_of_mut!(__init_tls::__thread_list_lock),
        ptr::addr_of_mut!(tl_lock_waiters),
        val,
        0,
    );
    if tl_lock_waiters != 0 {
        wake(
            ptr::addr_of_mut!(__init_tls::__thread_list_lock),
            1,
            0,
        )
    }
}

unsafe fn init_file_lock(f: *mut FILE)
{
    if !f.is_null() && (*f).lock < 0 {
        (*f).lock = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn pthread_exit(result: *mut c_void)
{
    let mut _self = pthread_self();
    let mut seg: sigset_t = core::mem::zeroed();

    (*_self).canceldisable = 1;
    (*_self).cancelasync = 0;
    (*_self).result = result;

    while !(*_self).cancelbuf.is_null() {
        let f = (*(*_self).cancelbuf).__f;
        let x = (*(*_self).cancelbuf).__x;
        (*_self).cancelbuf = (*(*_self).cancelbuf).__next;
        if let Some(f) = f {
            f(x);
        }
    }

    __pthread_tsd_run_dtors();

    __block_app_sigs(ptr::addr_of_mut!(seg) as *mut c_void);

    let state = a_cas(ptr::addr_of_mut!((*_self).detach_state),
        DT_STATUS::DT_JOINABLE as c_int,
        DT_STATUS::DT_EXITING as c_int);

    if state==DT_STATUS::DT_DETACHED as c_int && (*_self).map_base != ptr::null_mut() {
        vm_wait();
    }

    LOCK((*_self).killlock.as_mut_ptr());

    __tl_lock();

    if (*_self).next == _self {
        __tl_unlock();
        UNLOCK((*_self).killlock.as_mut_ptr());
        (*_self).detach_state = state;
        __restore_sigs(ptr::addr_of_mut!(seg) as *const c_void);
        exit(0);
    }

    (*_self).tid = 0;
    UNLOCK((*_self).killlock.as_mut_ptr());

    vm_lock();
    let mut rp: *mut *mut c_void = ptr::null_mut();    // volatile
    ptr::write_volatile(ptr::addr_of_mut!(rp),
        ptr::read_volatile(ptr::addr_of_mut!((*_self).robust_list.head)) as *mut *mut c_void);
    while ptr::read_volatile(ptr::addr_of_mut!(rp)) != ptr::null_mut() 
        && ptr::read_volatile(ptr::addr_of_mut!(rp)) != ptr::addr_of_mut!((*_self).robust_list.head) {
        let m: *mut pthread_mutex_t = (rp as *mut c_char).
            sub(core::mem::offset_of!(pthread_mutex_t, __u) + 4*size_of::<*mut c_void>())
            as *mut pthread_mutex_t;
        let waiters = (*m)._m_waiters();
        let type_priv = ((*m)._m_type() & 128) ^ 128;
        ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.pending),
            ptr::read_volatile(ptr::addr_of_mut!(rp)) as *mut c_void);
        ptr::write_volatile(ptr::addr_of_mut!((*_self).robust_list.head), 
            ptr::addr_of_mut!(rp) as *mut c_void);
        let cont = a_swap(ptr::addr_of_mut!((*m).__u.__vi[1]), 0x40000000);
        if cont<0 || waiters!=0 {
            wake(ptr::addr_of_mut!((*m).__u.__vi[1]), 1, type_priv);
        }
    }

    vm_unlock();

    __do_orphaned_stdio_locks();
    __dl_thread_cleanup();

    libc::libc.threads_minus_1 -= 1;
    if libc::libc.threads_minus_1 == 0 {
        libc::libc.need_locks = -1;
    }
    (*(*_self).next).prev = (*_self).prev;
    (*(*_self).prev).next = (*_self).next;
    (*_self).prev = _self;
    (*_self).next = _self;

    if state == DT_STATUS::DT_DETACHED as c_int
        && (*_self).map_base != ptr::null_mut() {
        __block_all_sigs(ptr::addr_of_mut!(seg) as *mut c_void);

        if (*_self).robust_list.off!=0 {
            __syscall2(SYS_set_robust_list as c_long,
                0, 3*size_of::<c_long>() as c_long);
        }

        __unmapself((*_self).map_base as *mut c_void, (*_self).map_size);
    }

    a_store(ptr::addr_of_mut!((*_self).detach_state), DT_STATUS::DT_EXITED as c_int);
    wake(
        ptr::addr_of_mut!((*_self).detach_state) as *mut c_int,
        1,
        1
    );

    loop {
        __syscall1(SYS_exit as c_long, 0);
    }

}

#[inline(always)]
fn ROUND(x: size_t) -> size_t
{
    (x + PAGE_SIZE - 1) & PAGE_SIZE.wrapping_neg()
}

#[no_mangle]
pub unsafe extern "C" fn __do_cleanup_push(cb: *mut __ptcb)
{
    (*cb).__next = (*pthread_self()).cancelbuf;
    (*pthread_self()).cancelbuf = cb;
}

#[no_mangle]
pub unsafe extern "C" fn __do_cleanup_pop(cb: *mut __ptcb)
{
    (*pthread_self()).cancelbuf = (*cb).__next;
}

#[repr(C)]
pub struct start_args {
    pub start_func: extern "C" fn(*mut c_void) -> *mut c_void,
    pub start_arg: *mut c_void,
    pub control: c_int,       // volatile
    pub sigmask: [c_ulong; _NSIG/8/core::mem::size_of::<c_long>()],
}

#[no_mangle]
unsafe extern "C" fn start(p: *mut c_void) -> c_int
{
    let args = p as *mut start_args;
    let state = (*args).control;
    if state != 0 {
        if a_cas(ptr::addr_of_mut!((*args).control), 1, 2) == 1 {
            wait(ptr::addr_of_mut!((*args).control) as *mut c_int, ptr::null_mut(), 2, 1);
        }
        if (*args).control != 0 {
            __syscall1(SYS_set_tid_address as c_long,
                ptr::addr_of_mut!((*args).control) as c_long);
            loop {
                __syscall1(SYS_exit as c_long, 0);
            }
        }
    }
    __syscall4(SYS_rt_sigprocmask as c_long, SIG_SETMASK as c_long,
        (*args).sigmask.as_mut_ptr() as c_long, 0, (_NSIG/8)as c_long);
    pthread_exit(((*args).start_func)((*args).start_arg));

    0
}

#[no_mangle]
unsafe extern "C" fn start_c11(p: *mut c_void) -> c_int
{
    let args = p as *mut start_args;
    let start: extern "C" fn(*mut c_void) -> *mut c_void = (*args).start_func;

    pthread_exit(start((*args).start_arg));
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_create(
    res: *mut pthread_t,
    attrp: *const pthread_attr_t,
    entry: extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> c_int {
    let mut ret: c_int;
    let c11: bool = attrp == __ATTRP_C11_THREAD as *const pthread_attr_t;
    let mut size: size_t;
    let guard: size_t;
    let mut _self: *mut pthread;
    let new: *mut pthread;
    let mut map: *mut c_uchar = ptr::null_mut();
    let mut stack: *mut c_uchar = ptr::null_mut();
    let mut tsd: *mut c_uchar = ptr::null_mut();
    let mut stack_limit: *mut c_uchar = ptr::null_mut();
    let flags: c_uint = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND
        | CLONE_THREAD | CLONE_SYSVSEM | CLONE_SETTLS
        | CLONE_PARENT_SETTID | CLONE_CHILD_CLEARTID | CLONE_DETACHED;
    let mut attr: pthread_attr_t = core::mem::zeroed();
    let mut set: sigset_t = core::mem::zeroed();

    if libc::libc.can_do_threads == 0 {
        return ENOSYS;
    }
    _self = pthread_self();
    if libc::libc.threaded == 0 {
        let mut f = *__ofl_lock();
        while !f.is_null() {
            init_file_lock(f);
            f = (*f).next;
        }
        __ofl_unlock();
        init_file_lock(__stdin_used);
        init_file_lock(__stdout_used);
        init_file_lock(__stderr_used);
        __syscall4(SYS_rt_sigprocmask as c_long, SIG_UNBLOCK as c_long,
            SIGPT_SET as c_long, 0, (_NSIG/8)as c_long);
        (*_self).tsd = ptr::addr_of_mut!(__pthread_tsd_main) as *mut *mut c_void;
        #[cfg(target_os = "linux")]
        __membarrier_init();
        libc::libc.threaded = 1;
    }
    if !attrp.is_null() && !c11 { attr = attrp.read(); }

    __acquire_ptc();
    if attrp.is_null() || c11 {
        attr.__u.__s[0] = __default_stack_size as c_ulong;
        attr.__u.__s[1] = __default_guard_size as c_ulong;
    }

    if attr._a_stackaddr() != 0 {
        let need: size_t = libc::libc.tls_size
            + ptr::read_volatile(ptr::addr_of_mut!(__pthread_tsd_size));
        size = attr._a_stacksize() as size_t;
        stack = (attr._a_stacksize() & (16 as c_ulong).wrapping_neg()) as *mut c_uchar;
        stack_limit = stack.offset(-(size as isize));

        if need<size/8 && need<2048 {
            tsd = stack.offset(ptr::read_volatile(ptr::addr_of_mut!(__pthread_tsd_size)).wrapping_neg() as isize);
            stack = tsd.sub(libc::libc.tls_size);
            memset(stack as *mut c_void, 0, need);
        } else {
            size = ROUND(need);
        }
        guard = 0;
    } else {
        guard = ROUND(attr._a_guardsize() as size_t);
        size = guard + ROUND(attr._a_stacksize() as size_t
            + libc::libc.tls_size + ptr::read_volatile(ptr::addr_of_mut!(__pthread_tsd_size)));
    }

    if tsd == ptr::null_mut() {
        if guard != 0 {
            map = __mmap(ptr::null_mut(), size, PROT_NONE as c_int,
                (MAP_PRIVATE|MAP_ANNO) as c_int, -1, 0) as *mut c_uchar;
            if map == MAP_FAILED as *mut c_uchar { return fail(); }
            if __mprotect(map.add(guard) as *mut c_void, size-guard, (PROT_READ|PROT_WRITE) as c_int)!=0
                && (*pthread_self()).errno_val != ENOSYS {
                __munmap(map as *mut c_void, size);
                return fail();
            }
        } else {
            map = __mmap(ptr::null_mut(), size, (PROT_READ|PROT_WRITE) as c_int,
                (MAP_PRIVATE|MAP_ANNO) as c_int, -1, 0) as *mut c_uchar;
            if map == MAP_FAILED as *mut c_uchar { return fail(); }
        }
        tsd = map.add(size - ptr::read_volatile(ptr::addr_of_mut!(__pthread_tsd_size)) as size_t);
        if stack.is_null() {
            stack = tsd.sub(libc::libc.tls_size);
            stack_limit = map.add(guard);
        }
    }

    new = __copy_tls(tsd.sub(libc::libc.tls_size) as *mut c_uchar) as *mut pthread;
    (*new).map_base = map;
    (*new).map_size = size;
    (*new).stack = stack as *mut c_void;
    (*new).stack_size = stack.offset_from(stack_limit) as size_t;
    (*new).guard_size = guard;
    (*new)._self = new;
    (*new).tsd = tsd as *mut *mut c_void;
    (*new).locale = ptr::addr_of_mut!(libc::libc.global_locale);
    if attr._a_detach()!=0 {
        (*new).detach_state = DT_STATUS::DT_DETACHED as c_int;
    } else {
        (*new).detach_state = DT_STATUS::DT_JOINABLE as c_int;
    }
    (*new).robust_list.head = ptr::addr_of_mut!((*new).robust_list.head) as *mut c_void;
    (*new).canary = (*_self).canary;
    (*new).sysinfo = (*_self).sysinfo;
    
    stack = stack.sub((stack as uintptr_t) % core::mem::size_of::<uintptr_t>());
    stack = stack.sub(size_of::<start_args>());
    let args: *mut start_args = stack as *mut start_args;
    (*args).start_func = entry;
    (*args).start_arg = arg;
    (*args).control = if attr._a_sched()!=0 {1} else {0};

    __block_app_sigs(&mut set as *mut _ as *mut c_void);

    memcpy(
        (*args).sigmask.as_mut_ptr() as *mut c_void,
        &set as *const _ as *const c_void,
        core::mem::size_of_val(&(*args).sigmask) as size_t,
    );
    (*args).sigmask[(SIGCANCEL-1)/8/size_of::<c_long>()] &=
        !(1 << ((SIGCANCEL-1) % (8*size_of::<c_long>())));
    
    __tl_lock();
    if libc::libc.threads_minus_1 == 0 {
        libc::libc.need_locks = 1;
    }
    libc::libc.threads_minus_1 += 1;
    ret = __clone(
        if c11 {start_c11} else {start},
        stack as *mut c_void,
        flags as c_int,
        args as *mut c_void,
        ptr::addr_of_mut!((*new).tid),
        TP_ADJ(new as *mut c_void),
        ptr::addr_of_mut!(__init_tls::__thread_list_lock) as *mut c_int,
    );

    if ret < 0 {
        ret = -EAGAIN;
    } else if attr._a_sched()!=0 {
        ret = __syscall3(SYS_sched_setscheduler as c_long,
            (*new).tid as c_long,
            attr._a_policy() as c_long,
            ptr::addr_of_mut!((attr.__u.__i[3*__SU+3])) as c_long) as c_int;
        if a_swap(ptr::addr_of_mut!((*args).control), if ret!=0 {3} else {0}) == 2 {
            wake(ptr::addr_of_mut!((*args).control) as *mut c_int, 1, 1);
        }
        if ret != 0 {
            wait(
                ptr::addr_of_mut!((*args).control) as *mut c_int,
                ptr::null_mut(),
                3,
                0,
            );
        }
    }

    if ret >= 0 {
        (*new).next = (*_self).next;
        (*new).prev = _self;
        (*(*new).next).prev = new;
        (*(*new).prev).next = new;
    } else {
        libc::libc.threads_minus_1 -= 1;
        if libc::libc.threads_minus_1 == 0 {
            libc::libc.need_locks = 0;
        }
    }

    __tl_unlock();
    __restore_sigs(ptr::addr_of_mut!(set) as *const c_void);
    __release_ptc();

    if ret < 0 {
        if !map.is_null() {
            __munmap(map as *mut c_void, size);
        }
        return -ret;
    }

    *res = new;    
    0
}

fn fail() -> c_int
{
    __release_ptc();
    EAGAIN
}