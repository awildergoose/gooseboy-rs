use crate::bindings;

pub fn get_time_nanos() -> i64 {
    unsafe { bindings::get_time_nanos() }
}
