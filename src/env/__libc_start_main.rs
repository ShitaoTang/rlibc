use crate::arch::atomic_arch::a_barrier;
use crate::arch::atomic_arch::a_crash;
use crate::include::ctype::*;
use crate::include::fcntl::*;
use crate::include::poll::pollfd;
use crate::include::poll::POLLNVAL;
#[cfg(target_arch = "aarch64")]
use crate::include::time::timespec;
// use core::sync::atomic::{compiler_fence, Ordering};
use crate::exit::exit::*;
use super::__environ;
use crate::include::libc;
use crate::include::elf::*;
use crate::internal::defsysinfo;
use super::__init_tls::init_tls;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
#[cfg(target_arch = "aarch64")]
use crate::arch::bits::signal::*;

const AUX_CNT: size_t = 38;

fn __init_ssp(_aux: *mut c_void)
{
    // stack protector does not support yet
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn __init_libc(envp: *mut *mut c_char, pn: *mut c_char)
{
    let mut pn = pn;
    let mut i: size_t = 0;
    let auxv: *mut size_t;
    let mut aux: [size_t; AUX_CNT] = [0; AUX_CNT];
    __environ::environ = envp;
    while !(*envp.add(i)).is_null() { i += 1; }
    auxv = envp.add(i+1) as *mut size_t;
    libc::libc.auxv = auxv;

    i = 0;
    while auxv.add(i).read() != 0 {
        if auxv.add(i).read() < AUX_CNT {
            aux[auxv.add(i).read() as usize] = auxv.add(i + 1).read();
        }
        i += 2;
    }
    libc::__hwcap = aux[AT_HWCAP];
    if aux[AT_SYSINFO] != 0 { defsysinfo::__sysinfo = aux[AT_SYSINFO]; }
    libc::libc.page_size = aux[AT_PAGESZ];

    if pn.is_null() { pn = aux[AT_EXECFN] as *mut c_char; }
    if pn.is_null() { pn = "".as_ptr() as *mut c_char; }
    libc::__progname = pn;
    libc::__progname_full = pn;
    i = 0;
    while pn.add(i).read() != 0 {
        if pn.add(i).read() == '/' as c_char {
            libc::__progname = pn.add(i + 1);
        }
        i += 1;
    }

    init_tls(aux.as_mut_ptr());
    // stack protector does not support yet
    __init_ssp(aux[AT_RANDOM] as *mut c_void);

    if aux[AT_UID]==aux[AT_EUID] && aux[AT_GID]==aux[AT_EGID]
        && aux[AT_SECURE] == 0 { return; }
    
    let pfd: [pollfd; 3] = [
        pollfd { fd: 0, events: 0, revents: 0 },
        pollfd { fd: 1, events: 0, revents: 0 },
        pollfd { fd: 2, events: 0, revents: 0 },
    ];

#[cfg(target_arch = "x86_64")]
    let r = __syscall3(SYS_poll as c_long, pfd.as_ptr() as c_long, 3, 0);
#[cfg(target_arch = "aarch64")]
    let r = __syscall5(SYS_ppoll as c_long,
        pfd.as_ptr() as c_long, 3,
        &timespec { tv_sec: 0, tv_nsec: 0 } as *const timespec as c_long, 0, (_NSIG/8) as c_long);
    if r<0 { a_crash(); }
    i = 0;
    while i < 3 {
        if (pfd[i].revents&POLLNVAL) != 0 {
            #[cfg(target_arch = "x86_64")]
            if __syscall2(SYS_open as c_long, "/dev/null\0".as_ptr() as c_long, O_RDWR as c_long) < 0 {
                a_crash();
            }
            #[cfg(target_arch = "aarch64")]
            if __syscall3(SYS_openat as c_long, AT_FDCWD as c_long,
                "/dev/null\0".as_ptr() as c_long, O_RDWR as c_long) < 0 {
                a_crash();
            }
        }
        i += 1;
    }
    libc::libc.secure = 1;
}

extern "C" {
    static __init_array_start: extern "C" fn();
    static __init_array_end: extern "C" fn();

    fn _init();
}

// this is for static linking
// weak_alias(libc_start_init, __libc_start_init)
#[no_mangle]
pub unsafe extern "C" fn libc_start_init()
{
    _init();
    
    let mut a: *const extern "C" fn() = &__init_array_start as *const _;
    while a < &__init_array_end as *const _ {
        (*a)();
        a = a.offset(1);
    }
}


type lsm2_fn = unsafe extern "C" fn(
    main: extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char) -> c_int,
    argc: c_int,
    argv: *mut *mut c_char,
) -> c_int;
// static mut libc_start_main_stage2: Option<lsm2_fn> = None;

#[no_mangle]
pub unsafe extern "C" fn __libc_start_main(
    main: extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char) -> c_int,
    argc: c_int,
    argv: *mut *mut c_char,
    _init: Option<extern "C" fn()>,
    _fini: Option<extern "C" fn()>,
    _ldso: Option<extern "C" fn()>,
) -> c_int {
    let envp: *mut *mut c_char = argv.add(argc as usize + 1) as *mut *mut c_char;

    __init_libc(envp, *argv as *mut c_char);

    let stage2: lsm2_fn = __libc_start_main_stage2;

    // compiler_fence(Ordering::SeqCst);
    a_barrier();

    unsafe { stage2(main, argc, argv) }
}

#[no_mangle]
pub unsafe extern "C" fn __libc_start_main_stage2(
    main: extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char) -> c_int,
    argc: c_int,
    argv: *mut *mut c_char,
) -> c_int {
    let envp = argv.add(argc as usize + 1) as *mut *mut c_char;
    libc_start_init();

    exit(main(argc, argv, envp));
}

// #[no_mangle]
// pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
//     let mut ptr = s;
//     let val = c as u8;
//     for _ in 0..n {
//         *ptr = val;
//         ptr = ptr.add(1);
//     }
//     s
// }

#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        if *s1.add(i) != *s2.add(i) {
            return 1;
        }
    }
    0
}