use std::alloc::{self, Layout};

use crate::bindings::{self, Pointer};

/// # Safety
/// This fills a memory region with no region or value checks
pub unsafe fn fill(addr: Pointer, len: i32, value: i32) {
    unsafe { bindings::mem_fill(addr, len, value) }
}

/// # Safety
/// This copies a memory region with no region or value checks
pub unsafe fn copy(dst: Pointer, src: Pointer, len: i32) {
    unsafe { bindings::mem_copy(dst, src, len) }
}

#[must_use]
pub fn alloc_bytes<T>(len: usize) -> *mut T {
    let layout = Layout::from_size_align(len, 4).unwrap();
    unsafe { alloc::alloc(layout).cast::<T>() }
}

/// # Safety
/// This frees a pointer, length is specified by an arg, self-explainable
pub unsafe fn free_bytes(ptr: *mut u8, len: usize) {
    let layout = Layout::from_size_align(len, 4).unwrap();
    unsafe {
        alloc::dealloc(ptr, layout);
    }
}

pub fn write_i32(ptr: *mut i32, value: i32) {
    unsafe {
        let p = ptr;
        *p = value;
    }
}

#[must_use]
pub const fn read_i32(ptr: *const i32) -> i32 {
    unsafe {
        let p = ptr;
        *p
    }
}
