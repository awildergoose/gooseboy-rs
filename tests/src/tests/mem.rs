use crate::test;
use gooseboy::mem::{alloc_bytes, read_i32, write_i32};

pub fn test_mem() {
    let ptr = alloc_bytes(16);
    test!("mem:alloc_nonzero", ptr != 0);

    write_i32(ptr, 0x12345678);
    let val = read_i32(ptr);
    test!("mem:write_read_i32", val == 0x12345678);
}
