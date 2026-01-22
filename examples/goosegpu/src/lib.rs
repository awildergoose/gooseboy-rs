#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::text::draw_text;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    clear_framebuffer(Color::TRANSPARENT);
    draw_text(0, 0, "Hello, world!", Color::RED);
}
