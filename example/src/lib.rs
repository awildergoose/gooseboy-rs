#![no_main]

mod sprites {
    include!("generated/sprites.rs");
}

use gooseboy::audio::{Audio, AudioInstance};
use gooseboy::color::Color;
use gooseboy::framebuffer::{clear_framebuffer, init_fb};
use gooseboy::input::{get_key, get_mouse_x, get_mouse_y, is_key_just_pressed};
use gooseboy::keys::{KEY_BACKSPACE, KEY_ENTER, KEY_F, KEY_GRAVE_ACCENT, KEY_N, KEY_SPACE};
use gooseboy::{make_audio, text};
use std::sync::{LazyLock, Mutex};

static mut CONSOLE_INPUT: String = String::new();
static SOUND: LazyLock<Mutex<Audio>> = make_audio!(test);
static LAST_SOUND_INSTANCE: LazyLock<Mutex<Option<AudioInstance>>> =
    LazyLock::new(|| Mutex::new(None));

#[gooseboy::main]
fn main() {
    init_fb();
}

#[allow(static_mut_refs)]
#[gooseboy::update]
fn update(_nano_time: i64) {
    let mut audio = SOUND.lock().unwrap();
    let mut last_sound = LAST_SOUND_INSTANCE.lock().unwrap();

    if is_key_just_pressed(KEY_F) {
        *last_sound = audio.play();
    } else if is_key_just_pressed(KEY_N)
        && let Some(ref mut sound) = *last_sound
    {
        sound.set_pitch(5.0);
    }

    if let Some(key) = get_key() {
        unsafe {
            match key {
                KEY_BACKSPACE => {
                    CONSOLE_INPUT.pop();
                }
                KEY_ENTER => {
                    CONSOLE_INPUT.push('\n');
                }
                KEY_SPACE..=KEY_GRAVE_ACCENT => {
                    CONSOLE_INPUT.push(key as u8 as char);
                }
                _ => {}
            }
        }
    }

    clear_framebuffer(Color::BLACK);
    sprites::ICON_GEAR.blit(get_mouse_x() as usize, get_mouse_y() as usize);

    text::draw_text_wrapped(0, 0, &unsafe { CONSOLE_INPUT.clone() }, Color::RED);
}
