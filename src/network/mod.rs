use crate::include::ctype::*;
use crate::cfg_if;

pub mod accept;
pub mod socket;
pub mod bind;
pub mod listen;
pub mod connect;
pub mod sendto;
pub mod send;
pub mod sendmsg;
pub mod sendmmsg;
pub mod recvfrom;
pub mod recv;
pub mod recvmsg;
pub mod recvmmsg;
pub mod getsockname;
pub mod getpeername;
pub mod getsockopt;
pub mod setsockopt;
pub mod shutdown;
pub mod inet_addr;
pub mod inet_aton;
pub mod inet_ntoa;
pub mod inet_pton;
pub mod inet_ntop;

type sa_family_t = c_ushort;
type socklen_t = c_uint;
type in_port_t = c_ushort;
type in_addr_t = c_uint;

pub const IOV_MAX:      c_int = 1024;

pub const SHUT_RD:      c_int = 0;
pub const SHUT_WR:      c_int = 1;
pub const SHUT_RDWR:    c_int = 2;

pub const SOCK_STREAM:      c_int = 1;
pub const SOCK_DGRAM:       c_int = 2;
pub const SOCK_RAW:         c_int = 3;
pub const SOCK_RDM:         c_int = 4;
pub const SOCK_SEQPACKET:   c_int = 5;
pub const SOCK_DCCP:        c_int = 6;
pub const SOCK_PACKET:      c_int = 10;

pub const SOCK_CLOEXEC:     c_int = 0o2000000;
pub const SOCK_NONBLOCK:    c_int = 0o4000;

// Protocol Family (PF) constants
pub const PF_UNSPEC:        c_int = 0;
pub const PF_LOCAL:         c_int = 1;
pub const PF_UNIX:          c_int = PF_LOCAL;
pub const PF_FILE:          c_int = PF_LOCAL;
pub const PF_INET:          c_int = 2;
pub const PF_AX25:          c_int = 3;
pub const PF_IPX:           c_int = 4;
pub const PF_APPLETALK:     c_int = 5;
pub const PF_NETROM:        c_int = 6;
pub const PF_BRIDGE:        c_int = 7;
pub const PF_ATMPVC:        c_int = 8;
pub const PF_X25:           c_int = 9;
pub const PF_INET6:         c_int = 10;
pub const PF_ROSE:          c_int = 11;
pub const PF_DECnet:        c_int = 12;
pub const PF_NETBEUI:       c_int = 13;
pub const PF_SECURITY:      c_int = 14;
pub const PF_KEY:           c_int = 15;
pub const PF_NETLINK:       c_int = 16;
pub const PF_ROUTE:         c_int = PF_NETLINK;
pub const PF_PACKET:        c_int = 17;
pub const PF_ASH:           c_int = 18;
pub const PF_ECONET:        c_int = 19;
pub const PF_ATMSVC:        c_int = 20;
pub const PF_RDS:           c_int = 21;
pub const PF_SNA:           c_int = 22;
pub const PF_IRDA:          c_int = 23;
pub const PF_PPPOX:         c_int = 24;
pub const PF_WANPIPE:       c_int = 25;
pub const PF_LLC:           c_int = 26;
pub const PF_IB:            c_int = 27;
pub const PF_MPLS:          c_int = 28;
pub const PF_CAN:           c_int = 29;
pub const PF_TIPC:          c_int = 30;
pub const PF_BLUETOOTH:     c_int = 31;
pub const PF_IUCV:          c_int = 32;
pub const PF_RXRPC:         c_int = 33;
pub const PF_ISDN:          c_int = 34;
pub const PF_PHONET:        c_int = 35;
pub const PF_IEEE802154:    c_int = 36;
pub const PF_CAIF:          c_int = 37;
pub const PF_ALG:           c_int = 38;
pub const PF_NFC:           c_int = 39;
pub const PF_VSOCK:         c_int = 40;
pub const PF_KCM:           c_int = 41;
pub const PF_QIPCRTR:       c_int = 42;
pub const PF_SMC:           c_int = 43;
pub const PF_XDP:           c_int = 44;
pub const PF_MAX:           c_int = 45;

