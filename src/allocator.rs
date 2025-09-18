use core::alloc::{GlobalAlloc, Layout};

use uefi::boot;

pub struct UefiAllocator;

unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Use the boot module to allocate
        match boot::allocate_pool(boot::MemoryType::LOADER_DATA, layout.size()) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if let Some(nonnull) = core::ptr::NonNull::new(ptr) {
            let _ = unsafe { boot::free_pool(nonnull) };
        }
    }
}


