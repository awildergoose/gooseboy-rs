use std::alloc::{self, Layout};

use crate::bindings::{self, Pointer, PointerMut};

/// # Safety
/// This fills a memory region with no region or value checks
pub unsafe fn fill(addr: PointerMut, len: i32, value: i32) {
    unsafe { bindings::mem_fill(addr, len, value) }
}

/// # Safety
/// This copies a memory region with no region or value checks
pub unsafe fn copy(dst: PointerMut, src: Pointer, len: i32) {
    unsafe { bindings::mem_copy(dst, src, len) }
}

/// Allocates bytes using the standard allocator.
///
/// # Panics
///
/// Panics if the length, when rounded up to the nearest multiple of 4, overflows isize.
///
/// # Safety
///
/// No length checks are put in place.
#[must_use]
pub unsafe fn alloc_bytes<T>(len: usize) -> *mut T {
    let layout = Layout::from_size_align(len, 4).unwrap();
    unsafe { alloc::alloc(layout).cast::<T>() }
}

/// Frees bytes using the standard allocator.
///
/// # Panics
///
/// Panics if the length, when rounded up to the nearest multiple of 4, overflows isize.
///
/// # Safety
///
/// No length checks are put in place.
pub unsafe fn free_bytes(ptr: PointerMut, len: usize) {
    let layout = Layout::from_size_align(len, 4).unwrap();
    unsafe {
        alloc::dealloc(ptr, layout);
    }
}
