use crate::include::ctype::*;

extern "C" {
    pub fn __clone(
        func: unsafe extern "C" fn(*mut c_void) -> c_int,
        stack: *mut c_void,
        flags: c_int,
        args: *mut c_void,
        ptid: *mut c_int,
        tls: *mut c_void,
        ctid: *mut c_int,
    ) -> c_int;
}