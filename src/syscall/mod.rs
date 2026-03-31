//! Sytem call functions.
//!
//! The calling conventions for Linux syscalls are [documented here].
//!
//! [documented here]: https://man7.org/linux/man-pages/man2/syscall.2.html
//!
//! # Safety
//!
//! The system call functions contains the inline `asm` statements performing the syscall
//! instructions.

// The module-level safety notice applies to all functions in this module.
#![allow(clippy::missing_safety_doc)]

/// Value passed in a CPU register.
///
/// This value should be casted to/from type dependent on used syscall.
///
/// Raw pointer is used instead of `usize` to preserve provenance.
#[repr(transparent)]
pub struct Reg(pub *mut ());

#[cfg_attr(target_arch = "aarch64", path = "aarch64.rs")]
#[cfg_attr(all(target_arch = "arm", not(thumb_mode)), path = "arm.rs")]
#[cfg_attr(all(target_arch = "arm", thumb_mode), path = "thumb.rs")]
#[cfg_attr(target_arch = "mips", path = "mips.rs")]
#[cfg_attr(target_arch = "mips32r6", path = "mips32r6.rs")]
#[cfg_attr(target_arch = "mips64", path = "mips64.rs")]
#[cfg_attr(target_arch = "mips64r6", path = "mips64r6.rs")]
#[cfg_attr(target_arch = "powerpc64", path = "powerpc64.rs")]
#[cfg_attr(target_arch = "riscv64", path = "riscv64.rs")]
#[cfg_attr(target_arch = "x86", path = "x86.rs")]
#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod asm;

pub use asm::*;
