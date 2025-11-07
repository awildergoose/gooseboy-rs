use glam::{Mat3, Vec2};
use gooseboy::color::Color;

#[derive(Clone, Copy, Debug)]
pub enum Resample {
    Nearest,
    Bilinear,
}

fn premultiply_rgba_inplace(pixels: &mut [u8]) {
    let chunks = pixels.chunks_exact_mut(4);
    for px in chunks {
        let a = px[3];
        if a == 255 {
            continue;
        }

        let a16 = a as u16;
        px[0] = ((px[0] as u16 * a16) / 255) as u8;
        px[1] = ((px[1] as u16 * a16) / 255) as u8;
        px[2] = ((px[2] as u16 * a16) / 255) as u8;
    }
}

#[inline]
fn sample_nearest(input: &[u8], width: usize, height: usize, x: f32, y: f32) -> [u8; 4] {
    let xi = x.round() as i32;
    let yi = y.round() as i32;
    let clamp = |v: i32, max: usize| v.max(0).min(max as i32 - 1);
    let xx = clamp(xi, width) as usize;
    let yy = clamp(yi, height) as usize;
    let idx = (yy * width + xx) * 4;

    [input[idx], input[idx + 1], input[idx + 2], input[idx + 3]]
}

#[inline]
fn sample_bilinear_premult(input: &[u8], width: usize, height: usize, x: f32, y: f32) -> [u8; 4] {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;
    let fx_inv = 1.0 - fx;
    let fy_inv = 1.0 - fy;

    let clamp = |v: i32, max: usize| v.max(0).min(max as i32 - 1);
    let x0 = clamp(x0, width);
    let x1 = clamp(x1, width);
    let y0 = clamp(y0, height);
    let y1 = clamp(y1, height);

    let idx00 = (y0 as usize) * width + (x0 as usize);
    let idx10 = (y0 as usize) * width + (x1 as usize);
    let idx01 = (y1 as usize) * width + (x0 as usize);
    let idx11 = (y1 as usize) * width + (x1 as usize);

    let c00 = &input[idx00 * 4..];
    let c10 = &input[idx10 * 4..];
    let c01 = &input[idx01 * 4..];
    let c11 = &input[idx11 * 4..];

    let w00 = fx_inv * fy_inv;
    let w10 = fx * fy_inv;
    let w01 = fx_inv * fy;
    let w11 = fx * fy;

    let mut out = [0f32; 4];
    for i in 0..4 {
        let val =
            c00[i] as f32 * w00 + c10[i] as f32 * w10 + c01[i] as f32 * w01 + c11[i] as f32 * w11;
        out[i] = val.clamp(0.0, 255.0);
    }

    [
        out[0].round() as u8,
        out[1].round() as u8,
        out[2].round() as u8,
        out[3].round() as u8,
    ]
}

pub fn get_output_dimensions(width: usize, height: usize) -> (usize, usize) {
    let diag = ((width * width + height * height) as f32).sqrt();
    let out = diag.ceil() as usize;
    (out, out)
}

pub fn transform_rgba(
    input: &[u8],
    width: usize,
    height: usize,
    transform: Mat3,
    resample: Resample,
    premultiply_input: bool,
) -> (usize, usize, i32, i32, Vec<u8>) {
    let corners = [
        Vec2::new(0.0, 0.0),
        Vec2::new(width as f32, 0.0),
        Vec2::new(0.0, height as f32),
        Vec2::new(width as f32, height as f32),
    ];

    let mut min = Vec2::splat(f32::INFINITY);
    let mut max = Vec2::splat(f32::NEG_INFINITY);
    for &c in &corners {
        let tc = (transform * c.extend(1.0)).truncate();
        min = min.min(tc);
        max = max.max(tc);
    }

    let min_x = min.x.floor() as i32;
    let min_y = min.y.floor() as i32;
    let max_x = max.x.ceil() as i32;
    let max_y = max.y.ceil() as i32;

    let out_w = (max_x - min_x).max(0) as usize;
    let out_h = (max_y - min_y).max(0) as usize;

    if out_w == 0 || out_h == 0 {
        return (0, 0, min_x, min_y, vec![]);
    }

    let src: Vec<u8> = if premultiply_input {
        let mut v = input.to_vec();
        premultiply_rgba_inplace(&mut v);
        v
    } else {
        input.into()
    };

    let inv = transform.inverse();
    let mut output = vec![0u8; out_w * out_h * 4];

    let min_xf = min_x as f32;
    let min_yf = min_y as f32;

    for oy in 0..out_h {
        let wy = (min_yf + oy as f32) + 0.5;
        for ox in 0..out_w {
            let wx = (min_xf + ox as f32) + 0.5;
            let src_uv = inv * Vec2::new(wx, wy).extend(1.0);
            let sx = src_uv.x;
            let sy = src_uv.y;

            if sx >= 0.0 && sx < width as f32 && sy >= 0.0 && sy < height as f32 {
                let color = match resample {
                    Resample::Nearest => sample_nearest(&src, width, height, sx, sy),
                    Resample::Bilinear => sample_bilinear_premult(&src, width, height, sx, sy),
                };
                let dst_idx = (oy * out_w + ox) * 4;
                output[dst_idx] = color[0];
                output[dst_idx + 1] = color[1];
                output[dst_idx + 2] = color[2];
                output[dst_idx + 3] = color[3];
            }
        }
    }

    (out_w, out_h, min_x, min_y, output)
}

pub fn tint_rgba(pixels: &mut [u8], tint: Color) {
    // micro-optimization
    if tint == Color::WHITE {
        return;
    }

    let r = tint.r as u16;
    let g = tint.g as u16;
    let b = tint.b as u16;
    let a = tint.a as u16;

    let chunks = pixels.chunks_exact_mut(4);
    for px in chunks {
        px[0] = ((px[0] as u16 * r) / 255) as u8;
        px[1] = ((px[1] as u16 * g) / 255) as u8;
        px[2] = ((px[2] as u16 * b) / 255) as u8;
        px[3] = ((px[3] as u16 * a) / 255) as u8;
    }
}
