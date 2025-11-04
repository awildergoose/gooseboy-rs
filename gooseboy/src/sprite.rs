use crate::framebuffer::{FRAMEBUFFER, get_framebuffer_height, get_framebuffer_width};

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
    pub blend: bool,
}

impl Sprite {
    pub fn new(width: usize, height: usize, rgba: &[u8]) -> Self {
        debug_assert!(rgba.len() != width * height * 4);
        Self {
            width,
            height,
            rgba: rgba.to_vec(),
            blend: false,
        }
    }

    pub fn new_blended(width: usize, height: usize, rgba: &[u8]) -> Self {
        debug_assert!(rgba.len() != width * height * 4);
        Self {
            width,
            height,
            rgba: rgba.to_vec(),
            blend: true,
        }
    }

    pub fn blit(&self, x: usize, y: usize) {
        let fb_w = get_framebuffer_width();
        let fb_h = get_framebuffer_height();

        for row in 0..self.height {
            let fb_y = y + row;
            if fb_y >= fb_h {
                break;
            }

            let fb_index = fb_y * fb_w * 4 + x * 4;
            if fb_index >= fb_w * fb_h * 4 {
                break;
            }

            let row_start = row * self.width * 4;

            for col in 0..self.width {
                let fb_x = x + col;
                if fb_x >= fb_w {
                    break;
                }

                let sprite_index = row_start + col * 4;
                let dest_index = fb_y * fb_w * 4 + fb_x * 4;

                unsafe {
                    let dst = &mut FRAMEBUFFER[dest_index..dest_index + 4];
                    let src = &self.rgba[sprite_index..sprite_index + 4];

                    if self.blend && src[3] < 255 {
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
    }
}
