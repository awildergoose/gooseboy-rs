use crate::bindings;

#[repr(i32)]
pub enum Permission {
    Console = 0,
    Audio = 1,
    InputKeyboard = 2,
    InputMouse = 3,
    InputMousePos = 4,
    InputGrabMouse = 5,
    StorageRead = 6,
    StorageWrite = 7,
    ExtendedMemory = 8,
}

#[must_use]
pub fn get_time_nanos() -> i64 {
    unsafe { bindings::get_time_nanos() }
}

#[must_use]
pub fn has_permission(permission: Permission) -> bool {
    unsafe { bindings::has_permission(permission as i32) }
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
pub fn convert_nano_time_to_seconds(nano_time: i64) -> f32 {
    convert_nano_time_to_seconds_f64(nano_time) as f32
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
pub fn convert_nano_time_to_seconds_f64(nano_time: i64) -> f64 {
    nano_time as f64 / 1_000_000_000.0
}
