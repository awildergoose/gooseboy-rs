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
