//! Used to hold enums and functions for system/platform-related functions.
use crate::bindings;

/// A Gooseboy host permission.
#[repr(i32)]
pub enum Permission {
    /// Writing to the console.
    Console = 0,
    /// Playing, and stopping audio, and changing volume/pitches.
    Audio = 1,
    /// Read keyboard inputs.
    InputKeyboard = 2,
    /// Read mouse inputs.
    InputMouse = 3,
    /// Read mouse position.
    InputMousePos = 4,
    /// Grab and release mouse.
    InputGrabMouse = 5,
    /// Read from the crate's storage.
    StorageRead = 6,
    /// Write to the crate's storage.
    StorageWrite = 7,
    /// Submitting commands to the `GooseGPU`, and the `GooseGPU` virtual memory.
    Gpu = 8,
}

/// Returns the time in nanoseconds since the Unix Epoch.
#[must_use]
pub fn get_time_nanos() -> i64 {
    unsafe { bindings::get_time_nanos() }
}

/// Does this crate have the following `permission`?
#[must_use]
pub fn has_permission(permission: Permission) -> bool {
    unsafe { bindings::has_permission(permission as i32) }
}

/// Returns the current platform name.
#[must_use]
pub fn get_platform_name() -> String {
    let mut buf = [0u8; 256];
    let len = unsafe { bindings::get_platform_name(buf.as_mut_ptr()) as usize };
    let len = len.min(buf.len());
    let bytes = &buf[..len];
    String::from_utf8_lossy(bytes).into_owned()
}

/// Converts time from nanoseconds to seconds, as an f32.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
pub fn convert_nano_time_to_seconds(nano_time: i64) -> f32 {
    convert_nano_time_to_seconds_f64(nano_time) as f32
}

/// Converts time from nanoseconds to seconds, as an f64.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
pub fn convert_nano_time_to_seconds_f64(nano_time: i64) -> f64 {
    nano_time as f64 / 1_000_000_000.0
}
