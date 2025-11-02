#[link(wasm_import_module = "console")]
unsafe extern "C" {
    pub(crate) fn log(ptr: i32, len: i32);
}

#[link(wasm_import_module = "framebuffer")]
unsafe extern "C" {
    pub(crate) fn get_framebuffer_width() -> usize;
    pub(crate) fn get_framebuffer_height() -> usize;
    pub(crate) fn clear(color: i32);
}

#[link(wasm_import_module = "memory")]
unsafe extern "C" {
    pub(crate) fn mem_fill(addr: i32, len: i32, value: i32);
}

#[link(wasm_import_module = "input")]
unsafe extern "C" {
    pub(crate) fn get_key(key: i32) -> bool;
    pub(crate) fn get_mouse_button(btn: i32) -> bool;
    pub(crate) fn get_mouse_x() -> i32;
    pub(crate) fn get_mouse_y() -> i32;
}

#[link(wasm_import_module = "audio")]
unsafe extern "C" {
    pub(crate) fn play_audio(ptr: i32, len: i32);
}
