#![no_main]

use std::sync::LazyLock;

use gooseboy::audio::Audio;
use gooseboy::color::Color;
use gooseboy::framebuffer::{clear_framebuffer, init_fb};
use gooseboy::input::is_key_just_pressed;
use gooseboy::keys::{KEY_F, KEY_N};
use gooseboy::make_audio;

static mut SOUND: LazyLock<Audio> = make_audio!(test);

#[gooseboy::main]
fn main() {
    init_fb();
}

#[allow(static_mut_refs)]
#[gooseboy::update]
fn update(_nano_time: i64) {
    if is_key_just_pressed(KEY_F) {
        unsafe {
            SOUND.play();
        }
    } else if is_key_just_pressed(KEY_N) {
        unsafe {
            SOUND.stop();
        }
    }

    clear_framebuffer(Color::BLACK);
    panic!("bih");
}
