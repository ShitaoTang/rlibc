use core::ptr;
use crate::arch::generic::bits::errno::ENOMEM;
use crate::include::ctype::*;
use crate::malloc::lite_malloc::__libc_malloc;
use crate::internal::lock::*;

#[repr(C)]
struct atfork_funcs {
    prepare: Option<extern "C" fn()>,
    parent: Option<extern "C" fn()>,
    child: Option<extern "C" fn()>,
    prev: *mut atfork_funcs,
    next: *mut atfork_funcs,
}

static mut funcs: *mut atfork_funcs = ptr::null_mut();

static mut lock: [c_int; 1] = [0];    // volatile

#[no_mangle]
pub unsafe extern "C" fn __fork_handler(who: c_int)
{
    let mut p: *mut atfork_funcs;
    if funcs.is_null() { return; }
    if who < 0 {
        LOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
        p = funcs;
        while !p.is_null() {
            if let Some(func) = (*p).prepare {
                func();
            }
            funcs = p;
            p = (*p).next;
        }
    } else {
        p = funcs;
        while !p.is_null() {
            if who==0 { if let Some(pfunc) = (*p).parent {
                pfunc();
            }} else if who!=0 { if let Some(cfunc) = (*p).child {
                cfunc();
            }}
            funcs = p;
            p = (*p).prev;
        }
        UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pthread_atfork(
    prepare: Option<extern "C" fn()>,
    parent: Option<extern "C" fn()>,
    child: Option<extern "C" fn()>,
) -> c_int {
    let new = __libc_malloc(core::mem::size_of::<atfork_funcs>()) as *mut atfork_funcs;
    if new.is_null() { return ENOMEM; }

    LOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
    (*new).prepare = prepare;
    (*new).parent = parent;
    (*new).child = child;
    (*new).prev = ptr::null_mut();
    (*new).next = funcs;
    if !funcs.is_null() {
        (*funcs).prev = new;
    }
    funcs = new;
    UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);

    0
}