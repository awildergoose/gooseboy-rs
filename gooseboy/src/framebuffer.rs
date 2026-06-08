//! This is used to manage and control the framebuffer.
//!
//! Example:
//! ```rs
//! get_framebuffer_surface_mut().clear(Color::GREEN);
//! ```
use std::sync::{Mutex, MutexGuard};

use crate::{
    bindings::{self},
    color::Color,
    unsafe_casts,
};

/// The global framebuffer surface.
pub static FRAMEBUFFER_SURFACE: Mutex<Option<Surface>> = Mutex::new(None);

/// Pointer to the raw framebuffer.
pub type RawFramebufferPointer = *const [u8; 4];
/// Mutable pointer to the raw framebuffer.
pub type RawFramebufferPointerMut = *mut [u8; 4];

/// Returns the pointer of the global framebuffer.
/// The host only calls this function once.
#[cfg(feature = "binary")]
#[unsafe(no_mangle)]
pub extern "C" fn get_framebuffer_ptr() -> RawFramebufferPointer {
    get_framebuffer_surface_ref().rgba.as_ptr() as RawFramebufferPointer
}

/// Returns the mutable pointer of the global framebuffer.
#[must_use]
pub fn get_framebuffer_ptr_mut() -> RawFramebufferPointerMut {
    get_framebuffer_surface_mut().rgba.as_mut_ptr() as RawFramebufferPointerMut
}

fn get_framebuffer_surface() -> MutexGuard<'static, Option<Surface>> {
    FRAMEBUFFER_SURFACE.lock().unwrap()
}

/// Returns the framebuffer surface as a reference.
///
/// # Panics
///
/// Panics if the framebuffer was not initialized.
#[must_use]
#[allow(clippy::significant_drop_tightening)]
pub fn get_framebuffer_surface_ref() -> &'static Surface {
    let guard = get_framebuffer_surface();
    let surface_ref: &Surface = guard.as_ref().expect("surface not initialized");
    unsafe { &*std::ptr::from_ref::<Surface>(surface_ref) }
}

/// Returns the framebuffer surface as a mutable reference.
///
/// # Panics
///
/// Panics if the framebuffer was not initialized.
#[must_use]
#[allow(clippy::significant_drop_tightening)]
pub fn get_framebuffer_surface_mut() -> &'static mut Surface {
    let mut guard = get_framebuffer_surface();
    let surface_mut: &mut Surface = guard.as_mut().expect("surface not initialized");
    unsafe { &mut *std::ptr::from_mut::<Surface>(surface_mut) }
}

/// Initializes the framebuffer surface.
///
/// # Panics
///
/// Panics if the framebuffer surface was being accessed by another thread and had panicked. (never)
pub fn init_fb() {
    unsafe {
        let mut guard = FRAMEBUFFER_SURFACE.lock().unwrap();
        *guard = Some(Surface::new_empty(
            bindings::get_framebuffer_width(),
            bindings::get_framebuffer_height(),
        ));
    }
}

/// Returns an index from a position, using the global framebuffer surface's size, and `None` if out of bounds.
#[must_use]
#[inline]
pub fn get_pixel_index(x: usize, y: usize) -> Option<usize> {
    get_framebuffer_surface_ref().get_pixel_index(x, y)
}

/// Sets a pixel on the global framebuffer surface.
pub fn set_pixel(x: usize, y: usize, color: Color) {
    if let Some(index) = get_pixel_index(x, y) {
        unsafe { color.blit(index) };
    }
}

/// Returns the framebuffer width.
#[must_use]
pub fn get_framebuffer_width() -> usize {
    get_framebuffer_surface_ref().width
}

/// Returns the framebuffer height.
#[must_use]
pub fn get_framebuffer_height() -> usize {
    get_framebuffer_surface_ref().height
}

/// Returns the size of the framebuffer. (w*h*4)
#[must_use]
pub fn get_framebuffer_size() -> usize {
    get_framebuffer_width() * get_framebuffer_height() * 4
}

/// Clears the framebuffer with the following `color`.
pub fn clear_framebuffer(color: Color) {
    get_framebuffer_surface_mut().clear(color);
}

/// Clears a surface with the following `color`.
///
/// # Safety
/// This expects ptr to be a pointer to an RGBA buffer (check Surface's rgba)
pub unsafe fn clear_surface(ptr: RawFramebufferPointer, size: usize, color: Color) {
    let color_val = (usize::from(color.a) << 24)
        | (usize::from(color.b) << 16)
        | (usize::from(color.g) << 8)
        | usize::from(color.r);

    unsafe {
        bindings::clear_surface(
            unsafe_casts::as_const_pointer(ptr),
            unsafe_casts::usize_as_i32(size),
            unsafe_casts::usize_as_i32(color_val),
        );
    };
}

/// A surface to render on.
#[derive(Clone)]
pub struct Surface {
    /// Raw RGBA pixels. (this should always be divisible by 4)
    pub rgba: Vec<u8>,
    /// The width of the surface.
    pub width: usize,
    /// The height of the surface.
    pub height: usize,
}

impl Surface {
    /// Creates a new surface with the dimensions of `width`x`height`, with `rgba` being in RGBA, and divisible by 4.
    #[must_use]
    pub const fn new(width: usize, height: usize, rgba: Vec<u8>) -> Self {
        debug_assert!(rgba.len().is_multiple_of(4), "Surface RGBA is invalid");
        Self {
            rgba,
            width,
            height,
        }
    }

    /// Creates a new fully black surface with the dimensions of `width`x`height`.
    #[must_use]
    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rgba: vec![0; width * height * 4],
        }
    }

    /// Clears the surface with the following `color`.
    pub fn clear(&mut self, color: Color) {
        unsafe {
            clear_surface(
                self.rgba.as_mut_ptr() as RawFramebufferPointer,
                self.width * self.height * 4,
                color,
            );
        }
    }

    /// Sets a pixel to the following `color`.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if let Some(index) = self.get_pixel_index(x, y) {
            unsafe {
                color.blit_ex(self, index);
            }
        }
    }

    /// Returns the index of a position, using `surface`'s size, and `None` if out of bounds.
    #[must_use]
    pub fn get_pixel_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let row = y.checked_mul(self.width)?;
        let pos = row.checked_add(x)?;
        pos.checked_mul(4)
    }

    /// Draws a rectangle to the following position with the following dimensions, to the following `color`,
    /// with optional blending.
    pub fn draw_rect(
        &mut self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        color: Color,
        blend: bool,
    ) {
        let mut surf = Self::new_empty(width, height);
        surf.clear(color);
        self.blit_premultiplied_clipped(x, y, width, height, &surf.rgba, blend);
    }

    /// Blits a surface to the following position with the following dimensions, to the following `color`,
    /// with optional blending.
    pub fn blit_premultiplied_clipped(
        &self,
        dest_x: i32,
        dest_y: i32,
        src_w: usize,
        src_h: usize,
        src_rgba: &[u8],
        blend: bool,
    ) {
        unsafe {
            bindings::blit_premultiplied_clipped(
                self.rgba.as_ptr(),
                self.width,
                self.height,
                dest_x,
                dest_y,
                src_w,
                src_h,
                src_rgba.as_ptr(),
                blend,
            );
        }
    }
}
