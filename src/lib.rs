#![no_main]

use std::{
    sync::{LazyLock, Mutex},
    time::Duration,
};

use crate::{
    audio::Audio,
    bindings::{storage_clear, storage_read, storage_size, storage_write},
    color::Color,
    framebuffer::{clear_framebuffer, init_fb},
    input::is_key_just_pressed,
    keys::{KEY_C, KEY_L, KEY_P},
    mem::alloc_bytes,
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

    let ptr = alloc_bytes(1);
    unsafe { storage_read(0, ptr, 1) };
    let value = unsafe { *(ptr as *const u8) };

    draw_text(
        0,
        0,
        format!(
            "size: {}\nraw: {}\nchar: {}",
            unsafe { storage_size() },
            value,
            char::from_u32(value as u32).unwrap_or('?')
        )
        .as_str(),
        Color::WHITE,
    );

    if is_key_just_pressed(KEY_L) {
        let ch: u8 = b'L';
        let ptr: *const u8 = &ch;
        unsafe {
            storage_write(0, ptr as i32, 1);
        }
    }

    if is_key_just_pressed(KEY_P) {
        let ch: u8 = b'P';
        let ptr: *const u8 = &ch;
        unsafe {
            storage_write(0, ptr as i32, 1);
        }
    }

    if is_key_just_pressed(KEY_C) {
        unsafe {
            storage_clear();
        }
    }
}
