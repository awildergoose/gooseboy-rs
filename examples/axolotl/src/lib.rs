#![no_main]

use std::sync::{LazyLock, Mutex};

use glam::{Mat3, Vec2};
use gooseboy::color::Color;
use gooseboy::framebuffer::init_fb;
use gooseboy::text::{get_text_height, get_text_width};

pub mod renderer;
pub mod transformer;

mod sprites {
    include!("generated/sprites.rs");
}

static RENDERER: Mutex<LazyLock<renderer::Renderer>> =
    Mutex::new(std::sync::LazyLock::new(renderer::Renderer::default));
static mut ANGLE: f32 = 0.0;
static mut LAST_NANO_TIME: i64 = 0;
static mut SPRITE_ID: usize = 0;

#[gooseboy::main]
fn main() {
    init_fb();

    let mut binding = RENDERER.lock();
    let r = binding.as_mut().unwrap();

    unsafe {
        SPRITE_ID = r.upload_sprite(&sprites::ICON);
    }
}

#[gooseboy::update]
fn update(nano_time: i64) {
    unsafe {
        let dt = (nano_time - LAST_NANO_TIME) as f32 / 1_000_000_000.0; // convert to seconds
        ANGLE += dt; // now ANGLE accumulates properly in radians per second
        LAST_NANO_TIME = nano_time;
    }

    let mut binding = RENDERER.lock();
    let r = binding.as_mut().unwrap();
    r.command(renderer::Command::Clear {
        color: Color::BLACK,
    });

    let text = "Hello, world!";
    let width = get_text_width(text) as f32;
    let height = get_text_height(text) as f32;
    let center = Vec2::new(width / 2.0, height / 2.0);
    let t1 = Mat3::from_translation(-center);
    let rr = Mat3::from_angle(unsafe { ANGLE });
    let t2 = Mat3::from_translation(center);
    let rotation = t2 * rr * t1;
    let translation = Mat3::from_translation(Vec2::new(50.0, 50.0));
    let final_transform = translation * rotation;

    r.command(renderer::Command::Text {
        transform: final_transform,
        text: text.to_owned(),
        color: Color::RED,
    });

    r.command(renderer::Command::Sprite {
        transform: final_transform,
        id: unsafe { SPRITE_ID },
        color: Color::WHITE,
    });

    r.command(renderer::Command::Rect {
        transform: final_transform,
        size: Vec2::new(50.0, 50.0),
        color: Color::BLUE,
    });

    r.flush();
}
