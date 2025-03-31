#![cfg_attr(not(test), no_std)]
// When bridging items like macros, we prefer to match the casing exactly.
#![allow(non_snake_case)]

#[cfg(feature = "alloc")]
extern crate alloc;

// These bindings are tied to the target they were generated for, and break on other platforms
#[cfg(target_arch = "riscv32")]
mod sdk;
#[cfg(target_arch = "riscv32")]
pub use sdk::*;

#[cfg(feature = "alloc")]
mod printing;
// only exports macros

#[cfg(target_arch = "riscv32")]
mod instructions;
#[cfg(target_arch = "riscv32")]
pub use instructions::*;

/// Hardware format is: 12bit R:G:B
pub const fn MAKECOLORRGB12(r: u8, g: u8, b: u8) -> u32 {
    ((((r as u32) & 0xF) << 8) | ((g as u32) & 0xF) << 4) | ((b as u32) & 0xF)
}

pub const fn MCONTROL_TYPE(xlen: u64) -> u64 {
    0xf_u64 << ((xlen) - 4)
}

pub const fn MCONTROL_DMODE(xlen: u32) -> u64 {
    1_u64 << ((xlen) - 5)
}

pub const fn MCONTROL_MASKMAX(xlen: u32) -> u64 {
    0x3f_u64 << ((xlen) - 11)
}

#[cfg(target_arch = "riscv32")]
pub fn APUFrame() -> u32 {
    unsafe { core::ptr::read_volatile(sdk::IO_AUDIOOUT) }
}

/// This crate is largely generated and from a specific git rev of the tinysys SDK headers.
/// This is the git rev that was used. See the full tinysys repo for more information on this specific hash.
pub const TINYSYS_SDK_GIT_REV: &str = include_str!("../c_sdk/git-HEAD.txt");

#[cfg(feature = "malloc_free")]
mod malloc_free {
    use super::*;

    use alloc::alloc::Layout;

    // This is so verbose
    const PTR_SIZE: usize = core::mem::size_of::<usize>();

    fn layout(size: usize) -> Layout {
        // Note: We'll always align to a pointer (4 bytes on rv32) to keep things simple
        Layout::from_size_align(size, PTR_SIZE).unwrap()
    }

    // NOTE: We cannot name this `malloc` on platforms with `std`, or risk stack overflow: `alloc::alloc::alloc` defers to `malloc`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ts_malloc(size: usize) -> *mut u8 {
        // Safety: Do not call anything that can alloc in here. That includes `dbg!()`!
        unsafe {
            // Adjust our size request to handle our size header
            let size = size + PTR_SIZE;
            let ptr: *mut u8 = alloc::alloc::alloc(layout(size));

            // Save the allocation size in the word before the allocation, so we can grab it in `free()`
            (ptr as *mut usize).write(size);

            // Return the pointer *after* our header
            ptr.add(PTR_SIZE)
        }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ts_free(ptr: *mut u8) {
        // C/++ require `free()` handles NULL
        if ptr.is_null() {
            return;
        }

        // Safety: Do not call anything that can alloc in here. That includes `dbg!()`!
        unsafe {
            // Adjust back from our header, which contains the allocation size
            let ptr = ptr.sub(PTR_SIZE);
            let size = (ptr as *mut usize).read();

            alloc::alloc::dealloc(ptr, layout(size));
        }
    }

    #[unsafe(no_mangle)]
    #[cfg(target_os = "none")]
    pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
        unsafe { ts_malloc(size) }
    }

    #[unsafe(no_mangle)]
    #[cfg(target_os = "none")]
    pub unsafe extern "C" fn free(ptr: *mut u8) {
        unsafe { ts_free(ptr) }
    }

    #[cfg(test)]
    mod t {
        use super::*;

        #[test]
        fn check_malloc_and_free_doesnt_crash() {
            unsafe {
                let a = ts_malloc(4);
                {
                    let a = &mut *(a as *mut u32);
                    *a = 10;
                    *a += 10;
                    assert_eq!(*a, 20);
                }
                ts_free(a);
            }
        }
    }
}
