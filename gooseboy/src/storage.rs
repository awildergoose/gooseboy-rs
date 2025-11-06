use crate::{
    bindings::{self, storage_read, storage_write},
    mem::alloc_bytes,
};

/// Requires StorageWrite permission
pub fn storage_write_value<T: Copy>(offset: i32, value: T) {
    debug_assert!(offset + size_of::<T>() as i32 <= storage_size() as i32);
    let ptr: *const T = &value;
    unsafe {
        storage_write(offset, ptr as i32, size_of::<T>() as i32);
    }
}

/// Requires StorageRead permission
pub fn storage_read_value<T: Copy>(offset: i32) -> T {
    let ptr = alloc_bytes(size_of::<T>());
    unsafe {
        storage_read(offset, ptr, size_of::<T>() as i32);
        *(ptr as *const T)
    }
}

/// Requires StorageWrite permission
pub fn storage_write_slice(offset: i32, data: &[u8]) {
    unsafe {
        storage_write(offset, data.as_ptr() as i32, data.len() as i32);
    }
}

/// Requires StorageRead permission
pub fn storage_read_slice(offset: i32, buf: &mut [u8]) -> i32 {
    unsafe { storage_read(offset, buf.as_mut_ptr() as i32, buf.len() as i32) }
}

/// Requires StorageWrite permission
pub fn storage_clear() {
    unsafe {
        bindings::storage_clear();
    }
}

/// Requires StorageRead permission
pub fn storage_size() -> u32 {
    unsafe { bindings::storage_size() }
}
