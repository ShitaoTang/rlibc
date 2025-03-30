use crate::include::ctype::*;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
use core::arch::asm;

/* The following is not for aarch64 and x86_64 */
/*
static mut unmap_base: *mut c_void = core::ptr::null_mut();
static mut unmap_size: size_t = 0;
static mut shared_stack: [c_char; 256] = [0; 256];

#[no_mangle]
unsafe fn do_unmap()
{
    __syscall2(SYS_munmap as c_long, unmap_base as c_long, unmap_size as c_long);
    __syscall0(SYS_exit as c_long);
}

#[no_mangle]
pub unsafe extern "C" fn __unmapself(base: *mut c_void, size: size_t)
{
    let share_stack_ptr = core::ptr::addr_of_mut!(shared_stack) as *mut c_void;
    let mut stack: *mut c_char = share_stack_ptr
        .add(256*(core::mem::size_of::<c_char> as usize)) as *mut c_char;
    stack = stack.sub((stack as uintptr_t) % 16);
    unmap_base = base;
    unmap_size = size;
    #[cfg(target_arch = "x86_64")]
    asm!(
        "mov rsp, {0}",
        "jmp {1}",

        in (reg) do_unmap,
        in (reg) stack,
        clobber_abi("C"), 
        options(noreturn)
    );
    #[cfg(target_arch = "aarch64")]
    asm!(
        "mov sp, {0}",
        "br {1}",

        in (reg) do_unmap,
        in (reg) stack,
        clobber_abi("C"), 
        options(noreturn)
    );
}
*/

#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub unsafe extern "C" fn __unmapself(_base: *mut c_void, _size: size_t)
{
    asm!(
        "mov x8, 215",  // SYS_munmap
        "svc 0",
        "mov x8, 93",   // SYS_exit
        "svc 0",
    );
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub unsafe extern "C" fn __unmapself(_base: *mut c_void, _size: size_t)
{
    asm!(
        "mov rax, 11",  // SYS_munmap
        "syscall",
        "xor rdi, rdi"
        "mov rax, 60",  // SYS_exit
        "syscall",
    );
}