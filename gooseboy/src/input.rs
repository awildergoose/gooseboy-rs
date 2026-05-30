//! This is used to hold input related functions, for keyboard and mouse functions.
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::{bindings, keys::Key};

static PREV_KEYS: LazyLock<Mutex<HashMap<Key, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static PREV_MOUSE: LazyLock<Mutex<HashMap<i32, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Is any key held down?
/// Requires [`InputKeyboard`](crate::system::Permission::InputKeyboard) permission
#[must_use]
pub fn is_any_key_down() -> bool {
    unsafe { bindings::get_key_code() != -1 }
}

/// Returns the current active key (or `None` if none)
/// Requires [`InputKeyboard`](crate::system::Permission::InputKeyboard) permission
#[must_use]
pub fn get_key() -> Option<i32> {
    let key = unsafe { bindings::get_key_code() };
    if key == -1 {
        return None;
    }
    Some(key)
}

/// Is `key` down?
/// Requires [`InputKeyboard`](crate::system::Permission::InputKeyboard) permission
#[must_use]
pub fn is_key_down(key: Key) -> bool {
    unsafe { bindings::get_key(key) }
}

/// Is `button` down?
/// Requires [`InputMouse`](crate::system::Permission::InputMouse) permission
#[must_use]
pub fn is_mouse_button_down(button: i32) -> bool {
    unsafe { bindings::get_mouse_button(button) }
}

/// Returns the mouse X position.
/// Requires [`InputMousePos`](crate::system::Permission::InputMousePos) permission
#[must_use]
pub fn get_mouse_x() -> i32 {
    unsafe { bindings::get_mouse_x() }
}

/// Returns the mouse Y position.
/// Requires [`InputMousePos`](crate::system::Permission::InputMousePos) permission
#[must_use]
pub fn get_mouse_y() -> i32 {
    unsafe { bindings::get_mouse_y() }
}

/// Returns the mouse accumulated delta X, helpful for first-person games.
/// Requires [`InputMousePos`](crate::system::Permission::InputMousePos) permission
#[must_use]
pub fn get_mouse_accumulated_dx() -> f64 {
    unsafe { bindings::get_mouse_accumulated_dx() }
}

/// Returns the mouse accumulated delta Y, helpful for first-person games.
/// Requires [`InputMousePos`](crate::system::Permission::InputMousePos) permission
#[must_use]
pub fn get_mouse_accumulated_dy() -> f64 {
    unsafe { bindings::get_mouse_accumulated_dy() }
}

/// Is the mouse grabbed?
/// Requires [`InputGrabMouse`](crate::system::Permission::InputGrabMouse) permission
#[must_use]
pub fn is_mouse_grabbed() -> bool {
    unsafe { bindings::is_mouse_grabbed() }
}

/// Grabs the mouse.
/// Requires [`InputGrabMouse`](crate::system::Permission::InputGrabMouse) permission
pub fn grab_mouse() {
    unsafe { bindings::grab_mouse() }
}

/// Releases the mouse.
/// Requires [`InputGrabMouse`](crate::system::Permission::InputGrabMouse) permission
pub fn release_mouse() {
    unsafe { bindings::release_mouse() }
}

/// Was `key` just pressed?
///
/// Requires [`InputKeyboard`](crate::system::Permission::InputKeyboard) permission
///
/// # Panics
/// Panics if the previous keys static was accessed by another thread and had panicked. (never)
pub fn is_key_just_pressed(key: Key) -> bool {
    let currently_pressed = is_key_down(key);
    let mut prev_keys = PREV_KEYS.lock().unwrap();
    let was_pressed = *prev_keys.get(&key).unwrap_or(&false);

    prev_keys.insert(key, currently_pressed);
    drop(prev_keys);

    currently_pressed && !was_pressed
}

/// Was `button` just pressed?
///
/// Requires [`InputMouse`](crate::system::Permission::InputMouse) permission
///
/// # Panics
/// Panics if the previous mouse buttons static was accessed by another thread and had panicked. (never)
pub fn is_mouse_button_just_pressed(button: i32) -> bool {
    let currently_pressed = is_mouse_button_down(button);
    let mut prev_mouse = PREV_MOUSE.lock().unwrap();
    let was_pressed = *prev_mouse.get(&button).unwrap_or(&false);

    prev_mouse.insert(button, currently_pressed);
    drop(prev_mouse);

    currently_pressed && !was_pressed
}
