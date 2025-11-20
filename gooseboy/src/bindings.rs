#[link(wasm_import_module = "console")]
unsafe extern "C" {
    pub fn log(ptr: *const u8, len: i32);
}

#[link(wasm_import_module = "framebuffer")]
unsafe extern "C" {
    pub(crate) fn get_framebuffer_width() -> usize;
    pub(crate) fn get_framebuffer_height() -> usize;
    pub(crate) fn clear_surface(ptr: *const u8, size: i32, color: i32);
    pub(crate) fn blit_premultiplied_clipped(
        dest_ptr: *const u8,
        dest_w: usize,
        dest_h: usize,
        dest_x: i32,
        dest_y: i32,
        src_w: usize,
        src_h: usize,
        src_ptr: *const u8,
        blend: bool,
    );
}

#[link(wasm_import_module = "memory")]
unsafe extern "C" {
    pub(crate) fn mem_fill(addr: *const u8, len: i32, value: i32);
    pub(crate) fn mem_copy(dst: *const u8, src: *const u8, len: i32);
}

#[link(wasm_import_module = "input")]
unsafe extern "C" {
    pub(crate) fn get_key_code() -> i32;
    pub(crate) fn get_key(key: i32) -> bool;
    pub(crate) fn get_mouse_button(btn: i32) -> bool;
    pub(crate) fn get_mouse_x() -> i32;
    pub(crate) fn get_mouse_y() -> i32;
    pub(crate) fn get_mouse_accumulated_dx() -> f64;
    pub(crate) fn get_mouse_accumulated_dy() -> f64;
    pub(crate) fn is_mouse_grabbed() -> bool;
    pub(crate) fn grab_mouse();
    pub(crate) fn release_mouse();
}

#[link(wasm_import_module = "audio")]
unsafe extern "C" {
    pub(crate) fn play_audio(ptr: *const u8, len: i32) -> i64;
    pub(crate) fn stop_audio(id: i64);
    pub(crate) fn stop_all_audio();
    pub(crate) fn set_audio_volume(id: i64, volume: f32);
    pub(crate) fn set_audio_pitch(id: i64, volume: f32);
    pub(crate) fn is_audio_playing(id: i64) -> bool;
}

#[link(wasm_import_module = "storage")]
unsafe extern "C" {
    pub(crate) fn storage_read(offset: i32, ptr: *const u8, len: i32) -> i32;
    pub(crate) fn storage_write(offset: i32, ptr: *const u8, len: i32) -> i32;
    pub(crate) fn storage_size() -> u32;
    pub(crate) fn storage_clear();
}

#[link(wasm_import_module = "system")]
unsafe extern "C" {
    pub(crate) fn get_time_nanos() -> i64;
    pub(crate) fn has_permission(permission: i32) -> bool;
}
