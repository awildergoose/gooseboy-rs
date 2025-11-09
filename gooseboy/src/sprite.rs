use crate::{
    bindings,
    framebuffer::{Surface, get_framebuffer_surface_mut},
};

#[derive(Clone, Debug)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
    pub blend: bool,
}

impl Sprite {
    pub fn new(width: usize, height: usize, rgba: &[u8]) -> Self {
        Self {
            width,
            height,
            rgba: rgba.to_vec(),
            blend: false,
        }
    }

    pub fn new_blended(width: usize, height: usize, rgba: &[u8]) -> Self {
        Self {
            width,
            height,
            rgba: rgba.to_vec(),
            blend: true,
        }
    }

    pub fn blit(&self, x: usize, y: usize) {
        blit_ex(
            get_framebuffer_surface_mut(),
            x,
            y,
            self.width,
            self.height,
            &self.rgba,
            self.blend,
        );
    }
}

/// TODO remove this and replace it with blit_premultiplied_clipped or
/// make an entirely new java function for it
pub fn blit_ex(
    surface: &mut Surface,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    rgba: &[u8],
    blend: bool,
) {
    let surface_w = surface.width;
    let surface_h = surface.height;

    for row in 0..height {
        let surface_y = y + row;
        if surface_y >= surface_h {
            break;
        }

        let surface_index = surface_y * surface_w * 4 + x * 4;
        if surface_index >= surface_w * surface_h * 4 {
            break;
        }

        let row_start = row * width * 4;

        for col in 0..width {
            let surface_x = x + col;
            if surface_x >= surface_w {
                break;
            }

            let sprite_index = row_start + col * 4;
            let dest_index = surface_y * surface_w * 4 + surface_x * 4;

            let dst = &mut surface.rgba[dest_index..dest_index + 4];
            let src = &rgba[sprite_index..sprite_index + 4];

            if blend && src[3] < 255 {
                let a = src[3] as f32 / 255.0;
                for i in 0..3 {
                    dst[i] = ((dst[i] as f32 * (1.0 - a)) + (src[i] as f32 * a)) as u8;
                }
                dst[3] = 255;
            } else {
                dst.copy_from_slice(src);
            }
        }
    }
}

// rust extern:
// dest_ptr: *const u8
// dest_x: i32
// dest_y: i32
// src_w: usize
// src_h: usize
// src_ptr: *const u8
// blend: bool

// in java:
// void blit_premultiplied_clipped(int dest_ptr, int dest_x, int dest_y, int src_w, int src_h, int src_ptr, int blend);
pub fn blit_premultiplied_clipped(
    dest: &mut Surface,
    dest_x: i32,
    dest_y: i32,
    src_w: usize,
    src_h: usize,
    src_rgba: &[u8],
    blend: bool,
) {
    unsafe {
        bindings::blit_premultiplied_clipped(
            dest.rgba.as_ptr(),
            dest.width,
            dest.height,
            dest_x,
            dest_y,
            src_w,
            src_h,
            src_rgba.as_ptr(),
            blend,
        );
    }
}
