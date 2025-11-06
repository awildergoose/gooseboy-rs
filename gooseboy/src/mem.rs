use std::alloc::{self, Layout};

use crate::bindings;

/// # Safety
/// This fills a memory region with no region or value checks
pub unsafe fn fill(addr: i32, len: i32, value: i32) {
    unsafe { bindings::mem_fill(addr, len, value) }
}

/// # Safety
/// This copies a memory region with no region or value checks
pub unsafe fn copy(dst: i32, src: i32, len: i32) {
    unsafe { bindings::mem_copy(dst, src, len) }
}

pub fn alloc_bytes(len: usize) -> i32 {
    let layout = Layout::from_size_align(len, 4).unwrap();
    unsafe {
        let ptr = alloc::alloc(layout);
        ptr as i32
    }
}

pub fn free_bytes(ptr: i32, len: usize) {
    let layout = Layout::from_size_align(len, 4).unwrap();
    unsafe {
        alloc::dealloc(ptr as *mut u8, layout);
    }
}

pub fn write_i32(ptr: i32, value: i32) {
    unsafe {
        let p = ptr as *mut i32;
        *p = value;
    }
}

pub fn read_i32(ptr: i32) -> i32 {
    unsafe {
        let p = ptr as *const i32;
        *p
    }
}
