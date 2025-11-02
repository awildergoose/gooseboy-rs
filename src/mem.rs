use crate::bindings;

/// # Safety
/// This fills a memory region with no region or value checks
pub unsafe fn fill(addr: i32, len: i32, value: i32) {
    unsafe { bindings::mem_fill(addr, len, value) }
}
