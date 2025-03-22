pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;

pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub type c_float = f32;
pub type c_double = f64;

pub type size_t = usize;
pub type ssize_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;

pub type c_void = core::ffi::c_void;

cfg_if! {
    if #[cfg(all(
        not(windows),
        // FIXME(ctest): just use `target_vendor` = "apple"` once `ctest` supports it
        not(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "watchos",
            target_os = "visionos",
        )),
        not(target_os = "vita"),
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "csky",
            target_arch = "hexagon",
            target_arch = "msp430",
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "xtensa",
        )
    ))] {
        pub type c_char = u8;
    } else {
        // On every other target, c_char is signed.
        // c_char is signed on x86_64
        pub type c_char = i8;
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "avr", target_arch = "msp430"))] {
        pub type c_int = i16;
        pub type c_uint = u16;
    } else {
        pub type c_int = i32;
        pub type c_uint = u32;
    }
}

cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        pub type c_long = i64;
        pub type c_ulong = u64;
    } else {
        // The minimal size of `long` in the C standard is 32 bits
        pub type c_long = i32;
        pub type c_ulong = u32;
    }
}

// #[deprecated(since = "0.2.55", note = "Use i8 instead.")]
pub type int8_t = i8;
// #[deprecated(since = "0.2.55", note = "Use i16 instead.")]
pub type int16_t = i16;
// #[deprecated(since = "0.2.55", note = "Use i32 instead.")]
pub type int32_t = i32;
// #[deprecated(since = "0.2.55", note = "Use i64 instead.")]
pub type int64_t = i64;
// #[deprecated(since = "0.2.55", note = "Use u8 instead.")]
pub type uint8_t = u8;
// #[deprecated(since = "0.2.55", note = "Use u16 instead.")]
pub type uint16_t = u16;
// #[deprecated(since = "0.2.55", note = "Use u32 instead.")]
pub type uint32_t = u32;
// #[deprecated(since = "0.2.55", note = "Use u64 instead.")]
pub type uint64_t = u64;

cfg_if! {
    if #[cfg(all(target_arch = "aarch64", not(target_os = "windows")))] {
        pub type __int128 = i128;

        pub type __uint128 = u128;
        
        pub type __int128_t = i128;
        
        pub type __uint128_t = u128;

        // NOTE: if you add more platforms to here, you may need to cfg
        // these consts. They should always match the platform's values
        // for `sizeof(__int128)` and `_Alignof(__int128)`.
        const _SIZE_128: usize = 16;
        const _ALIGN_128: usize = 16;

        // FIXME(ctest): ctest doesn't handle `_` as an identifier so these tests are temporarily
        // disabled.
        // macro_rules! static_assert_eq {
        //     ($a:expr, $b:expr) => {
        //         const _: [(); $a] = [(); $b];
        //     };
        // }
        //
        // // Since Rust doesn't officially guarantee that these types
        // // have compatible ABIs, we const assert that these values have the
        // // known size/align of the target platform's libc. If rustc ever
        // // tries to regress things, it will cause a compilation error.
        // //
        // // This isn't a bullet-proof solution because e.g. it doesn't
        // // catch the fact that llvm and gcc disagree on how x64 __int128
        // // is actually *passed* on the stack (clang underaligns it for
        // // the same reason that rustc *never* properly aligns it).
        // static_assert_eq!(core::mem::size_of::<__int128>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__int128>(), _ALIGN_128);

        // static_assert_eq!(core::mem::size_of::<__uint128>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__uint128>(), _ALIGN_128);

        // static_assert_eq!(core::mem::size_of::<__int128_t>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__int128_t>(), _ALIGN_128);

        // static_assert_eq!(core::mem::size_of::<__uint128_t>(), _SIZE_128);
        // static_assert_eq!(core::mem::align_of::<__uint128_t>(), _ALIGN_128);
    }
}

#[repr(C)]
pub struct sigset_t {
    #[cfg(target_pointer_width = "32")]
    __val: [c_ulong; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [c_ulong; 16],
}

pub type syscall_arg_t = c_long;