use crate::{bindings, color::Color};

static mut WIDTH: usize = 0;
static mut HEIGHT: usize = 0;
static mut FB_SIZE: usize = 0;
pub static mut FRAMEBUFFER: Vec<u8> = vec![];

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn get_framebuffer_ptr() -> *const u8 {
    unsafe { FRAMEBUFFER.as_ptr() }
}

pub fn init_fb() {
    unsafe {
        WIDTH = crate::bindings::get_framebuffer_width();
        HEIGHT = crate::bindings::get_framebuffer_height();
        FB_SIZE = WIDTH * HEIGHT * 4;
    }

    unsafe {
        FRAMEBUFFER = vec![0u8; FB_SIZE];
    }
}

pub fn get_pixel_index(x: usize, y: usize) -> Option<usize> {
    if x >= unsafe { WIDTH } || y >= unsafe { HEIGHT } {
        return None;
    }

    let row = y.checked_mul(unsafe { WIDTH })?;
    let pos = row.checked_add(x)?;
    pos.checked_mul(4)
}

pub fn set_pixel(x: usize, y: usize, color: Color) {
    if let Some(index) = get_pixel_index(x, y) {
        unsafe { color.blit(index) };
    }
}

pub fn get_framebuffer_width() -> usize {
    unsafe { WIDTH }
}

pub fn get_framebuffer_height() -> usize {
    unsafe { HEIGHT }
}

pub fn clear_framebuffer(color: Color) {
    let color_val = ((color.a as u32) << 24)
        | ((color.b as u32) << 16)
        | ((color.g as u32) << 8)
        | (color.r as u32);
    unsafe { bindings::clear_framebuffer(color_val as i32) };
}
