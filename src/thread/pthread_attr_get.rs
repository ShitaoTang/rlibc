use crate::include::ctype::*;
use crate::include::sched::*;
use crate::arch::generic::bits::errno::EINVAL;
use super::*;

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getdetachstate(a: *const pthread_attr_t, state: *mut c_int) -> c_int
{
    if a.is_null() || state.is_null() {
        return EINVAL;
    }

    *state = (*a).__u.__i[3*__SU+0];
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getguardsize(a: *const pthread_attr_t, size: *mut size_t) -> c_int
{
    if a.is_null() || size.is_null() {
        return EINVAL;
    }

    *size = (*a).__u.__s[1] as size_t;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getinheritsched(a: *const pthread_attr_t, inherit: *mut c_int) -> c_int
{
    if a.is_null() || inherit.is_null() {
        return EINVAL;
    }

    *inherit = (*a).__u.__i[3*__SU+1];
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getschedpolicy(a: *const pthread_attr_t, policy: *mut c_int) -> c_int
{
    if a.is_null() || policy.is_null() {
        return EINVAL;
    }

    *policy = (*a).__u.__i[3*__SU+2];
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getschedparam(a: *const pthread_attr_t, param: *mut sched_param) -> c_int
{
    if a.is_null() || param.is_null() {
        return EINVAL;
    }

    (*param).sched_priority = (*a).__u.__i[3*__SU+3];
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getscope(a: *const pthread_attr_t, scope: *mut c_int) -> c_int
{
    if a.is_null() || scope.is_null() {
        return EINVAL;
    }

    *scope = PTHREAD_SCOPE_SYSTEM;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getstack(a: *const pthread_attr_t, addr: *mut *mut c_void, size: *mut size_t) -> c_int
{
    if a.is_null() || addr.is_null() || size.is_null() {
        return EINVAL;
    }

    *size = (*a).__u.__s[0] as size_t;
    *addr = ((*a).__u.__s[2] as *mut c_void).sub(*size);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_getstacksize(a: *const pthread_attr_t, size: *mut size_t) -> c_int
{
    if a.is_null() || size.is_null() {
        return EINVAL;
    }

    *size = (*a).__u.__s[0] as size_t;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_barrierattr_getpshared(a: *const pthread_barrierattr_t, pshared: *mut c_int) -> c_int
{
    if a.is_null() || pshared.is_null() {
        return EINVAL;
    }

    *pshared = if (*a).__attr==0 {0} else {1};
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_condattr_getclock(a: *const pthread_condattr_t, clock: *mut clockid_t) -> c_int
{
    if a.is_null() || clock.is_null() {
        return EINVAL;
    }

    *clock = ((*a).__attr & 0x7fffffff) as clockid_t;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_condattr_getpshared(a: *const pthread_condattr_t, pshared: *mut c_int) -> c_int
{
    if a.is_null() || pshared.is_null() {
        return EINVAL;
    }

    *pshared = ((*a).__attr>>31) as c_int;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_mutexattr_getprotocol(a: *const pthread_mutexattr_t, protocol: *mut c_int) -> c_int
{
    if a.is_null() || protocol.is_null() {
        return EINVAL;
    }

    *protocol = ((*a).__attr / 8) as c_int % 2;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_mutexattr_getpshared(a: *const pthread_mutexattr_t, pshared: *mut c_int) -> c_int
{
    if a.is_null() || pshared.is_null() {
        return EINVAL;
    }

    *pshared = ((*a).__attr / 128) as c_int;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_mutexattr_getrobust(a: *const pthread_mutexattr_t, robust: *mut c_int) -> c_int
{
    if a.is_null() || robust.is_null() {
        return EINVAL;
    }

    *robust = ((*a).__attr / 4) as c_int % 2;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_mutexattr_gettype(a: *const pthread_mutexattr_t, _type: *mut c_int) -> c_int
{
    if a.is_null() || _type.is_null() {
        return EINVAL;
    }

    *_type = ((*a).__attr & 3) as c_int;
    0
}

#[no_mangle]
pub unsafe extern "C" fn pthread_rwlockattr_getpshared(a: *const pthread_rwlockattr_t, pshared: *mut c_int) -> c_int
{
    if a.is_null() || pshared.is_null() {
        return EINVAL;
    }

    *pshared = (*a).__attr[0] as c_int;
    0
}