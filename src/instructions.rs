//! Custom instructions for tinysys are exposed as Rust functions

use core::arch::asm;

/// Flush data cache to memory
#[inline(always)]
pub fn CFLUSH_D_L1() {
    // See: https://doc.rust-lang.org/reference/inline-assembly.html#options
    unsafe {
        asm!(
            ".insn 0xFC000073",
            options(nostack, preserves_flags, readonly)
        );
    }
}

/// Discard data cache contents
#[inline(always)]
pub fn CDISCARD_D_L1() {
    // See: https://doc.rust-lang.org/reference/inline-assembly.html#options
    unsafe {
        asm!(
            ".insn 0xFC200073",
            options(nostack, preserves_flags, readonly)
        );
    }
}

/// Invalidate instruction cache
#[inline(always)]
pub fn FENCE_I() {
    // See: https://doc.rust-lang.org/reference/inline-assembly.html#options
    unsafe {
        asm!("fence.i", options(nostack, preserves_flags, readonly));
    }
}
