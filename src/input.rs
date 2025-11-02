use crate::{bindings, keys::Key};

pub fn is_key_down(key: Key) -> bool {
    unsafe { bindings::get_key(key) }
}

pub fn is_mouse_button_down(button: i32) -> bool {
    unsafe { bindings::get_mouse_button(button) }
}

pub fn get_mouse_x() -> i32 {
    unsafe { bindings::get_mouse_x() }
}

pub fn get_mouse_y() -> i32 {
    unsafe { bindings::get_mouse_y() }
}
