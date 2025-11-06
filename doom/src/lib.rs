#![no_main]

use std::ffi::c_char;

use gooseboy::framebuffer::init_fb;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

unsafe extern "C" {
    fn doom_init(argc: u32, argv: *mut *mut c_char, flags: u32);
    fn doom_update();
}

#[gooseboy::main]
fn main() {
    init_fb();
    unsafe {
        doom_init(0, std::ptr::null_mut(), 0);
    }
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    unsafe {
        doom_update();
    }

    clear_framebuffer(Color::BLACK);
}
