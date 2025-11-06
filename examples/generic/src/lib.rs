#![no_main]

use gooseboy::framebuffer::{get_framebuffer_width, init_fb};
use gooseboy::text::{draw_text, get_text_width};
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

// Every script has to have a main function, make sure to decorate it
// with gooseboy::main though, or else the script won't start
#[gooseboy::main]
fn main() {
    // Initializes the framebuffer, you are required to initialize this
    // here if you plan to draw to the screen (which is very likely)
    init_fb();
}

// This is also required in every script, the gooseboy::update is required
// here too, this function runs X times per second where X is equal to your
// maximum framerate in the options
#[gooseboy::update]
fn update(nano_time: i64) {
    // Clear out the screen, erasing everything that was there previously
    clear_framebuffer(Color::BLACK);

    // Initialize the string we want to draw to the screen, You can also use Rust's
    // String type here, with the caveat of having to clone it at draw_text
    let text = "Hello, world!";
    // Convert the time from nanoseconds to seconds
    let time_sec = nano_time as f64 / 1_000_000_000.0;
    // Get the position of the right corner and subtract the width of the text
    // to make the text fit into the screen, You can also use draw_text_wrapped
    // to automatically wrap text if it passes the end of the framebuffer
    let right_corner = (get_framebuffer_width() - get_text_width(text)) as f64;
    // Gets us an X position that smoothly moves from the left to the right using sine
    let x_pos = ((time_sec.sin() * 0.5 + 0.5) * (right_corner - 1.0)) as usize;

    // Finally, draw the text with the red color (or use Color::new(r, g, b, a) or Color::new_opaque(r, g, b))
    draw_text(x_pos, 0, text, Color::RED);
}
