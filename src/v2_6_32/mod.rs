#[cfg(target_arch = "powerpc")]
mod powerpc;
#[cfg(target_arch = "powerpc")]
pub use powerpc::*;
#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86")]
pub use x86::*;
#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
