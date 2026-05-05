#[cfg(feature = "audio")]
pub mod audio;
pub mod bindings;
#[cfg(feature = "gpu")]
pub mod camera;
#[cfg(feature = "framebuffer")]
pub mod color;
pub mod error;
#[cfg(feature = "framebuffer")]
pub mod font;
#[cfg(feature = "framebuffer")]
pub mod framebuffer;
#[cfg(feature = "gpu")]
pub mod gpu;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "input")]
pub mod keys;
pub mod mem;
mod panic;
#[cfg(feature = "rand")]
pub mod rand;
pub mod runtime;
#[cfg(feature = "framebuffer")]
pub mod sprite;
#[cfg(feature = "storage")]
pub mod storage;
pub mod system;
#[cfg(feature = "framebuffer")]
pub mod text;
pub mod timer;
pub mod unsafe_casts;

pub use gooseboy_macros::{gpu_main, main, update};
#[cfg(any(feature = "gpu", feature = "framebuffer"))]
pub use vek::{self, Aabb, Mat3, Mat4, Rect, Vec2, Vec3, Vec4};

#[doc(hidden)]
pub fn __internal_main() {
    panic::set_panic_handler();
}

#[doc(hidden)]
pub const fn __internal_update(_nano_time: i64) {}
#[doc(hidden)]
pub const fn __internal_gpu_main() {}

#[doc(hidden)]
pub fn __internal_caught_unwind<R>(res: Result<R, Box<dyn std::any::Any + Send>>) {
    if let Err(payload) = res {
        log!("caught unwind: {:?}", payload);
    }
}
