#![no_main]

use crate::{
    bindings::{get_key, get_mouse_x, get_mouse_y},
    color::Color,
    framebuffer::{clear_framebuffer, init_fb},
    keys::KEY_L,
    text::{draw_char, draw_text},
};

pub mod bindings;
pub mod color;
pub mod font;
pub mod framebuffer;
pub mod keys;
pub mod mem;
pub mod runtime;
pub mod text;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init_fb();
}

#[unsafe(no_mangle)]
pub extern "C" fn update(nano_time: i64) {
    clear_framebuffer(Color::BLACK);
    draw_text(
        unsafe { get_mouse_x().try_into().unwrap() },
        unsafe { get_mouse_y().try_into().unwrap() },
        format!("time: {}", nano_time).as_str(),
        Color::WHITE,
    );

    if unsafe { get_key(KEY_L) } {
        draw_char(0, 0, b'L', Color::WHITE);
    }
}
