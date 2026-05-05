#![no_main]

mod state;
mod suite;
mod tests;
mod ui;

use crate::state::RESULTS;
use crate::suite::run_tests;
use gooseboy::framebuffer::init_fb;

#[gooseboy::main]
fn main() {
    init_fb();

    RESULTS.lock().unwrap().clear();
    run_tests();
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    ui::render();
}
