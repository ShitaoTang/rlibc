use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::SYS_brk;
use crate::__syscall;
use crate::include::ctype::*;
use crate::include::libc;
use crate::include::sys::mman::*;
use crate::internal::lock::*;
use crate::mman::mmap::__mmap;
use crate::thread::pthread_self::pthread_self;
use crate::arch::generic::bits::errno::*;
use crate::include::limits::*;
use core::ptr;

const ALIGN: size_t = 16;

static mut lock: [c_int; 1] = [0];    // volatile
const __bump_lockptr: *mut c_int = unsafe { ptr::addr_of_mut!(lock[0]) as *mut c_int };

static mut brk: uintptr_t = 0;
static mut end: uintptr_t = 0;
static mut cur: uintptr_t = 0;
static mut mmap_step: c_uint = 0;

#[no_mangle]
unsafe fn traverses_stack_p(old: uintptr_t, new: uintptr_t) -> bool
{
    let len: uintptr_t = 8<<20;
    let mut a: uintptr_t;
    let mut b: uintptr_t;

    b = libc::libc.auxv as uintptr_t;
    a = if b>len {b-len} else {0};
    if new>a && old<b { return true; }

    b = &b as *const _ as uintptr_t;
    a = if b>len {b-len} else {0};
    if new>a && old<b { return true; }

    false
}

#[no_mangle]
pub unsafe extern "C" fn __simple_malloc(n: size_t) -> *mut c_void
{
    let mut align: size_t = 1;
    let p: *mut c_void;
    let mut n = n;

    if n > (usize::MAX/2) {
        (*pthread_self()).errno_val = ENOMEM;
        return ptr::null_mut();
    }

    if n==0 { n+=1; }
    while align<n && align<ALIGN {
        align += align;
    }

    LOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);

    cur += cur.wrapping_neg() & (align - 1);

    if n > end-cur {
        let mut req = (n - (end-cur)) + (PAGE_SIZE-1) & PAGE_SIZE.wrapping_neg();

        if cur == 0 {
            // brk = __syscall1(SYS_brk as c_long, 0) as uintptr_t;
            brk = __syscall!(SYS_brk, 0) as uintptr_t;
            brk += brk.wrapping_neg() & PAGE_SIZE - 1;
            cur = brk;
            end = brk;
        }

        if brk==end && req < usize::MAX-brk
            && !traverses_stack_p(brk, brk+req)
            // && __syscall1(SYS_brk as c_long, (brk+req) as c_long) as usize == brk+req{
            && __syscall!(SYS_brk, (brk+req) as c_long) as usize == brk+req {
            end += req;
            brk = end;
        } else {
            let mut new_area = 0;
            req = (n + PAGE_SIZE - 1) & PAGE_SIZE.wrapping_neg();
            if req-n > req/8 {
                let min = PAGE_SIZE<<(mmap_step/2);
                if min-n > end-cur {
                    if req < min {
                        req = min;
                        if mmap_step < 12 {
                            mmap_step += 1;
                        }
                    }
                    new_area = 1;
                }
            }
            let mem: *mut c_void = __mmap(ptr::null_mut(), req, (PROT_READ|PROT_WRITE) as c_int,
                (MAP_PRIVATE|MAP_ANONYMOUS) as c_int, -1, 0);
            if mem == MAP_FAILED || new_area == 0 {
                UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);
                return if mem==MAP_FAILED {ptr::null_mut()} else {mem}
            }
            cur = mem as uintptr_t;
            end = cur + req;
        }
    }

    p = cur as *mut c_void;
    cur += n;
    UNLOCK(ptr::addr_of_mut!(lock) as *mut _ as *mut c_int);

    p
}

#[no_mangle]
pub unsafe extern "C" fn __libc_malloc(n: size_t) -> *mut c_void
{
    __simple_malloc(n)
}

#[no_mangle]
pub unsafe extern "C" fn default_malloc(n: size_t) -> *mut c_void
{
    __simple_malloc(n)
}