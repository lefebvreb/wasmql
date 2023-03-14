//! A simple and fast bump allocator.

use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32;
use core::ptr;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::SeqCst;

/// This is the minimal address that is acceptable to use (i.e. not NULL).
const MIN_ADDRESS: usize = 1;
/// The size of a wasm memory page.
const PAGE_SIZE: usize = 65536;

struct BumpAllocator {
    cursor: AtomicUsize,
}

unsafe impl GlobalAlloc for BumpAllocator {
    // Simply bumps the cursor, ensuring alignment is valid and we have
    // enough memory.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (start, end) = {
            // Get current cursor.
            let cursor = self.cursor.load(SeqCst);
            // Compute padding needed to ensure correct alignment. 
            let padding = layout.align() - (cursor % layout.align());
            // Add padding to the cursor.
            let start = cursor + padding;
            // Reserve enough bytes.
            let end = cursor + layout.size();
            // Update the cursor to point to the end of the memory zone.
            self.cursor.store(end, SeqCst);
            // Yield the newly allocated zone's start and end addresses.
            (start, end)
        };

        {
            // Total amount of memory available.
            let total = wasm32::memory_size(0) * PAGE_SIZE;
            // If we don't have enough memory.
            if total < end {
                // Compute number of pages to request.
                let delta = (end - total) / PAGE_SIZE;
                // Grow memory.
                if wasm32::memory_grow(0, delta) == usize::MAX {
                    // If growing failed, return NULL pointer to signify OOM error.
                    return ptr::null::<u8>() as *mut u8;
                }
            }
        }

        // Return the start address as a pointer
        start as *mut u8
    }

    // This is a bump allocator, there is no such thing as a "deallocation" ! You fool !
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static GLOBAL: BumpAllocator = BumpAllocator { cursor: AtomicUsize::new(MIN_ADDRESS) };

/// Resets the allocator, not actually deleting anything.
/// 
/// # Safety
/// 
/// The caller must ensure that no allocated objects currently exist in the program.
pub unsafe fn clear() {
    GLOBAL.cursor.store(MIN_ADDRESS, SeqCst);
}
