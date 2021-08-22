#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![no_std]

#[cfg(not(feature = "libc"))]
extern crate std;
#[cfg(not(feature = "libc"))]
use std::os::raw as ctypes;

#[cfg(feature = "libc")]
use libc as ctypes;

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
