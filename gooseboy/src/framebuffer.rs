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

fn pack_abgr_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32)
}

fn pack_abgr_i32(r: u8, g: u8, b: u8, a: u8) -> i32 {
    pack_abgr_u32(r, g, b, a) as i32
}

pub fn clear_framebuffer(color: Color) {
    unsafe { bindings::clear_framebuffer(pack_abgr_i32(color.r, color.g, color.b, color.a)) };
}
