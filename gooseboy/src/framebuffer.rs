use std::sync::{Mutex, MutexGuard};

use crate::{bindings, color::Color};

pub static FRAMEBUFFER_SURFACE: Mutex<Option<Surface>> = Mutex::new(None);

#[unsafe(no_mangle)]
pub extern "C" fn get_framebuffer_ptr() -> *const u8 {
    get_framebuffer_surface_ref().rgba.as_ptr()
}

fn get_framebuffer_surface() -> MutexGuard<'static, Option<Surface>> {
    FRAMEBUFFER_SURFACE.lock().unwrap()
}

#[must_use] 
pub fn get_framebuffer_surface_ref() -> &'static Surface {
    let guard = get_framebuffer_surface();
    let surface_ref: &Surface = guard.as_ref().expect("surface not initialized");
    unsafe { &*std::ptr::from_ref::<Surface>(surface_ref) }
}

#[must_use] 
pub fn get_framebuffer_surface_mut() -> &'static mut Surface {
    let mut guard = get_framebuffer_surface();
    let surface_mut: &mut Surface = guard.as_mut().expect("surface not initialized");
    unsafe { &mut *std::ptr::from_mut::<Surface>(surface_mut) }
}

pub fn init_fb() {
    unsafe {
        let mut guard = FRAMEBUFFER_SURFACE.lock().unwrap();
        *guard = Some(Surface::new_empty(
            bindings::get_framebuffer_width(),
            bindings::get_framebuffer_height(),
        ));
    }
}

#[must_use] 
pub fn get_pixel_index(x: usize, y: usize) -> Option<usize> {
    get_pixel_index_ex(get_framebuffer_surface_ref(), x, y)
}

#[must_use] 
pub fn get_pixel_index_ex(surface: &Surface, x: usize, y: usize) -> Option<usize> {
    if x >= surface.width || y >= surface.height {
        return None;
    }

    let row = y.checked_mul(surface.width)?;
    let pos = row.checked_add(x)?;
    pos.checked_mul(4)
}

pub fn set_pixel(x: usize, y: usize, color: Color) {
    if let Some(index) = get_pixel_index(x, y) {
        unsafe { color.blit(index) };
    }
}

pub fn set_pixel_ex(surface: &mut Surface, x: usize, y: usize, color: Color) {
    if let Some(index) = get_pixel_index_ex(surface, x, y) {
        unsafe {
            color.blit_ex(surface, index);
        }
    }
}

#[must_use] 
pub fn get_framebuffer_width() -> usize {
    get_framebuffer_surface_ref().width
}

#[must_use] 
pub fn get_framebuffer_height() -> usize {
    get_framebuffer_surface_ref().height
}

#[must_use] 
pub fn get_framebuffer_size() -> usize {
    get_framebuffer_width() * get_framebuffer_height() * 4
}

pub fn clear_framebuffer(color: Color) {
    unsafe { clear_surface(get_framebuffer_ptr(), get_framebuffer_size(), color) };
}

/// # Safety
/// This expects ptr to be a pointer to an RGBA buffer (check Surface's rgba)
pub unsafe fn clear_surface(ptr: *const u8, size: usize, color: Color) {
    let color_val = (u32::from(color.a) << 24)
        | (u32::from(color.b) << 16)
        | (u32::from(color.g) << 8)
        | u32::from(color.r);

    unsafe { bindings::clear_surface(ptr, size as i32, color_val as i32) };
}

#[derive(Clone)]
pub struct Surface {
    pub rgba: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Surface {
    #[must_use] 
    pub const fn new(width: usize, height: usize, rgba: Vec<u8>) -> Self {
        Self {
            rgba,
            width,
            height,
        }
    }

    #[must_use] 
    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rgba: vec![0; width * height * 4],
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe { clear_surface(self.rgba.as_mut_ptr(), self.width * self.height * 4, color) };
    }
}

pub fn draw_rect(
    surface: &mut Surface,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    color: Color,
    blend: bool,
) {
    let mut surf = Surface::new_empty(width, height);
    surf.clear(color);
    blit_premultiplied_clipped(surface, x, y, width, height, &surf.rgba, blend);
}

pub fn blit_premultiplied_clipped(
    dest: &mut Surface,
    dest_x: i32,
    dest_y: i32,
    src_w: usize,
    src_h: usize,
    src_rgba: &[u8],
    blend: bool,
) {
    unsafe {
        bindings::blit_premultiplied_clipped(
            dest.rgba.as_ptr(),
            dest.width,
            dest.height,
            dest_x,
            dest_y,
            src_w,
            src_h,
            src_rgba.as_ptr(),
            blend,
        );
    }
}
