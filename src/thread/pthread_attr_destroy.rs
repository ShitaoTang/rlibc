use crate::include::ctype::*;
use crate::arch::generic::bits::errno::EINVAL;

#[no_mangle]
pub extern "C" fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int {
    if attr.is_null() { return EINVAL; }
    
    0
}