use glam::{Mat3, Vec2};
use gooseboy::color::Color;

#[derive(Clone, Copy, Debug)]
pub enum Resample {
    Nearest,
    Bilinear,
}

fn premultiply_rgba_inplace(pixels: &mut [u8]) {
    for px in pixels.chunks_exact_mut(4) {
        let a = px[3] as u32;
        if a == 255 {
            continue;
        }
        px[0] = ((px[0] as u32 * a) / 255) as u8;
        px[1] = ((px[1] as u32 * a) / 255) as u8;
        px[2] = ((px[2] as u32 * a) / 255) as u8;
    }
}

#[inline]
fn sample_nearest(input: &[u8], width: usize, height: usize, x: f32, y: f32) -> [u8; 4] {
    let xi = x.round().max(0.0) as i32;
    let yi = y.round().max(0.0) as i32;
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

    let clamp = |v: i32, max: usize| v.max(0).min(max as i32 - 1);
    let x0 = clamp(x0, width);
    let x1 = clamp(x1, width);
    let y0 = clamp(y0, height);
    let y1 = clamp(y1, height);

    let idx = |xx: i32, yy: i32| ((yy as usize) * width + (xx as usize)) * 4;
    let c00 = &input[idx(x0, y0)..idx(x0, y0) + 4];
    let c10 = &input[idx(x1, y0)..idx(x1, y0) + 4];
    let c01 = &input[idx(x0, y1)..idx(x0, y1) + 4];
    let c11 = &input[idx(x1, y1)..idx(x1, y1) + 4];

    let to_f = |c: &[u8]| {
        let a = c[3] as f32 / 255.0;
        [
            (c[0] as f32 / 255.0) * a,
            (c[1] as f32 / 255.0) * a,
            (c[2] as f32 / 255.0) * a,
            a,
        ]
    };

    let s00 = to_f(c00);
    let s10 = to_f(c10);
    let s01 = to_f(c01);
    let s11 = to_f(c11);

    let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;

    let mut out = [0f32; 4];
    for i in 0..4 {
        let top = lerp(s00[i], s10[i], fx);
        let bottom = lerp(s01[i], s11[i], fx);
        out[i] = lerp(top, bottom, fy);
    }

    [
        (out[0].clamp(0.0, 1.0) * 255.0).round() as u8,
        (out[1].clamp(0.0, 1.0) * 255.0).round() as u8,
        (out[2].clamp(0.0, 1.0) * 255.0).round() as u8,
        (out[3].clamp(0.0, 1.0) * 255.0).round() as u8,
    ]
}

pub fn get_output_dimensions(width: usize, height: usize) -> (usize, usize) {
    let diag = ((width * width + height * height) as f32).sqrt();
    let out_width = diag.ceil() as usize;
    let out_height = diag.ceil() as usize;
    (out_width, out_height)
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

    let working: Vec<u8>;
    let src = if premultiply_input {
        working = {
            let mut v = input.to_vec();
            premultiply_rgba_inplace(&mut v);
            v
        };
        &working[..]
    } else {
        input
    };

    let inv = transform.inverse();
    let mut output = vec![0u8; out_w * out_h * 4];

    for oy in 0..out_h {
        for ox in 0..out_w {
            let wx = (min_x + ox as i32) as f32 + 0.5;
            let wy = (min_y + oy as i32) as f32 + 0.5;
            let src_uv = inv * Vec2::new(wx, wy).extend(1.0);
            let sx = src_uv.x;
            let sy = src_uv.y;

            if sx >= 0.0 && sx < width as f32 && sy >= 0.0 && sy < height as f32 {
                let color = match resample {
                    Resample::Nearest => sample_nearest(src, width, height, sx, sy),
                    Resample::Bilinear => sample_bilinear_premult(src, width, height, sx, sy),
                };
                let dst_idx = (oy * out_w + ox) * 4;
                output[dst_idx..dst_idx + 4].copy_from_slice(&color);
            } else {
                // leave transparent
            }
        }
    }

    (out_w, out_h, min_x, min_y, output)
}

pub fn tint_rgba(pixels: &mut [u8], tint: Color) {
    // micro-optimization
    if tint.r == 255 && tint.g == 255 && tint.b == 255 {
        return;
    }

    for px in pixels.chunks_exact_mut(4) {
        px[0] = ((px[0] as u16 * tint.r as u16) / 255) as u8;
        px[1] = ((px[1] as u16 * tint.g as u16) / 255) as u8;
        px[2] = ((px[2] as u16 * tint.b as u16) / 255) as u8;
        px[3] = ((px[3] as u16 * tint.a as u16) / 255) as u8;
    }
}
