use crate::bindings;

pub fn get_minecraft_width() -> i32 {
    unsafe { bindings::get_minecraft_width() }
}

pub fn get_minecraft_height() -> i32 {
    unsafe { bindings::get_minecraft_height() }
}

pub fn get_time_nanos() -> i64 {
    unsafe { bindings::get_time_nanos() }
}
