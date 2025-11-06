use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::{bindings, keys::Key};

static PREV_KEYS: LazyLock<Mutex<HashMap<Key, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static PREV_MOUSE: LazyLock<Mutex<HashMap<i32, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn is_any_key_down() -> bool {
    unsafe { bindings::get_key_code() != -1 }
}

pub fn get_key() -> Option<i32> {
    let key = unsafe { bindings::get_key_code() };
    if key == -1 {
        return None;
    }
    Some(key)
}

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

pub fn grab_mouse() {
    unsafe { bindings::grab_mouse() }
}

pub fn release_mouse() {
    unsafe { bindings::release_mouse() }
}

pub fn is_key_just_pressed(key: Key) -> bool {
    let currently_pressed = is_key_down(key);
    let mut prev_keys = PREV_KEYS.lock().unwrap();
    let was_pressed = *prev_keys.get(&key).unwrap_or(&false);

    prev_keys.insert(key, currently_pressed);

    currently_pressed && !was_pressed
}

pub fn is_mouse_button_just_pressed(button: i32) -> bool {
    let currently_pressed = is_mouse_button_down(button);
    let mut prev_mouse = PREV_MOUSE.lock().unwrap();
    let was_pressed = *prev_mouse.get(&button).unwrap_or(&false);

    prev_mouse.insert(button, currently_pressed);
    currently_pressed && !was_pressed
}
