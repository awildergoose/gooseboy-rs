use crate::{
    bindings::{self, storage_read, storage_write},
    mem::alloc_bytes,
};

pub fn storage_write_value<T: Copy>(offset: i32, value: T) {
    let ptr: *const T = &value;
    unsafe {
        storage_write(offset, ptr as i32, size_of::<T>() as i32);
    }
}

pub fn storage_read_value<T: Copy>(offset: i32) -> T {
    let ptr = alloc_bytes(size_of::<T>());
    unsafe {
        storage_read(offset, ptr, size_of::<T>() as i32);
        *(ptr as *const T)
    }
}

pub fn storage_clear() {
    unsafe {
        bindings::storage_clear();
    }
}

pub fn storage_size() -> u32 {
    unsafe { bindings::storage_size() }
}
