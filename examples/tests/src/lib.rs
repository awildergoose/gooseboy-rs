#![no_main]

mod state;
mod suite;
mod tests;
mod ui;

use gooseboy::framebuffer::init_fb;

use crate::{state::RESULTS, suite::run_tests};

#[gooseboy::main]
fn main() {
    init_fb();

    RESULTS.lock().unwrap().clear();
    run_tests();
}

#[gooseboy::update]
fn update(nano_time: i64) {
    #[allow(path_statements)]
    nano_time;
    ui::render();
}
