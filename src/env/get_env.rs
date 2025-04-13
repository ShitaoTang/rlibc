use super::__environ;
use crate::include::ctype::*;
use crate::string::strncmp::*;
use crate::string::strchrnul::*;
use core::ptr;

#[no_mangle]
pub unsafe extern "C" fn getenv(name: *const c_char) -> *mut c_char
{
    let l: size_t = strchrnul(name, '=' as c_int).offset_from(name) as size_t;
    if l!=0 && name.add(l).read()==0 && __environ::environ!=ptr::null_mut() {
        let mut e: *mut *mut c_char = __environ::environ;
        while *e!=ptr::null_mut() {
            if strncmp(name, *e as *const c_char, l)==0 && *(*e as *const c_char).add(l) == '=' as c_char {
                // may should add '\0' in the end?
                return (*e as *const c_char).add(l+1) as *mut c_char;
            }
            e = e.add(1);
        }
    }

    ptr::null_mut()
}