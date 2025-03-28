use crate::arch::*;
use crate::include::ctype::*;
use crate::include::libc::tls_module;
use crate::internal::defsysinfo;
use crate::internal::dynlink::Phdr;
use crate::string::memcpy::*;
use crate::thread::default_attr::__default_stack_size;
use crate::thread::pthread_impl::*;
use crate::thread::__set_thraed_area::*;
use crate::include::libc;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
use crate::arch::atomic_arch::a_crash;
use crate::include::elf::*;
use crate::include::sys::mman::*;

pub static mut __thread_list_lock: c_int = 0;   // volatile

#[no_mangle]
pub unsafe extern "C" fn __init_tp(p: *mut c_void) -> c_int
{
    let td: pthread_t = p as pthread_t;
    (*td)._self = td;
    let r = __set_thread_area(TP_ADJ(p));
    if r<0 { return -1; }
    if r!=0 { libc::libc.can_do_threads = 1 };

    (*td).detach_state = DT_STATUS::DT_JOINABLE as c_int;
    (*td).tid = __syscall1(SYS_set_tid_address as c_long,
        core::ptr::addr_of_mut!(__thread_list_lock) as c_long) as c_int;
    (*td).locale = core::ptr::addr_of_mut!(libc::libc.global_locale) as *const __locale_struct as *mut __locale_struct;
    (*td).robust_list.head = &mut (*td).robust_list.head as *mut _ as *mut c_void;
    (*td).sysinfo = defsysinfo::__sysinfo;
    (*td).next = td;
    (*td).prev = td;

    0
}

#[repr(C)]
pub struct builtin_tls {
    pub c: c_char,
    pub pt: pthread,
    pub space: [*mut c_void; 16],
}

#[no_mangle]
pub static mut builtin_tls: builtin_tls = unsafe {core::mem::zeroed()};

const MIN_TLS_ALIGN: size_t = core::mem::offset_of!(builtin_tls, pt) as size_t;

#[no_mangle]
static mut main_tls: tls_module = unsafe {core::mem::zeroed()};

#[no_mangle]
pub unsafe extern "C" fn __copy_tls(mem: *mut c_uchar) -> *mut c_void
{
    let td: pthread_t;
    let mut p: *mut tls_module;
    let mut i: size_t;
    let dtv: *mut uintptr_t;
    let mut mem = mem;

if TLS_ABOVE_TP
{
    dtv = mem.offset(libc::libc.tls_size as isize).sub(libc::libc.tls_cnt + 1) as *mut uintptr_t;

    mem = mem.offset(-((mem as uintptr_t + core::mem::size_of::<pthread>() & (libc::libc.tls_align.wrapping_sub(1))) as isize));
    td = mem as pthread_t;
    mem = mem.add(core::mem::size_of::<pthread>());

    i = 1; p = libc::libc.tls_head;
    while !p.is_null() {
        *dtv.add(i) = (mem as uintptr_t + (*p).offset) as uintptr_t + DTP_OFFSET;
        memcpy(
            mem.offset((*p).offset as isize) as *mut c_void,
            (*p).image as *const c_void,
            (*p).len
        );
        i += 1;
        p = (*p).next;
    }

} else {
    dtv = mem as *mut uintptr_t;

    mem = mem.offset(libc::libc.tls_size as isize - core::mem::size_of::<pthread>() as isize);
    mem = mem.offset(-((mem as uintptr_t & (libc::libc.tls_align.wrapping_sub(1))) as isize));
    td = mem as pthread_t;

    i = 1; p = libc::libc.tls_head;
    while !p.is_null() {
        *dtv.add(i) = (mem as uintptr_t - (*p).offset) as uintptr_t + DTP_OFFSET;
        memcpy(
            mem.offset(-((*p).offset as isize)) as *mut c_void,
            (*p).image as *const c_void,
            (*p).len
        );
        i += 1;
        p = (*p).next;
    }
}
    *dtv = libc::libc.tls_cnt;
    (*td).dtv = dtv;
    td as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn init_tls(aux: *mut size_t)
{
    let mut p: *mut c_uchar;
    let mut n: size_t;
    let mut phdr: *mut Phdr;
    let mut tls_phdr: *mut Phdr = core::ptr::null_mut();
    let mut base: size_t = 0;
    let mem: *mut c_void;

    p = aux.add(AT_PHDR).read() as *mut c_uchar;
    n = aux.add(AT_PHNUM).read();
    while n != 0 {
        phdr = p as *mut Phdr;
        if (*phdr).p_type == PT_PHDR {
            base = aux.add(AT_PHDR).read() - (*phdr).p_vaddr as size_t;
        }
        /* not support yet */
        if (*phdr).p_type == PT_DYNAMIC {
            core::panic!("PT_DYNAMIC not support yet");
        }
        if (*phdr).p_type == PT_TLS {
            tls_phdr = phdr;
        }
        if (*phdr).p_type == PT_GNU_STACK &&
           (*phdr).p_memsz > __default_stack_size {
            __default_stack_size = 
                if (*phdr).p_memsz < DEFAULT_STACK_SIZE { (*phdr).p_memsz }
                else { DEFAULT_STACK_SIZE };
        }
        p = p.add(aux.add(AT_PHENT).read() as size_t);
        n -= 1;
    }

    if !tls_phdr.is_null() {
        main_tls.image = (base + (*tls_phdr).p_vaddr as size_t) as *mut c_void;
        main_tls.len = (*tls_phdr).p_filesz as size_t;
        main_tls.size = (*tls_phdr).p_memsz as size_t;
        main_tls.align = (*tls_phdr).p_align as size_t;
        libc::libc.tls_cnt = 1;
        libc::libc.tls_head = core::ptr::addr_of_mut!(main_tls) as *const tls_module as *mut tls_module;
    }

    main_tls.size += ((main_tls.size).wrapping_neg().wrapping_sub(main_tls.image as uintptr_t))
        & (main_tls.align.wrapping_sub(1));

if TLS_ABOVE_TP {
    main_tls.offset = GAP_ABOVE_TP;
    main_tls.offset += GAP_ABOVE_TP.wrapping_neg().wrapping_add(main_tls.image as uintptr_t)
        & (main_tls.align.wrapping_sub(1));
} else {
    main_tls.offset = main_tls.size;
}

    if main_tls.align < MIN_TLS_ALIGN {
        main_tls.align = MIN_TLS_ALIGN;
    }

    libc::libc.tls_align = main_tls.align;
if TLS_ABOVE_TP {
    libc::libc.tls_size = 2*core::mem::size_of::<uintptr_t>() + core::mem::size_of::<pthread>()
        + main_tls.offset
        + main_tls.size + main_tls.align
        + MIN_TLS_ALIGN-1 & MIN_TLS_ALIGN.wrapping_neg();
} else {
    libc::libc.tls_size = 2*core::mem::size_of::<uintptr_t>() + core::mem::size_of::<pthread>()
        + main_tls.size + main_tls.align
        + MIN_TLS_ALIGN-1 & MIN_TLS_ALIGN.wrapping_neg();
}
    
    if libc::libc.tls_size > core::mem::size_of::<builtin_tls>() as size_t {
#[cfg(target_os = "linux")]
const SYS_mmap2: c_long = SYS_mmap as c_long;
        mem = __syscall6(
            SYS_mmap2,
            0, libc::libc.tls_size as c_long,
            PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1, 0) as *mut c_void;
    } else {
        mem = core::ptr::addr_of_mut!(builtin_tls) as *const _ as *mut c_void;
    }

    if __init_tp(__copy_tls(mem as *mut c_uchar)) < 0 {
        a_crash();
    }

    
}