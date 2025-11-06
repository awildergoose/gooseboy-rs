#![no_main]

use glam::{Mat3, Vec2};
use gooseboy::color::Color;
use gooseboy::framebuffer::{clear_framebuffer, init_fb};
use gooseboy::sprite::blit_ex;

use crate::transformer::transform_rgba;

pub mod renderer;
pub mod transformer;

mod sprites {
    include!("generated/sprites.rs");
}

static mut ANGLE: f32 = 0.0;

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    clear_framebuffer(Color::BLACK);

    let sprite = &sprites::ICON.rgba;
    let width = sprites::ICON.width;
    let height = sprites::ICON.height;

    unsafe {
        ANGLE += 0.05;
        if ANGLE > std::f32::consts::TAU {
            ANGLE -= std::f32::consts::TAU;
        }

        let diag = ((width * width + height * height) as f32).sqrt();
        let out_w = diag.ceil() as usize;
        let out_h = diag.ceil() as usize;

        let input_center = Vec2::new(width as f32 / 2.0, height as f32 / 2.0);
        let output_center = Vec2::new(out_w as f32 / 2.0, out_h as f32 / 2.0);

        let transform = Mat3::from_translation(output_center)
            * Mat3::from_angle(ANGLE.tan())
            * Mat3::from_scale(Vec2::new(ANGLE.sin(), ANGLE.cos()))
            * Mat3::from_translation(-input_center);

        let out = transform_rgba(sprite, width, height, transform, out_w, out_h);

        let screen_x = 20;
        let screen_y = 20;
        blit_ex(screen_x, screen_y, out_w, out_h, &out, true);
    }
}
