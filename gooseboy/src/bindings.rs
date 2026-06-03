//! This is used for calling the WASM host functions directly,
//! This is *not* recommended to be used, unless you know what you're doing.

/// WASM pointer
pub type Pointer = *const u8;
/// WASM mutable pointer
pub type PointerMut = *mut u8;

#[link(wasm_import_module = "console")]
unsafe extern "C" {
    /// Writes to the console.
    pub fn log(ptr: Pointer, len: i32);
}

#[cfg(feature = "framebuffer")]
#[link(wasm_import_module = "framebuffer")]
unsafe extern "C" {
    /// Returns the framebuffer width.
    pub fn get_framebuffer_width() -> usize;
    /// Returns the framebuffer height.
    pub fn get_framebuffer_height() -> usize;
    /// Clears a surface with `color`, with size being the size of the surface struct.
    /// Internally, this just does the following:
    /// ```rs
    /// for i in (0..size).step_by(4) {
    ///     ptr[i] = color;
    /// }
    /// ```
    pub fn clear_surface(ptr: Pointer, size: i32, color: i32);
    /// Blits a Surface onto another Surface, with optional blending.
    pub fn blit_premultiplied_clipped(
        dest_ptr: Pointer,
        dest_w: usize,
        dest_h: usize,
        dest_x: i32,
        dest_y: i32,
        src_w: usize,
        src_h: usize,
        src_ptr: Pointer,
        blend: bool,
    );
}

#[link(wasm_import_module = "memory")]
unsafe extern "C" {
    /// Fills a region of memory.
    pub fn mem_fill(addr: PointerMut, len: i32, value: i32);
    /// Copies a region of memory.
    pub fn mem_copy(dst: PointerMut, src: Pointer, len: i32);
}

#[cfg(feature = "input")]
#[link(wasm_import_module = "input")]
unsafe extern "C" {
    /// Returns the current key being held down.
    pub fn get_key_code() -> i32;
    /// Is `key` held down?
    pub fn get_key(key: i32) -> bool;
    /// Is `btn` held down?
    pub fn get_mouse_button(btn: i32) -> bool;
    /// Returns the mouse X position.
    pub fn get_mouse_x() -> i32;
    /// Returns the mouse Y position.
    pub fn get_mouse_y() -> i32;
    /// Returns the mouse accumulated delta X, helpful for first-person games.
    pub fn get_mouse_accumulated_dx() -> f64;
    /// Returns the mouse accumulated delta Y, helpful for first-person games.
    pub fn get_mouse_accumulated_dy() -> f64;
    /// Is the mouse grabbed?
    pub fn is_mouse_grabbed() -> bool;
    /// Grabs the mouse.
    pub fn grab_mouse();
    /// Releases the mouse.
    pub fn release_mouse();
}

#[cfg(feature = "audio")]
#[link(wasm_import_module = "audio")]
unsafe extern "C" {
    /// Plays an audio, with format being `AudioFormat::repr`, returning the audio instance id.
    pub fn play_audio(ptr: Pointer, len: i32, sample_rate: i32, format: i32) -> i64;
    /// Stops an audio instance.
    pub fn stop_audio(id: i64);
    /// Stops all running audio instances.
    pub fn stop_all_audio();
    /// Sets the volume of an audio instance.
    pub fn set_audio_volume(id: i64, volume: f32);
    /// Sets the pitch of an audio instance.
    pub fn set_audio_pitch(id: i64, pitch: f32);
    /// Is this audio instance currently playing?
    pub fn is_audio_playing(id: i64) -> bool;
}

#[cfg(feature = "storage")]
#[link(wasm_import_module = "storage")]
unsafe extern "C" {
    /// Reads from the crate storage, and returns the amount of read bytes.
    pub fn storage_read(offset: i32, ptr: PointerMut, len: i32) -> i32;
    /// Writes to the crate storage, and returns the amount of written bytes.
    pub fn storage_write(offset: i32, ptr: Pointer, len: i32) -> i32;
    /// Returns the size of the crate storage in bytes.
    pub fn storage_size() -> u32;
    /// Clears the crate storage entirely.
    pub fn storage_clear();
}

#[link(wasm_import_module = "system")]
unsafe extern "C" {
    /// Returns the time in nanoseconds since the Unix Epoch.
    pub fn get_time_nanos() -> i64;
    /// Does this crate have that permission?
    pub fn has_permission(permission: i32) -> bool;
    /// Gets the platform name, and returns the length of the string.
    pub fn get_platform_name(ptr: PointerMut) -> u32;
}

#[cfg(feature = "gpu")]
#[link(wasm_import_module = "gpu")]
unsafe extern "C" {
    /// Gets the current camera transform, and returns true if successful.
    pub fn get_camera_transform(ptr: PointerMut) -> bool;
    /// Sets the current camera transform, and returns true if successful.
    pub fn set_camera_transform(x: f32, y: f32, z: f32, yaw: f32, pitch: f32) -> bool;
    /// Submits a group of GPU commands, and returns true if successful.
    pub fn submit_gpu_commands(ptr: Pointer, count: i32) -> bool;
    /// Defers until the queued GPU commands run.
    pub fn defer_gpu();
    /// Reads from the GPU memory, and returns the amount of read bytes.
    pub fn gpu_read(offset: i32, ptr: Pointer, len: i32) -> u32;
}
