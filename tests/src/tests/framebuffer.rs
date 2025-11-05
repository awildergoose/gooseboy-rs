use crate::test;
use gooseboy::color::Color;
use gooseboy::framebuffer::{
    get_framebuffer_height, get_framebuffer_ptr, get_framebuffer_width, get_pixel_index, set_pixel,
};

pub fn test_framebuffer() {
    let w = get_framebuffer_width();
    let h = get_framebuffer_height();

    test!("fb:pixel_index_some", get_pixel_index(0, 0).is_some());
    test!("fb:pixel_index_none_x", get_pixel_index(w, 0).is_none());
    test!("fb:pixel_index_none_y", get_pixel_index(0, h).is_none());

    let col = Color::new_opaque(11, 22, 33);
    set_pixel(0, 0, col);

    unsafe {
        if let Some(idx) = get_pixel_index(0, 0) {
            let fb_ptr = get_framebuffer_ptr();
            if fb_ptr.is_null() {
                test!("fb:fb_ptr_null", false);
                return;
            }

            let b0 = *fb_ptr.add(idx);
            let b1 = *fb_ptr.add(idx + 1);
            let b2 = *fb_ptr.add(idx + 2);

            let ok = b0 == col.r && b1 == col.g && b2 == col.b;
            test!("fb:set_pixel_writes_bytes", ok);
        } else {
            test!("fb:set_pixel_skipped", false);
        }
    }
}
