#![no_main]

use gooseboy::framebuffer::init_fb;

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::update]
fn update(_nano_time: i64) {}
