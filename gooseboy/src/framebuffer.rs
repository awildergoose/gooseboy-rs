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

pub fn get_framebuffer_surface_ref() -> &'static Surface {
    let guard = get_framebuffer_surface();
    let surface_ref: &Surface = guard.as_ref().expect("surface not initialized");
    unsafe { &*(surface_ref as *const Surface) }
}

pub fn get_framebuffer_surface_mut() -> &'static mut Surface {
    let mut guard = get_framebuffer_surface();
    let surface_mut: &mut Surface = guard.as_mut().expect("surface not initialized");
    unsafe { &mut *(surface_mut as *mut Surface) }
}

pub fn init_fb() {
    unsafe {
        let width = crate::bindings::get_framebuffer_width();
        let height = crate::bindings::get_framebuffer_height();
        let mut guard = FRAMEBUFFER_SURFACE.lock().unwrap();
        *guard = Some(Surface::new_empty(width, height));
    }
}

pub fn get_pixel_index(x: usize, y: usize) -> Option<usize> {
    get_pixel_index_ex(get_framebuffer_surface_ref(), x, y)
}

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

pub fn get_framebuffer_width() -> usize {
    get_framebuffer_surface_ref().width
}

pub fn get_framebuffer_height() -> usize {
    get_framebuffer_surface_ref().height
}

pub fn get_framebuffer_size() -> usize {
    get_framebuffer_width() * get_framebuffer_height() * 4
}

pub fn clear_framebuffer(color: Color) {
    clear_surface(get_framebuffer_ptr(), get_framebuffer_size(), color);
}

pub fn clear_surface(ptr: *const u8, size: usize, color: Color) {
    let color_val = ((color.a as u32) << 24)
        | ((color.b as u32) << 16)
        | ((color.g as u32) << 8)
        | (color.r as u32);

    unsafe { bindings::clear_surface(ptr as i32, size as i32, color_val as i32) };
}

#[derive(Clone)]
pub struct Surface {
    pub rgba: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Surface {
    pub fn new(width: usize, height: usize, rgba: Vec<u8>) -> Self {
        Self {
            width,
            height,
            rgba,
        }
    }

    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rgba: vec![0; width * height * 4],
        }
    }
}
