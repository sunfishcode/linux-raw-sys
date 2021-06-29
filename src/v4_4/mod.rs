#[cfg(target_arch = "mips")]
mod mips;
#[cfg(target_arch = "mips")]
pub use mips::*;
#[cfg(target_arch = "mips64")]
mod mips64;
#[cfg(target_arch = "mips64")]
pub use mips64::*;
