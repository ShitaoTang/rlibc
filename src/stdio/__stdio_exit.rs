use core::ptr;
use crate::include::ctype::*;
use crate::include::fcntl::SEEK_CUR;
use crate::stdio::__lockfile::*;
use super::ofl::__ofl_lock;
use super::stdin;
use super::stdout;
use super::stderr;

unsafe fn close_file(f: *mut FILE)
{
    if f.is_null() { return; }

    if (*f).lock >= 0 { __lockfile(f); }
    
    if (*f).wpos != (*f).wbase {
        if let Some(write) = (*f).write {
            write(f, ptr::null_mut(), 0);
        }
    }

    if (*f).rpos != (*f).rend {
        if let Some(seek) = (*f).seek {
            seek(f, (*(*f).rpos - *(*f).rend) as c_long, SEEK_CUR);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __stdio_exit()
{
    let mut f = *__ofl_lock();
    while !f.is_null() {
        close_file(f);
        f = (*f).next;
    }
    close_file(stdin::__stdin_used);
    close_file(stdout::__stdout_used);
    close_file(stderr::__stderr_used);
}