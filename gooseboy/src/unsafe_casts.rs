#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

use crate::bindings::Pointer;

#[inline]
#[must_use]
pub const unsafe fn as_const_pointer<T>(val: *const T) -> Pointer {
    val as Pointer
}

#[inline]
#[must_use]
pub const unsafe fn usize_as_i32(val: usize) -> i32 {
    val as i32
}

#[inline]
#[must_use]
pub const unsafe fn u32_as_i32(val: u32) -> i32 {
    val as i32
}

#[inline]
#[must_use]
pub unsafe fn as_pointer<T>(val: &T) -> i32 {
    std::ptr::from_ref::<T>(val) as usize as i32
}

#[inline]
#[must_use]
pub unsafe fn as_pointer_mut<T>(val: &mut T) -> i32 {
    std::ptr::from_mut::<T>(val) as usize as i32
}

#[inline]
#[must_use]
pub const unsafe fn arr_len<T>(arr: &[T]) -> i32 {
    arr.len() as i32
}

#[inline]
#[must_use]
pub unsafe fn str_len(str: impl AsRef<str>) -> i32 {
    str.as_ref().len() as i32
}
