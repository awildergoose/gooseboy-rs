use crate::test;
use gooseboy::storage::{storage_clear, storage_read_slice, storage_size, storage_write_slice};

pub fn test_storage() {
    let size = storage_size() as usize;
    if size == 0 {
        test!("storage:slice_no_storage", true);
        return;
    }

    storage_clear();

    let data = [1u8, 2u8, 3u8, 4u8];
    storage_write_slice(0, &data);

    let mut buf = [0u8; 4];
    let read = storage_read_slice(0, &mut buf);

    test!("storage:slice_bytes_read_len", read as usize == buf.len());
    test!("storage:slice_bytes_equal", buf == data);
}
