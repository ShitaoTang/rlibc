use crate::include::ctype::*;

use crate::internal::locale_impl::__locale_struct;

#[repr(C)]
pub struct tls_module {
    pub next: *mut tls_module,
    pub image: *mut c_void,
    pub len: size_t,
    pub size: size_t,
    pub align: size_t,
    pub offset: size_t,
}

#[repr(C)]
pub struct __libc {
    pub can_do_threads: c_char,
    pub threaded: c_char,
    pub secure: c_char,
    pub need_locks: c_schar,    // volatile
    pub threads_minus_1: c_int,
    pub auxv: *mut size_t,
    pub tls_head: *mut tls_module,
    pub tls_size: size_t,
    pub tls_align: size_t,
    pub tls_cnt: size_t,
    pub page_size: size_t,
    pub global_locale: __locale_struct,
}

#[no_mangle]
pub static mut libc: __libc = unsafe {core::mem::zeroed()};

#[no_mangle]
pub static mut __hwcap: size_t = 0;
#[no_mangle]
pub static mut __progname: *mut c_char = core::ptr::null_mut();
#[no_mangle]
pub static mut __progname_full: *mut c_char = core::ptr::null_mut();