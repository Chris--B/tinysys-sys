#![no_std]

// These bindings are tied to the target they were generated for, and break on other platforms
#[cfg(target_arch = "riscv32")]
mod sdk;
#[cfg(target_arch = "riscv32")]
pub use sdk::*;