// Address Family (AF) constants
pub const AF_UNSPEC:        c_int = PF_UNSPEC;
pub const AF_LOCAL:         c_int = PF_LOCAL;
pub const AF_UNIX:          c_int = AF_LOCAL;
pub const AF_FILE:          c_int = AF_LOCAL;
pub const AF_INET:          c_int = PF_INET;
pub const AF_AX25:          c_int = PF_AX25;
pub const AF_IPX:           c_int = PF_IPX;
pub const AF_APPLETALK:     c_int = PF_APPLETALK;
pub const AF_NETROM:        c_int = PF_NETROM;
pub const AF_BRIDGE:        c_int = PF_BRIDGE;
pub const AF_ATMPVC:        c_int = PF_ATMPVC;
pub const AF_X25:           c_int = PF_X25;
pub const AF_INET6:         c_int = PF_INET6;
pub const AF_ROSE:          c_int = PF_ROSE;
pub const AF_DECnet:        c_int = PF_DECnet;
pub const AF_NETBEUI:       c_int = PF_NETBEUI;
pub const AF_SECURITY:      c_int = PF_SECURITY;
pub const AF_KEY:           c_int = PF_KEY;
pub const AF_NETLINK:       c_int = PF_NETLINK;
pub const AF_ROUTE:         c_int = PF_ROUTE;
pub const AF_PACKET:        c_int = PF_PACKET;
pub const AF_ASH:           c_int = PF_ASH;
pub const AF_ECONET:        c_int = PF_ECONET;
pub const AF_ATMSVC:        c_int = PF_ATMSVC;
pub const AF_RDS:           c_int = PF_RDS;
pub const AF_SNA:           c_int = PF_SNA;
pub const AF_IRDA:          c_int = PF_IRDA;
pub const AF_PPPOX:         c_int = PF_PPPOX;
pub const AF_WANPIPE:       c_int = PF_WANPIPE;
pub const AF_LLC:           c_int = PF_LLC;
pub const AF_IB:            c_int = PF_IB;
pub const AF_MPLS:          c_int = PF_MPLS;
pub const AF_CAN:           c_int = PF_CAN;
pub const AF_TIPC:          c_int = PF_TIPC;
pub const AF_BLUETOOTH:     c_int = PF_BLUETOOTH;
pub const AF_IUCV:          c_int = PF_IUCV;
pub const AF_RXRPC:         c_int = PF_RXRPC;
pub const AF_ISDN:          c_int = PF_ISDN;
pub const AF_PHONET:        c_int = PF_PHONET;
pub const AF_IEEE802154:    c_int = PF_IEEE802154;
pub const AF_CAIF:          c_int = PF_CAIF;
pub const AF_ALG:           c_int = PF_ALG;
pub const AF_NFC:           c_int = PF_NFC;
pub const AF_VSOCK:         c_int = PF_VSOCK;
pub const AF_KCM:           c_int = PF_KCM;
pub const AF_QIPCRTR:       c_int = PF_QIPCRTR;
pub const AF_SMC:           c_int = PF_SMC;
pub const AF_XDP:           c_int = PF_XDP;
pub const AF_MAX:           c_int = PF_MAX;

pub const SO_DEBUG:         c_int = 1;
pub const SO_REUSEADDR:     c_int = 2;
pub const SO_TYPE:          c_int = 3;
pub const SO_ERROR:         c_int = 4;
pub const SO_DONTROUTE:     c_int = 5;
pub const SO_BROADCAST:     c_int = 6;
pub const SO_SNDBUF:        c_int = 7;
pub const SO_RCVBUF:        c_int = 8;
pub const SO_KEEPALIVE:     c_int = 9;
pub const SO_OOBINLINE:     c_int = 10;
pub const SO_NO_CHECK:      c_int = 11;
pub const SO_PRIORITY:      c_int = 12;
pub const SO_LINGER:        c_int = 13;
pub const SO_BSDCOMPAT:     c_int = 14;
pub const SO_REUSEPORT:     c_int = 15;
pub const SO_PASSCRED:      c_int = 16;
pub const SO_PEERCRED:      c_int = 17;
pub const SO_RCVLOWAT:      c_int = 18;
pub const SO_SNDLOWAT:      c_int = 19;
pub const SO_ACCETCONN:     c_int = 30;
pub const SO_PEERSEC:       c_int = 31;
pub const SO_SNDBUFFORCE:   c_int = 32;
pub const SO_RCVBUFFORCE:   c_int = 33;
pub const SO_PROTOCOL:      c_int = 38;
pub const SO_DOMAIN:        c_int = 39;

cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        pub const SO_RCVTIMEO: c_int = 66;
        pub const SO_SNDTIMEO: c_int = 67;
    } else {
        pub const SO_RCVTIMEO: c_int = 20;
        pub const SO_SNDTIMEO: c_int = 21;
    }
}

cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        pub const SO_TIMESTAMP:     c_int = 63;
        pub const SO_TIMESTAMPNS:   c_int = 64;
        pub const SO_TIMESTAMPING:  c_int = 65;
    } else {
        pub const SO_TIMESTAMP:     c_int = 29;
        pub const SO_TIMESTAMPNS:   c_int = 35;
        pub const SO_TIMESTAMPING:  c_int = 37;
    }
}

