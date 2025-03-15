use core::arch::asm;
use libc::c_long;

#[inline(always)]
pub unsafe fn __syscall0(n: c_long) -> c_long {
    let mut ret: c_long;
    asm!(
        "syscall",
        in("rax") n,
        lateout("rax") ret,
        clobber_abi("sysv64")
    );
    ret
}

#[inline(always)]
pub unsafe fn __syscall1(n: c_long, a1: c_long) -> c_long {
    let mut ret: c_long;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        lateout("rax") ret,
        clobber_abi("sysv64")
    );
    ret
}

#[inline(always)]
pub unsafe fn __syscall2(n: c_long, a1: c_long, a2: c_long) -> c_long {
    let mut ret: c_long;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        lateout("rax") ret,
        clobber_abi("sysv64")
    );
    ret
}

#[inline(always)]
pub unsafe fn __syscall3(n: c_long, a1: c_long, a2: c_long, a3: c_long) -> c_long {
    let mut ret: c_long;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        lateout("rax") ret,
        clobber_abi("sysv64")
    );
    ret
}

#[inline(always)]
pub unsafe fn __syscall4(n: c_long, a1: c_long, a2: c_long, a3: c_long, a4: c_long) -> c_long {
    let mut ret: c_long;
    let r10 = a4;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") r10,
        lateout("rax") ret,
        clobber_abi("sysv64")
    );
    ret
}

#[inline(always)]
pub unsafe fn __syscall5(n: c_long, a1: c_long, a2: c_long, a3: c_long, a4: c_long, a5: c_long) -> c_long {
    let mut ret: c_long;
    let r10 = a4;
    let r8 = a5;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") r10,
        in("r8") r8,
        lateout("rax") ret,
        clobber_abi("sysv64")
    );
    ret
}

#[inline(always)]
pub unsafe fn __syscall6(n: c_long, a1: c_long, a2: c_long, a3: c_long, a4: c_long, a5: c_long, a6: c_long) -> c_long {
    let mut ret: c_long;
    let r10 = a4;
    let r8 = a5;
    let r9 = a6;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") r10,
        in("r8") r8,
        in("r9") r9,
        lateout("rax") ret,
        clobber_abi("sysv64")
    );
    ret
}
