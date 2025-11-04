pub mod audio;
pub mod bindings;
pub mod color;
pub mod font;
pub mod framebuffer;
pub mod input;
pub mod keys;
pub mod mem;
pub mod runtime;
pub mod storage;
pub mod text;
pub mod timer;

use std::panic;

pub use gooseboy_macros::main;
pub use gooseboy_macros::update;

pub fn __internal_main() {
    panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "<unknown>".into());

        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| &**s))
            .unwrap_or("<non-string panic>");

        log!("PANIC at {}: {}", location, payload);
    }));
}

pub fn __internal_update(_nano_time: i64) {}

pub fn __internal_caught_unwind<R>(res: Result<R, Box<dyn std::any::Any + Send>>) {
    if let Err(payload) = res {
        log!("caught unwind: {:?}", payload);
    }
}
