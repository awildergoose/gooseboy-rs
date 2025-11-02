#![no_main]

use std::{
    sync::{LazyLock, Mutex},
    time::Duration,
};

use crate::{
    audio::Audio,
    color::Color,
    framebuffer::{clear_framebuffer, init_fb},
    input::is_key_just_pressed,
    keys::KEY_L,
    text::{draw_char, draw_text},
    timer::Timer,
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
pub mod timer;

static SOUND: LazyLock<Audio> = make_audio!(test);
static SOUND_TIMER: LazyLock<Mutex<Timer>> = make_timer!(Duration::from_secs(1));

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init_fb();
}

#[unsafe(no_mangle)]
pub extern "C" fn update(nano_time: i64) {
    let elapsed = Duration::from_nanos(nano_time as u64);

    clear_framebuffer(Color::BLACK);
    draw_text(0, 0, format!("time: {}", nano_time).as_str(), Color::WHITE);

    if SOUND_TIMER.lock().unwrap().tick(elapsed) {
        SOUND.play();
    }

    if is_key_just_pressed(KEY_L) {
        draw_char(0, 0, b'L', Color::WHITE);
    }
}
