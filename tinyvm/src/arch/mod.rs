// @Hustler
//
// Self-Education Only

//! Architecture module, including architecture-specific functions and data structures.

#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "riscv64")]
pub use self::riscv64::*;

#[cfg(target_arch = "riscv64")]
mod riscv64;

pub use self::traits::*;
pub mod traits;
