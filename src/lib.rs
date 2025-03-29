#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

// These bindings are tied to the target they were generated for, and break on other platforms
#[cfg(target_arch = "riscv32")]
mod sdk;
#[cfg(target_arch = "riscv32")]
pub use sdk::*;

#[cfg(feature = "alloc")]
mod printing;

/// This crate is largely generated and from a specific git rev of the tinysys SDK headers.
/// This is the git rev that was used. See the full tinysys repo for more information on this specific hash.
pub const TINYSYS_SDK_GIT_REV: &str = include_str!("../tinysys_c_sdk/git-HEAD.txt");

#[cfg(feature = "malloc_free")]
mod malloc_free {
    use super::*;

    use alloc::alloc::Layout;

    // This is so verbose
    const PTR_SIZE: usize = core::mem::size_of::<usize>();

    fn layout(size: usize) -> Layout {
        // Note: We'll always align to a pointer (4 bytes on rv32) to keep things simple
        unsafe { Layout::from_size_align_unchecked(size, PTR_SIZE) }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
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
    pub unsafe extern "C" fn free(ptr: *mut u8) {
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
}
