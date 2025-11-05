use crate::test;

pub fn test_storage() {
    use gooseboy::storage::storage_read_value;

    for i in 0..100 {
        if i % 5 == 0 {
            test!("storage", storage_read_value::<u8>(0) == 0);
        }
        test!("should_fail", storage_read_value::<u8>(0) == 1);
    }
}
