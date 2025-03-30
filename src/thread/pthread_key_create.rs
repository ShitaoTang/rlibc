use crate::include::ctype::*;
use crate::include::limits::*;
use core::ptr;
use core::ptr::fn_addr_eq;
use core::sync::atomic::AtomicPtr;
use super::pthread_self::*;
use super::pthread_rwlock_rdlock::*;
use super::pthread_rwlock_unlock::*;

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