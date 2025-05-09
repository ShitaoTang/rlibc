use core::ptr;
use crate::arch::generic::bits::fcntl::*;
use crate::__syscall;
use crate::arch::syscall_arch::*;
use crate::arch::syscall_bits::*;
use crate::include::ctype::*;
use crate::include::fcntl::*;
use crate::include::sys::mman::*;
use crate::internal::syscall_ret::*;
use crate::mman::mmap::*;
use crate::stat::fstat::*;

#[no_mangle]
pub unsafe fn __map_file(pathname: *const c_char, size: &mut size_t) -> *const c_uchar
{
    let mut st = stat::new();
    let mut map= MAP_FAILED as *const c_uchar;

    #[cfg(target_arch="x86_64")]
    // let fd = __syscall_ret(__syscall2(SYS_open as c_long,
    //     pathname as c_long, (O_RDONLY|O_CLOEXEC|O_NONBLOCK) as c_long) as c_ulong) as c_int;
    let fd = __syscall_ret(__syscall!(SYS_open, pathname, (O_RDONLY|O_CLOEXEC|O_NONBLOCK)) as c_ulong) as c_int;
    #[cfg(target_arch="aarch64")]
    // let fd = __syscall_ret(__syscall3(SYS_openat as c_long,
    //     AT_FDCWD as c_long, pathname as c_long, (O_RDONLY|O_CLOEXEC|O_NONBLOCK) as c_long) as c_ulong) as c_int;
    let fd = __syscall_ret(
        __syscall!(SYS_openat, AT_FDCWD, pathname, (O_RDONLY|O_CLOEXEC|O_NONBLOCK)) as c_ulong
    ) as c_int;
    if fd < 0 { return ptr::null(); }
    if fstat(fd, &mut st) == 0 {
        map = __mmap(ptr::null_mut(), st.st_size as size_t,
            PROT_READ as c_int, MAP_SHARED as c_int, fd, 0) as *const c_uchar;
        *size = st.st_size as size_t;
    }
    // __syscall1(SYS_close as c_long, fd as c_long);
    __syscall!(SYS_close, fd);

    if map == MAP_FAILED as *const c_uchar { ptr::null() }
    else { map }
}