pub const SO_SECURITY_AUTHENTICATION:        c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT:  c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK:    c_int = 24;

pub const SO_BINDTODEVICE: c_int = 25;

pub const SO_ATTACH_FILTER: c_int = 26;
pub const SO_DETACH_FILTER: c_int = 27;
pub const SO_GET_FILTER:    c_int = SO_ATTACH_FILTER;

pub const SO_PEERNAME:                  c_int = 28;
pub const SCM_TIMESTAMP:                c_int = SO_TIMESTAMP;
pub const SO_PASSSEC:                   c_int = 34;
pub const SCM_TIMESTAMPNS:              c_int = SO_TIMESTAMPNS;
pub const SO_MARK:                      c_int = 36;
pub const scM_TIMESTAMPING:             c_int = SO_TIMESTAMPING;
pub const SO_RXQ_OVFL:                  c_int = 40;
pub const SO_WIFI_STATUS:               c_int = 41;
pub const SCM_WIFI_STATUS:              c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF:                  c_int = 42;
pub const SO_NOFCS:                     c_int = 43;
pub const SO_LOCK_FILTER:               c_int = 44;
pub const SO_SELECT_ERR_QUEUE:          c_int = 45;
pub const SO_BUSY_POLL:                 c_int = 46;
pub const SO_MAX_PACING_RATE:           c_int = 47;
pub const SO_BPF_EXTENSIONS:            c_int = 48;
pub const SO_INCOMING_CPU:              c_int = 49;
pub const SO_ATTACH_BPF:                c_int = 50;
pub const SO_DETACH_BPF:                c_int = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF:     c_int = 51;
pub const SO_ATTACH_REUSEPORT_EBPF:     c_int = 52;
pub const SO_CNX_ADVICE:                c_int = 53;
pub const SCM_TIMESTAMPING_OPT_STATS:   c_int = 54;
pub const SO_MEMINFO:                   c_int = 55;
pub const SO_INCOMING_NAPI_ID:          c_int = 56;
pub const SO_COOKIE:                    c_int = 57;
pub const SCM_TIMESTAMPING_PKTINFO:     c_int = 58;
pub const SO_PEERGROUPS:                c_int = 59;
pub const SO_ZEROCOPY:                  c_int = 60;
pub const SO_TXTIME:                    c_int = 61;
pub const SCM_TXTIME:                   c_int = SO_TXTIME;
pub const SO_BINDTOIFINDEX:             c_int = 62;
pub const SO_DETACH_REUSEPORT_BPF:      c_int = 68;
pub const SO_PREFER_BUSY_POLL:          c_int = 69;
pub const SO_BUSY_POLL_BUDGET:          c_int = 70;

pub const SOL_SOCKET:   c_int = 1;

pub const SOL_IP:       c_int = 0;
pub const SOL_IPV6:     c_int = 41;
pub const SOL_ICMPV6:   c_int = 58;

pub const SOL_RAW:      c_int = 255;
pub const SOL_DECNET:   c_int = 261;
pub const SOL_X25:      c_int = 262;
pub const SOL_PACKET:   c_int = 263;
pub const SOL_ATM:      c_int = 264;
pub const SOL_AAL:      c_int = 265;
pub const SOL_IRDA:     c_int = 266;
pub const SOL_NETBEUI:  c_int = 267;
pub const SOL_LLC:      c_int = 268;
pub const SOL_DCCP:     c_int = 269;
pub const SOL_NETLINK:  c_int = 270;
pub const SOL_TIPC:     c_int = 271;
pub const SOL_RXRPC:    c_int = 272;
pub const SOL_PPPOL2TP: c_int = 273;
pub const SOL_BLUETOOTH:c_int = 274;
pub const SOL_PNPIPE:   c_int = 275;
pub const SOL_RDS:      c_int = 276;
pub const SOL_IUCV:     c_int = 277;
pub const SOL_CAIF:     c_int = 278;
pub const SOL_ALG:      c_int = 279;
pub const SOL_NFC:      c_int = 280;
pub const SOL_KCM:      c_int = 281;
pub const SOL_TLS:      c_int = 282;
pub const SOL_XDP:      c_int = 283;

pub const SOMAXCONN: c_int = 128;

