use crate::include::ctype::c_uint;
use super::pthread_impl::*;

pub static mut __default_stack_size: c_uint = DEFAULT_STACK_SIZE;
pub static mut __default_guard_size: c_uint = DEFAULT_GUARD_SIZE;