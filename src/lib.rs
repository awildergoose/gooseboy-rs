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
    keys::{KEY_C, KEY_L, KEY_P},
    storage::{storage_clear, storage_read_value, storage_size, storage_write_value},
    text::draw_text,
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
pub mod storage;
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
    draw_text(0, 24, format!("time: {}", nano_time).as_str(), Color::WHITE);

    if SOUND_TIMER.lock().unwrap().tick(elapsed) {
        SOUND.play();
    }

    let value: u8 = storage_read_value(0);

    draw_text(
        0,
        0,
        format!(
            "size: {}\nraw: {}\nchar: {}",
            storage_size(),
            value,
            char::from_u32(value as u32).unwrap_or('?')
        )
        .as_str(),
        Color::WHITE,
    );

    if is_key_just_pressed(KEY_L) {
        storage_write_value(0, b'L');
    }

    if is_key_just_pressed(KEY_P) {
        storage_write_value(0, b'P');
    }

    if is_key_just_pressed(KEY_C) {
        storage_clear();
    }
}
