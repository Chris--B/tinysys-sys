#![no_std]

// These bindings are tied to the target they were generated for, and break on other platforms
#[cfg(target_arch = "riscv32")]
mod sdk;
#[cfg(target_arch = "riscv32")]
pub use sdk::*;

/// This crate is largely generated and from a specific git rev of the tinysys SDK headers.
/// This is the git rev that was used. See the full tinysys repo for more information on this specific hash.
pub const TINYSYS_SDK_GIT_REV: &str = include_str!("../tinysys_c_sdk/git-HEAD.txt");