pub const MSG_OOB:         c_int = 0x0001;
pub const MSG_PEEK:        c_int = 0x0002;
pub const MSG_DONTROUTE:   c_int = 0x0004;
pub const MSG_CTRUNC:      c_int = 0x0008;
pub const MSG_PROXY:       c_int = 0x0010;
pub const MSG_TRUNC:       c_int = 0x0020;
pub const MSG_DONTWAIT:    c_int = 0x0040;
pub const MSG_EOR:         c_int = 0x0080;
pub const MSG_WAITALL:     c_int = 0x0100;
pub const MSG_FIN:         c_int = 0x0200;
pub const MSG_SYN:         c_int = 0x0400;
pub const MSG_CONFIRM:     c_int = 0x0800;
pub const MSG_RST:         c_int = 0x1000;
pub const MSG_ERRQUEUE:    c_int = 0x2000;
pub const MSG_NOSIGNAL:    c_int = 0x4000;
pub const MSG_MORE:        c_int = 0x8000;
pub const MSG_WAITFORONE:  c_int = 0x10000;
pub const MSG_BATCH:       c_int = 0x40000;
pub const MSG_ZEROCOPY:    c_int = 0x4000000;
pub const MSG_FASTOPEN:    c_int = 0x20000000;
pub const MSG_CMSG_CLOEXEC:c_int = 0x40000000;

pub const SCM_RIGHTS:      c_int = 0x01;
pub const SCM_CREDENTIALS: c_int = 0x02;

#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

// 2B+14B=16B, better for alignment
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [uint8_t; 8],
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "big"))]
    pub __pad1: c_int,
    pub msg_iovlen: c_int,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "little"))]
    pub __pad1: c_int,
    pub msg_control: *mut c_void,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "big"))]
    pub __pad2: c_int,
    pub msg_controllen: size_t,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "little"))]
    pub __pad2: c_int,
    pub msg_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsghdr {
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "big"))]
    pub __pad1: c_int,
    pub cmsg_len: socklen_t,
    #[cfg(all(target_pointer_width = "64", not(target_os = "windows"), target_endian = "little"))]
    pub __pad1: c_int,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

// #[cfg(target_os = "linux")]
// pub const SOCK_CLOEXEC: i32 = 0o2000000;
// #[cfg(target_os = "linux")]
// pub const SOCK_NONBLOCK: i32 = 0o4000;

#[cfg(target_pointer_width = "64")]
pub const fn CMSG_ALIGN(len: usize) -> usize
{
    (len + 8 - 1) & (!(8-1))
}

#[cfg(target_pointer_width = "32")]
pub const fn CMSG_ALIGN(len: usize) -> usize
{
    (len + 4 - 1) & (!(4-1))
}

pub const fn CMSG_SPACE(len: usize) -> usize
{
    CMSG_ALIGN(len) + CMSG_ALIGN(core::mem::size_of::<cmsghdr>())
}

pub const fn CMSG_LEN(len: usize) -> usize
{
    CMSG_ALIGN(core::mem::size_of::<cmsghdr>()) + len
}

pub fn __CMSG_LEN(cmsg: *const cmsghdr) -> size_t
{
    // (len + alignmeng) & !alignment
    unsafe {
        ((*cmsg).cmsg_len as usize + core::mem::size_of::<c_long>() as usize - 1) & !((core::mem::size_of::<c_long>() - 1) as c_long) as usize
    }
}

pub fn __CMSG_NEXT(cmsg: *const cmsghdr) -> *mut c_uchar
{
    unsafe {
        (cmsg as *mut c_uchar).add(__CMSG_LEN(cmsg) as usize)
    }
}

pub fn __MHDR_END(mhdr: *const msghdr) -> *mut c_uchar
{
    unsafe {
        ((*mhdr).msg_control as *mut c_uchar).add((*mhdr).msg_controllen as usize)
    }
}

pub fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar
{
    unsafe {
        (cmsg as *mut cmsghdr).add(1) as *mut c_uchar
    }
}

pub fn CMSG_FIRSTHDR(msg: *const msghdr) -> *mut cmsghdr
{
    if unsafe { (*msg).msg_controllen } >= core::mem::size_of::<cmsghdr>() as size_t {
        unsafe { (*msg).msg_control as *mut cmsghdr }
    } else {
        core::ptr::null_mut()
    }
}

pub fn CMSG_NEXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr
{
    unsafe {
        if (*cmsg).cmsg_len < core::mem::size_of::<cmsghdr>() as u32 {
            return core::ptr::null_mut();
        }
        let remaining = __MHDR_END(mhdr).offset_from(cmsg as *mut c_uchar) as usize;

        if ((*cmsg).cmsg_len < core::mem::size_of::<cmsghdr>() as u32) ||
            (__CMSG_LEN(cmsg) as usize + core::mem::size_of::<cmsghdr>() >= remaining)  {
            return core::ptr::null_mut();
        } else {
            return __CMSG_NEXT(cmsg) as *mut cmsghdr;
        }
    }
}