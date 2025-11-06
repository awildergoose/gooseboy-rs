#![no_main]

use std::ffi::{CStr, c_char};

use gooseboy::framebuffer::init_fb;
use gooseboy::log;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

type DoomExitFn = extern "C" fn(i32);
type DoomPrintFn = extern "C" fn(*const c_char);

unsafe extern "C" {
    fn doom_init(argc: i32, argv: *mut *mut c_char, flags: i32);
    fn doom_update();
    fn doom_get_framebuffer(channels: i32) -> *const u8;
    fn doom_set_print(print_fn: DoomPrintFn);
    fn doom_set_exit(cb: DoomExitFn);
}

extern "C" fn doom_exit_override(code: i32) {
    panic!("Doom tried to exit with code: {code}");
}

extern "C" fn doom_print_override(s: *const c_char) {
    if s.is_null() {
        return;
    }
    let cstr = unsafe { CStr::from_ptr(s) };
    if let Ok(msg) = cstr.to_str() {
        log!("DOOM: {}", msg);
    }
}

#[gooseboy::main]
fn main() {
    init_fb();

    let dummy = std::ptr::null_mut();
    let argv: [*mut c_char; 1] = [dummy];

    unsafe {
        doom_set_exit(doom_exit_override);
        doom_set_print(doom_print_override);
        doom_init(0, argv.as_ptr() as *mut *mut c_char, 0);
    }
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    unsafe {
        doom_update();
    }

    clear_framebuffer(Color::BLACK);
}
