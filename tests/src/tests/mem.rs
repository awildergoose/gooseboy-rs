use crate::test;
use gooseboy::mem::alloc_bytes;

pub fn test_mem() {
    let ptr: *mut i32 = unsafe { alloc_bytes(16) };
    test!("mem:alloc_nonzero", !ptr.is_null());

    // write_i32(ptr, 0x12345678);
    // let val = read_i32(ptr);
    // test!("mem:write_read_i32", val == 0x12345678);
}
