#![no_main]

use std::sync::LazyLock;

use crate::{
    audio::Audio,
    color::Color,
    framebuffer::{clear_framebuffer, init_fb},
    input::is_key_down,
    keys::KEY_L,
    text::{draw_char, draw_text},
};

pub mod audio;
pub mod bindings;
pub mod color;
pub mod font;
pub mod framebuffer;
pub mod input;
pub mod keys;
pub mod mem;
pub mod runtime;
pub mod text;

pub static SOUND: LazyLock<Audio> = make_audio!(test);
static mut KEY_L_PREV: bool = false;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init_fb();
}

#[unsafe(no_mangle)]
pub extern "C" fn update(nano_time: i64) {
    clear_framebuffer(Color::BLACK);
    draw_text(0, 0, format!("time: {}", nano_time).as_str(), Color::WHITE);

    let key_pressed = is_key_down(KEY_L);

    if key_pressed && unsafe { !KEY_L_PREV } {
        draw_char(0, 0, b'L', Color::WHITE);

        SOUND.play();
    }

    unsafe { KEY_L_PREV = key_pressed };
}
