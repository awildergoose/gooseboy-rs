#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use crate::bindings::Pointer;

/// Converts a *const T pointer into a raw pointer.
///
/// # Safety
///
/// The caller must ensure that the pointer is the right type.
#[inline]
#[must_use]
pub const unsafe fn as_const_pointer<T>(value: *const T) -> Pointer {
    value as Pointer
}

/// Converts a usize into an i32.
///
/// # Safety
///
/// The caller must ensure that the value can fit inside an i32.
/// This shouldn't be an issue if you're dealing with bindings though,
/// as we only use 32-bit.
#[inline]
#[must_use]
pub const unsafe fn usize_as_i32(value: usize) -> i32 {
    value as i32
}

/// Converts a u32 into an i32.
///
/// # Safety
///
/// The caller must ensure that the value can fit inside an i32.
#[inline]
#[must_use]
pub const unsafe fn u32_as_i32(value: u32) -> i32 {
    value as i32
}

/// Converts an i32 into a u32.
///
/// # Safety
///
/// The caller must ensure that the value can fit inside a u32.
/// Do note that if the value is negative, the negative sign will be lost.
#[inline]
#[must_use]
pub const unsafe fn i32_as_u32(value: i32) -> u32 {
    value as u32
}

/// Returns the array length as an i32.
/// Useful for bindings.
#[inline]
#[must_use]
pub const fn arr_len<T>(arr: &[T]) -> i32 {
    arr.len() as i32
}

/// Returns the string length as an i32.
/// Useful for bindings.
#[inline]
#[must_use]
pub fn str_len(str: impl AsRef<str>) -> i32 {
    str.as_ref().len() as i32
}

/// Returns the value as a raw pointer.
#[inline]
#[must_use]
pub const fn as_raw_pointer<T>(value: *const T) -> *const u8 {
    value.cast::<u8>()
}

/// Returns the value as a mutable raw pointer.
#[inline]
#[must_use]
pub const fn as_raw_pointer_mut<T>(value: *mut T) -> *mut u8 {
    value.cast::<u8>()
}
