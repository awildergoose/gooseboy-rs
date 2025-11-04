#![no_main]

use std::sync::{LazyLock, Mutex};

use gooseboy::audio::Audio;
use gooseboy::color::Color;
use gooseboy::framebuffer::{clear_framebuffer, init_fb};
use gooseboy::input::is_key_just_pressed;
use gooseboy::keys::{KEY_F, KEY_N};
use gooseboy::make_audio;

static SOUND: LazyLock<Mutex<Audio>> = make_audio!(test);

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    let mut audio = SOUND.lock().unwrap();

    if is_key_just_pressed(KEY_F) {
        audio.play();
    } else if is_key_just_pressed(KEY_N) {
        audio.stop();
    }

    clear_framebuffer(Color::BLACK);
    panic!("bih");
}
