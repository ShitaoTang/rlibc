use super::ctype::*;
use super::signal::*;

#[repr(C)]
pub struct aiocb {
    pub aio_fildes: c_int,
    pub aio_lio_opcode: c_int,
    pub aio_reqprio: c_int,
    pub aio_buf: *mut c_void,   // volatile
    pub aio_nbytes: size_t,
    pub aio_sigevent: sigevent,
    pub __td: *mut c_void,
    pub lock: [c_int; 2],
    pub __err: c_int,           // volatile
    pub __ret: ssize_t,
    pub aio_offset: off_t,
    pub __next: *mut c_void,
    pub __prev: *mut c_void,
    dummy4: [c_char; 32 - 2*size_of::<*mut c_void>()],
}

pub const AIO_CANCELED:     c_int = 0;
pub const AIO_NOTCANCELED:  c_int = 1;
pub const AIO_ALLDONE:      c_int = 2;

pub const LIO_READ:  c_int = 0;
pub const LIO_WRITE: c_int = 1;
pub const LIO_NOP:   c_int = 2;

pub const LIO_WAIT:   c_int = 0;
pub const LIO_NOWAIT: c_int = 1;