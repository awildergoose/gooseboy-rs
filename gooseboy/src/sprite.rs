use crate::framebuffer::{Surface, get_framebuffer_surface_mut};

#[derive(Clone)]
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

pub fn blit_premultiplied_clipped(
    surface: &mut Surface,
    dest_x: i32,
    dest_y: i32,
    src_w: usize,
    src_h: usize,
    src_rgba: &[u8],
    blend: bool,
) {
    let surf_w = surface.width as i32;
    let surf_h = surface.height as i32;

    if src_w == 0 || src_h == 0 {
        return;
    }

    let src_left = dest_x;
    let src_top = dest_y;
    let src_right = dest_x + src_w as i32;
    let src_bottom = dest_y + src_h as i32;

    let vis_left = src_left.max(0);
    let vis_top = src_top.max(0);
    let vis_right = src_right.min(surf_w);
    let vis_bottom = src_bottom.min(surf_h);

    if vis_left >= vis_right || vis_top >= vis_bottom {
        return;
    }

    let start_src_x = (vis_left - dest_x) as usize;
    let start_src_y = (vis_top - dest_y) as usize;

    let vis_w = (vis_right - vis_left) as usize;
    let vis_h = (vis_bottom - vis_top) as usize;

    for row in 0..vis_h {
        let dst_y = (vis_top as usize) + row;
        let src_row = (start_src_y + row) * src_w * 4;
        for col in 0..vis_w {
            let dst_x = (vis_left as usize) + col;
            let sidx = src_row + (start_src_x + col) * 4;
            let didx = (dst_y * surface.width + dst_x) * 4;

            let sa = src_rgba[sidx + 3];
            if sa == 0 {
                continue;
            }

            if !blend || sa == 255 {
                surface.rgba[didx..didx + 4].copy_from_slice(&src_rgba[sidx..sidx + 4]);
                continue;
            }

            let sa_u32 = sa as u32;
            let inv = 255u32 - sa_u32;

            let sr = src_rgba[sidx] as u32;
            let sg = src_rgba[sidx + 1] as u32;
            let sb = src_rgba[sidx + 2] as u32;

            let dr = surface.rgba[didx] as u32;
            let dg = surface.rgba[didx + 1] as u32;
            let db = surface.rgba[didx + 2] as u32;
            let da = surface.rgba[didx + 3] as u32;

            let out_r = sr + ((dr * inv + 127) / 255);
            let out_g = sg + ((dg * inv + 127) / 255);
            let out_b = sb + ((db * inv + 127) / 255);
            let out_a = sa_u32 + ((da * inv + 127) / 255);

            surface.rgba[didx] = out_r.min(255) as u8;
            surface.rgba[didx + 1] = out_g.min(255) as u8;
            surface.rgba[didx + 2] = out_b.min(255) as u8;
            surface.rgba[didx + 3] = out_a.min(255) as u8;
        }
    }
}
