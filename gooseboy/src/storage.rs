use crate::{
    bindings::{self, storage_read, storage_write},
    mem::alloc_bytes,
    unsafe_casts,
};

/// Requires `StorageWrite` permission
pub fn storage_write_value<T: Copy>(offset: i32, value: T) {
    let size = unsafe { unsafe_casts::usize_as_i32(size_of::<T>()) };
    debug_assert!(offset + size <= storage_size().cast_signed());

    let ptr: *const T = &raw const value;
    unsafe {
        storage_write(offset, ptr.cast::<u8>(), size);
    }
}

/// Requires `StorageRead` permission
#[must_use]
pub fn storage_read_value<T: Copy>(offset: i32) -> T {
    unsafe {
        let ptr = alloc_bytes(size_of::<T>());
        storage_read(offset, ptr, unsafe_casts::usize_as_i32(size_of::<T>()));
        *(ptr as *const T)
    }
}

/// Requires `StorageWrite` permission
pub fn storage_write_slice(offset: i32, data: &[u8]) {
    unsafe {
        storage_write(offset, data.as_ptr(), unsafe_casts::arr_len(data));
    }
}

/// Requires `StorageRead` permission
pub fn storage_read_slice(offset: i32, buf: &mut [u8]) -> i32 {
    unsafe { storage_read(offset, buf.as_mut_ptr(), unsafe_casts::arr_len(buf)) }
}

/// Requires `StorageWrite` permission
pub fn storage_clear() {
    unsafe {
        bindings::storage_clear();
    }
}

/// Requires `StorageRead` permission
#[must_use]
pub fn storage_size() -> u32 {
    unsafe { bindings::storage_size() }
}
