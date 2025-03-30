use super::ctype::*;

pub const NULL: *mut c_void = 0 as *mut c_void;

pub const EOF: c_int = -1;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const _IOFBF: c_int = 0;
pub const _IOLBF: c_int = 1;
pub const _IONBF: c_int = 2;

pub const BUFSIZ: size_t = 1024;
pub const FILENAME_MAX: size_t = 4096;
pub const FOPEN_MAX: size_t = 1000;
pub const TMP_MAX: size_t = 10000;
pub const L_tmpnam: size_t = 20;