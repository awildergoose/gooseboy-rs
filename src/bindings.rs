#[link(wasm_import_module = "console")]
unsafe extern "C" {
    pub fn log(ptr: i32, len: i32);
}

#[link(wasm_import_module = "framebuffer")]
unsafe extern "C" {
    pub fn get_framebuffer_width() -> usize;
    pub fn get_framebuffer_height() -> usize;
    pub fn clear(color: i32);
}

#[link(wasm_import_module = "memory")]
unsafe extern "C" {
    pub fn mem_fill(addr: i32, len: i32, value: i32);
}

#[link(wasm_import_module = "input")]
unsafe extern "C" {
    pub fn get_key(key: i32) -> bool;
    pub fn get_mouse_button(btn: i32) -> bool;
    pub fn get_mouse_x() -> i32;
    pub fn get_mouse_y() -> i32;
}
