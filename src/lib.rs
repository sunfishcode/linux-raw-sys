#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub use std::os::raw as ctypes;

#[cfg(all(not(feature = "std"), feature = "no_std"))]
pub mod ctypes {
    // The signedness of `char` is platform-specific, however a consequence
    // of it being platform-specific is that any code which depends on the
    // signedness of `char` is already non-portable. So we can just use `u8`
    // here and no portable code will notice.
    pub type c_char = u8;

    // The following assumes that Linux is always either ILP32 or LP64,
    // and char is always 8-bit.
    //
    // In theory, `c_long` and `c_ulong` could be `isize` and `usize`
    // respectively, however in practice Linux doesn't use them in that way
    // consistently. So stick with the convention followed by `libc` and
    // others and use the fixed-width types.
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    #[cfg(target_pointer_width = "32")]
    pub type c_long = i32;
    #[cfg(target_pointer_width = "32")]
    pub type c_ulong = u32;
    #[cfg(target_pointer_width = "64")]
    pub type c_long = i64;
    #[cfg(target_pointer_width = "64")]
    pub type c_ulong = u64;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;

    pub use core::ffi::c_void;
}

// The rest of this file is auto-generated!
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "powerpc"))]
pub mod v2_6_32;
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "powerpc"))]
pub use v2_6_32::*;
#[cfg(target_arch = "arm")]
pub mod v3_2;
#[cfg(target_arch = "arm")]
pub use v3_2::*;
#[cfg(target_arch = "powerpc64")]
pub mod v3_10;
#[cfg(target_arch = "powerpc64")]
pub use v3_10::*;
#[cfg(target_arch = "aarch64")]
pub mod v4_2;
#[cfg(target_arch = "aarch64")]
pub use v4_2::*;
#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
pub mod v4_4;
#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
pub use v4_4::*;
#[cfg(target_arch = "riscv64")]
pub mod v4_20;
#[cfg(target_arch = "riscv64")]
pub use v4_20::*;
#[cfg(feature = "v5_4")]
pub mod v5_4;
#[cfg(feature = "v5_11")]
pub mod v5_11;
