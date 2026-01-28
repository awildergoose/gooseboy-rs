use crate::test;
use gooseboy::bindings::Pointer;
use gooseboy::color::Color;
use gooseboy::framebuffer::{clear_framebuffer, get_framebuffer_ptr, get_pixel_index};
use gooseboy::sprite::Sprite;

unsafe fn read_pixel_rgba(fb_ptr: Pointer, x: usize, y: usize) -> Option<[u8; 4]> {
    if let Some(idx) = get_pixel_index(x, y) {
        let b0 = unsafe { *fb_ptr.add(idx) };
        let b1 = unsafe { *fb_ptr.add(idx + 1) };
        let b2 = unsafe { *fb_ptr.add(idx + 2) };
        let b3 = unsafe { *fb_ptr.add(idx + 3) };
        Some([b0, b1, b2, b3])
    } else {
        None
    }
}

pub fn test_sprite() {
    let bg = Color::new_opaque(10, 20, 30);
    clear_framebuffer(bg);

    let rgba_opaque = [
        1u8, 2u8, 3u8, 255u8, 4u8, 5u8, 6u8, 255u8, 7u8, 8u8, 9u8, 255u8, 10u8, 11u8, 12u8, 255u8,
    ];

    let s = Sprite::new(2, 2, &rgba_opaque);
    s.blit(1, 1);

    let fb_ptr = get_framebuffer_ptr();
    if fb_ptr.is_null() {
        test!("sprite:fb_ptr_null", false);
        return;
    }

    let mut ok = true;
    let mut expected_idx = 0usize;
    for row in 0..2 {
        for col in 0..2 {
            let px = 1 + col;
            let py = 1 + row;
            let expected = [
                rgba_opaque[expected_idx],
                rgba_opaque[expected_idx + 1],
                rgba_opaque[expected_idx + 2],
                rgba_opaque[expected_idx + 3],
            ];
            expected_idx += 4;

            let got = unsafe { read_pixel_rgba(fb_ptr, px, py) };
            match got {
                Some(g) => {
                    if g != expected {
                        ok = false;
                    }
                }
                None => ok = false,
            }
        }
    }

    test!("sprite:blit_all_pixels_opaque", ok);
}
