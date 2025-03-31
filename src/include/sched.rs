use crate::include::ctype::*;

#[repr(C)]
pub struct reversed2_s {
    pub __reversed1: time_t,
    pub __reversed2: c_long,
}

#[repr(C)]
pub struct sched_param {
    pub sched_priority: c_int,
    pub __reserved: c_int,
    #[cfg(target_arch = "x86_64")]
    pub __reserved2: [reversed2_s; 2],
    #[cfg(target_arch = "aarch64")]
    pub __reserved2: [reversed2_s; 2],
    pub __reserved3: c_int,
}

pub const SCHED_OTHER: c_int = 0;
pub const SCHED_FIFO: c_int = 1;
pub const SCHED_RR: c_int = 2;
pub const SCHED_BATCH: c_int = 3;
pub const SCHED_IDLE: c_int = 5;
pub const SCHED_DEADLINE: c_int = 6;
pub const SCHED_RESET_ON_FORK: c_int = 0x40000000;

pub const CSIGNAl:       c_uint = 0x000000ff;
pub const CLONE_NEWTIME: c_uint = 0x00000080;
pub const CLONE_VM:      c_uint = 0x00000100;
pub const CLONE_FS:      c_uint = 0x00000200;
pub const CLONE_FILES:   c_uint = 0x00000400;
pub const CLONE_SIGHAND: c_uint = 0x00000800;
pub const CLONE_PIDFD:   c_uint = 0x00001000;
pub const CLONE_PTRACE:  c_uint = 0x00002000;
pub const CLONE_VFORK:   c_uint = 0x00004000;
pub const CLONE_PARENT:  c_uint = 0x00008000;
pub const CLONE_THREAD:  c_uint = 0x00010000;
pub const CLONE_NEWNS:   c_uint = 0x00020000;
pub const CLONE_SYSVSEM: c_uint = 0x00040000;
pub const CLONE_SETTLS:  c_uint = 0x00080000;
pub const CLONE_PARENT_SETTID:   c_uint = 0x00100000;
pub const CLONE_CHILD_CLEARTID:  c_uint = 0x00200000;
pub const CLONE_DETACHED:    c_uint = 0x00400000;
pub const CLONE_UNTRACED:    c_uint = 0x00800000;
pub const CLONE_CHILD_SETTID:    c_uint = 0x01000000;
pub const CLONE_NEWCGROUP:   c_uint = 0x02000000;
pub const CLONE_NEWUTS:      c_uint = 0x04000000;
pub const CLONE_NEWIPC:      c_uint = 0x08000000;
pub const CLONE_NEWUSER:     c_uint = 0x10000000;
pub const CLONE_NEWPID:      c_uint = 0x20000000;
pub const CLONE_NEWNET:      c_uint = 0x40000000;
pub const CLONE_NEWIO:       c_uint = 0x80000000;