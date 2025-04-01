use crate::include::ctype::*;
use crate::include::sched::*;
use crate::arch::generic::bits::errno::EINVAL;

#[no_mangle]
pub unsafe extern "C" fn pthread_attr_setschedparam(a: *mut pthread_attr_t, param: *const sched_param) -> c_int
{
    if a.is_null() || param.is_null() {
        return EINVAL;
    }

    (*a).__u.__i[3*__SU+3] = (*param).sched_priority;
    0
}