#![no_main]

use std::sync::{LazyLock, Mutex};

use glam::{Mat3, Vec2};
use gooseboy::color::Color;
use gooseboy::framebuffer::init_fb;
use gooseboy::text::{get_text_height, get_text_width};

use crate::renderer::Command;

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

fn make_transform_for_object(angle: f32, pos: Vec2, size: Vec2) -> Mat3 {
    let center = Vec2::new(size.x / 2.0, size.y / 2.0);
    let t1 = Mat3::from_translation(-center);
    let r = Mat3::from_angle(angle);
    let t2 = Mat3::from_translation(center);
    let translation = Mat3::from_translation(pos);
    translation * (t2 * r * t1)
}

#[gooseboy::update]
fn update(nano_time: i64) {
    unsafe {
        let dt = (nano_time - LAST_NANO_TIME) as f32 / 1_000_000_000.0; // convert to seconds
        ANGLE += dt;
        LAST_NANO_TIME = nano_time;
    }

    let mut binding = RENDERER.lock();
    let r = binding.as_mut().unwrap();

    r.clear(Color::BLACK);
    r.group("text sprite rect", |r| {
        let text = "Hello, world!";
        let text_sz = Vec2::new(get_text_width(text) as f32, get_text_height(text) as f32);
        let tx = make_transform_for_object(unsafe { ANGLE }, Vec2::new(50.0, 50.0), text_sz);
        r.command(Command::Text {
            transform: tx,
            text: text.to_owned(),
            color: Color::RED,
            max_width: None,
            resampling: transformer::Resample::Nearest,
        });

        let sprite_sz = Vec2::new(100.0, 100.0);
        let sprite_tx =
            make_transform_for_object(unsafe { ANGLE }, Vec2::new(50.0, 50.0), sprite_sz);
        r.command(Command::Sprite {
            transform: sprite_tx,
            id: unsafe { SPRITE_ID },
            color: Color::WHITE,
            resampling: transformer::Resample::Bilinear,
        });

        let rect_sz = Vec2::new(50.0, 50.0);
        let rect_tx = make_transform_for_object(unsafe { ANGLE }, Vec2::new(50.0, 50.0), rect_sz);
        r.command(Command::Rect {
            transform: rect_tx,
            size: rect_sz,
            color: Color::BLUE,
            resampling: transformer::Resample::Nearest,
        });
    });
    r.flush();
}
