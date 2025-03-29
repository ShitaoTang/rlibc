use crate::include::ctype::pthread_rwlock_t;
use super::pthread_rwlock_wrlock::*;
use super::pthread_rwlock_rdlock::*;
use super::pthread_rwlock_unlock::*;
use core::ptr;

static mut lock: pthread_rwlock_t = unsafe { core::mem::zeroed() };

#[no_mangle]
pub fn __inhibit_ptc()
{
    pthread_rwlock_wrlock(ptr::addr_of_mut!(lock));
}

#[no_mangle]
pub fn __acquire_ptc()
{
    pthread_rwlock_rdlock(ptr::addr_of_mut!(lock));
}

#[no_mangle]
pub fn __release_ptc()
{
    pthread_rwlock_unlock(ptr::addr_of_mut!(lock));
}