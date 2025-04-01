use crate::arch::generic::bits::errno::EAGAIN;
use crate::include::ctype::*;
use crate::include::limits::*;
use crate::signal::block::__block_app_sigs;
use crate::signal::block::__restore_sigs;
use core::ptr;
use core::ptr::fn_addr_eq;
use core::sync::atomic::AtomicPtr;
use super::pthread_self::*;
use super::pthread_create::*;
use super::pthread_rwlock_rdlock::*;
use super::pthread_rwlock_wrlock::*;
use super::pthread_rwlock_unlock::*;
use super::*;

/// volatile
pub static mut __pthread_tsd_size: size_t = core::mem::size_of::<*mut c_void>() * PTHREAD_KEYS_MAX;

pub static mut __pthread_tsd_main: [AtomicPtr<c_void>; PTHREAD_KEYS_MAX] = {
    const NULL: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
    [NULL; PTHREAD_KEYS_MAX]
};

static mut keys: [unsafe extern "C" fn(*mut c_void); PTHREAD_KEYS_MAX] = {
    unsafe extern "C" fn null_fn(_: *mut c_void) {}
    [null_fn; PTHREAD_KEYS_MAX]
};

unsafe extern "C" fn nodtor(_: *mut c_void)
{
}

static mut key_lock: pthread_rwlock_t = unsafe { core::mem::zeroed() };

static mut next_key: pthread_key_t = 0;

#[no_mangle]
pub unsafe extern "C" fn __pthread_tsd_run_dtors()
{
    let _self = pthread_self();
    let mut i: size_t;
    let mut j: size_t = 0;
    while (*_self).tsd_used!=0 && j<PTHREAD_DESTRUCTOR_ITERATIONS {
        pthread_rwlock_rdlock(ptr::addr_of_mut!(key_lock));
        (*_self).tsd_used = 0;
        i = 0;
        while i < PTHREAD_KEYS_MAX {
            let val: *mut c_void = ((*_self).tsd).add(i).read() as *mut c_void;
            let dtor = keys[i];
            (*_self).tsd.add(i).write(core::ptr::null_mut());
            if !val.is_null() && fn_addr_eq(dtor, nodtor as unsafe extern "C" fn(*mut c_void)) {
                pthread_rwlock_unlock(ptr::addr_of_mut!(key_lock));
                dtor(val);
                pthread_rwlock_rdlock(ptr::addr_of_mut!(key_lock));
            }
            i += 1;
        }
        pthread_rwlock_unlock(ptr::addr_of_mut!(key_lock));
        j += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __pthread_key_atfork(who: c_int)
{
    if who<0 { pthread_rwlock_rdlock(ptr::addr_of_mut!(key_lock)); }
    else if who>0 { pthread_rwlock_unlock(ptr::addr_of_mut!(key_lock)); }
    else { key_lock = PTHREAD_RWLOCK_INITIALIZER; }
}

#[no_mangle]
pub unsafe extern "C" fn pthread_key_create(k: *mut pthread_key_t, dtor: Option<unsafe extern "C" fn(*mut c_void)>) -> c_int
{
    let mut dtor = dtor;
    let _self = pthread_self();

    if (*_self).tsd.is_null() { (*_self).tsd = ptr::addr_of_mut!(__pthread_tsd_main) as *mut *mut c_void; }

    if dtor.is_none() { dtor = Some(nodtor); }

    pthread_rwlock_wrlock(ptr::addr_of_mut!(key_lock));
    let mut j = next_key;
    loop {
        if fn_addr_eq(keys[j as usize], nodtor as unsafe extern "C" fn(*mut c_void)) {
            *k = j;
            next_key = j;
            keys[j as usize] = dtor.unwrap();
            pthread_rwlock_unlock(ptr::addr_of_mut!(key_lock));
            return 0;
        }
        j = (j + 1) % (PTHREAD_KEYS_MAX as pthread_key_t);
        if j == next_key { break; }
    }

    pthread_rwlock_unlock(ptr::addr_of_mut!(key_lock));

    EAGAIN
}

#[no_mangle]
pub unsafe extern "C" fn pthread_key_delete(k: pthread_key_t) -> c_int
{
    let mut set: sigset_t = core::mem::zeroed();
    let _self = pthread_self();
    let mut td = _self;

    __block_app_sigs(ptr::addr_of_mut!(set) as *mut c_void);
    pthread_rwlock_wrlock(ptr::addr_of_mut!(key_lock));

    __tl_lock();
    loop {
        (*td).tsd.add(k as usize).write(core::ptr::null_mut());
        td = (*td).next;
        if td==_self { break; }
    }

    (*td).tsd.add(k as usize).write(core::ptr::null_mut());

    pthread_rwlock_unlock(ptr::addr_of_mut!(key_lock));
    __restore_sigs(ptr::addr_of_mut!(set) as *mut c_void);

    0
}