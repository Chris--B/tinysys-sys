#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod sdk;
pub use sdk::*;
