pub mod audio;
pub mod bindings;
pub mod color;
pub mod font;
pub mod framebuffer;
pub mod input;
pub mod keys;
pub mod mem;
pub mod panic;
pub mod rand;
pub mod runtime;
pub mod sprite;
pub mod storage;
pub mod system;
pub mod text;
pub mod timer;

pub use gooseboy_macros::main;
pub use gooseboy_macros::update;

pub fn __internal_main() {
    panic::set_panic_handler();
}

pub fn __internal_update(_nano_time: i64) {}

pub fn __internal_caught_unwind<R>(res: Result<R, Box<dyn std::any::Any + Send>>) {
    if let Err(payload) = res {
        log!("caught unwind: {:?}", payload);
    }
}
