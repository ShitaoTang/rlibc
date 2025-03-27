use crate::include::{
    ctype::*,
    libc::*,
};
use crate::internal::dynlink::*;
use crate::arch::generic::bits::link::*;
// use crate::thread::pthread_self::pthread_self;
// use crate::thread::pthread_mutex_lock::*;
// use crate::thread::pthread_mutex_unlock::*;
// use crate::thread::pthread_cond_wait::*;
// use crate::thread::pthread_cond_broadcast::*;

#[repr(C)]
pub struct td_index {
    pub args: [size_t; 2],
    pub next: *mut td_index,
}

#[repr(C)]
pub struct funcdesc {
    pub addr: *mut c_void,
    pub got: *mut size_t,
}

#[repr(C)]
pub struct dso {
    #[cfg(target_arch = "x86_64")]
    pub base: *mut c_uchar,
    #[cfg(target_arch = "aarch64")]
    pub base: *mut c_uchar,
    pub name: *mut c_char,
    pub dynv: *mut size_t,
    pub next: *mut dso,
    pub prev: *mut dso,

    pub phdr: *mut Phdr,
    pub phnum: c_int,
    pub phentsize: size_t,
    pub syms: *mut Sym,
    pub hashtab: *mut Elf_Symndx,
    pub ghashtab: *mut uint32_t,
    pub versym: *mut int16_t,
    pub strings: *mut c_char,
    pub syms_next: *mut dso,
    pub lazy_next: *mut dso,
    pub lazy: *mut size_t,
    pub lazy_cnt: size_t,
    pub map: *mut c_uchar,
    pub map_len: size_t,
    pub dev: dev_t,
    pub ino: ino_t,
    pub relocated: c_char,
    pub constructed: c_char,
    pub kernel_mapped: c_char,
    pub mark: c_char,
    pub bfs_built: c_char,
    pub runtime_loaded: c_char,
    pub deps: *mut *mut dso,
    pub needed_by: *mut dso,
    pub ndeps_direct: size_t,
    pub next_dep: size_t,
    pub ctor_visitor: pthread_t,
    pub rpath_orig: *mut c_char,
    pub rpath: *mut c_char,
    pub tls: tls_module,
    pub tls_id: size_t,
    pub relro_start: size_t,
    pub relro_end: size_t,
    pub new_dtv: *mut uintptr_t,
    pub new_tls: *mut c_uchar,
    pub td_index: *mut td_index,
    pub fini_next: *mut dso,
    pub shortname: *mut c_char,
    #[cfg(target_arch = "x86_64")]
    pub loadmap: *mut fdpic_loadmap,
    #[cfg(target_arch = "aarch64")]
    pub loadmap: *mut fdpic_loadmap,
    pub funcdescs: *mut funcdesc,
    pub got: *mut size_t,
    pub buf: [c_char; 0],
}

pub static mut shutting_down: c_int = 0;
pub static mut init_fini_lock: pthread_mutex_t = unsafe {core::mem::zeroed()};
pub static mut ctor_cond: pthread_cond_t = unsafe {core::mem::zeroed()};

#[no_mangle]
unsafe fn decode_vec(v: *const size_t, a: *mut size_t, cnt: size_t)
{
    for i in 0..cnt { *a.add(i) = 0; }

    let mut v = v;
    let size_long = core::mem::size_of::<c_long>();
    while *v != 0 {
        let index = *v;
        if index-1<cnt-1 {
            if index < 8*size_long {
                *a |= 1<<index;
            }
            *a.add(index) = *v.add(1);
        }
        v = v.add(2);
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn do_init_fini(queue: *mut *mut dso)
// {
//     let mut p: *mut dso;
//     let mut dyn_array: [size_t; DYN_CNT] = [0; DYN_CNT];
//     let mut i: size_t = 0;
//     let mut _self = pthread_self();

//     pthread_mutex_lock(&mut init_fini_lock);
//     p = (*queue).add(1);
//     while !p.is_null() {
//         while (!(*p).ctor_visitor.is_null() && (*p).ctor_visitor != _self) || shutting_down!=0 {
//             pthread_cond_wait(&mut ctor_cond, &mut init_fini_lock);
//         }
//         if !(*p).ctor_visitor.is_null() || (*p).constructed!=0 {
//             continue;
//         }
//         (*p).ctor_visitor = _self;

//         decode_vec((*p).dynv, dyn_array.as_mut_ptr(), DYN_CNT);
//     }
// }

#[no_mangle]
pub unsafe extern "C" fn __dl_libc_start_init()
{
    // do_init_fini(main_ctor_queue);
}