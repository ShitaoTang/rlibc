use crate::include::ctype::*;
use super::_Exit::*;

fn __funcs_on_exit()
{
}

fn __stdio_exit()
{
}

extern "C" {
    static __init_array_start: extern "C" fn();
    static __init_array_end: extern "C" fn();

    fn _fini();
}

#[no_mangle]
pub unsafe fn libc_exit_fini()
{
    let mut a: *const extern "C" fn() = &__init_array_end as *const _;
    while a > &__init_array_start as *const _ {
        a = a.offset(-1);
        (*a)();
    }

    _fini();
}

#[no_mangle]
pub unsafe extern "C" fn exit(code: c_int) -> !
{
    __funcs_on_exit();
    libc_exit_fini();
    __stdio_exit();
    _Exit(code);
}
