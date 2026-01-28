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

        let a32 = u32::from(a);
        px[0] = ((u32::from(px[0]) * a32 + 127) / 255) as u8;
        px[1] = ((u32::from(px[1]) * a32 + 127) / 255) as u8;
        px[2] = ((u32::from(px[2]) * a32 + 127) / 255) as u8;
    }
}

#[inline]
fn sample_nearest(input: &[u8], width: usize, height: usize, x: f32, y: f32) -> [u8; 4] {
    let xi = (x + 0.5) as i32;
    let yi = (y + 0.5) as i32;
    let xx = xi.max(0).min(width as i32 - 1) as usize;
    let yy = yi.max(0).min(height as i32 - 1) as usize;
    let idx = (yy * width + xx) * 4;

    unsafe {
        [
            *input.get_unchecked(idx),
            *input.get_unchecked(idx + 1),
            *input.get_unchecked(idx + 2),
            *input.get_unchecked(idx + 3),
        ]
    }
}

#[inline]
fn sample_bilinear_premult(input: &[u8], width: usize, height: usize, x: f32, y: f32) -> [u8; 4] {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;

    let x0 = x0.max(0).min(width as i32 - 1) as usize;
    let x1 = x1.max(0).min(width as i32 - 1) as usize;
    let y0 = y0.max(0).min(height as i32 - 1) as usize;
    let y1 = y1.max(0).min(height as i32 - 1) as usize;

    let idx00 = y0 * width + x0;
    let idx10 = y0 * width + x1;
    let idx01 = y1 * width + x0;
    let idx11 = y1 * width + x1;

    unsafe {
        let c00 = input.get_unchecked(idx00 * 4..);
        let c10 = input.get_unchecked(idx10 * 4..);
        let c01 = input.get_unchecked(idx01 * 4..);
        let c11 = input.get_unchecked(idx11 * 4..);

        let w00 = (1.0 - fx) * (1.0 - fy);
        let w10 = fx * (1.0 - fy);
        let w01 = (1.0 - fx) * fy;
        let w11 = fx * fy;

        [
            (f32::from(c11[0]).mul_add(w11, f32::from(c01[0]).mul_add(w01, f32::from(c00[0]).mul_add(w00, f32::from(c10[0]) * w10)))
                + 0.5) as u8,
            (f32::from(c11[1]).mul_add(w11, f32::from(c01[1]).mul_add(w01, f32::from(c00[1]).mul_add(w00, f32::from(c10[1]) * w10)))
                + 0.5) as u8,
            (f32::from(c11[2]).mul_add(w11, f32::from(c01[2]).mul_add(w01, f32::from(c00[2]).mul_add(w00, f32::from(c10[2]) * w10)))
                + 0.5) as u8,
            (f32::from(c11[3]).mul_add(w11, f32::from(c01[3]).mul_add(w01, f32::from(c00[3]).mul_add(w00, f32::from(c10[3]) * w10)))
                + 0.5) as u8,
        ]
    }
}

#[must_use] 
pub fn get_output_dimensions(width: usize, height: usize) -> (usize, usize) {
    let diag = ((width * width + height * height) as f32).sqrt();
    let out = diag.ceil() as usize;
    (out, out)
}

#[must_use] 
pub fn transform_rgba(
    input: &[u8],
    width: usize,
    height: usize,
    transform: Mat3,
    resample: Resample,
    premultiply_input: bool,
) -> (usize, usize, i32, i32, Vec<u8>) {
    let (min, max) = compute_bounds(width, height, transform);

    let min_x = min.x as i32;
    let min_y = min.y as i32;
    let max_x = max.x as i32;
    let max_y = max.y as i32;

    let out_w = (max_x - min_x).max(0) as usize;
    let out_h = (max_y - min_y).max(0) as usize;

    if out_w == 0 || out_h == 0 {
        return (0, 0, min_x, min_y, vec![]);
    }

    let working = if premultiply_input {
        let mut v = input.to_vec();
        premultiply_rgba_inplace(&mut v);
        Some(v)
    } else {
        None
    };
    let src = working.as_deref().unwrap_or(input);

    let inv = transform.inverse();
    let mut output = vec![0u8; out_w * out_h * 4];

    let inv_cols = inv.to_cols_array_2d();
    let (a, b, c, d, e, f) = (
        inv_cols[0][0],
        inv_cols[1][0],
        inv_cols[2][0],
        inv_cols[0][1],
        inv_cols[1][1],
        inv_cols[2][1],
    );

    match resample {
        Resample::Nearest => transform_nearest_fast(
            src,
            width,
            height,
            &mut output,
            out_w,
            out_h,
            min_x,
            min_y,
            a,
            b,
            c,
            d,
            e,
            f,
        ),
        Resample::Bilinear => transform_bilinear_fast(
            src,
            width,
            height,
            &mut output,
            out_w,
            out_h,
            min_x,
            min_y,
            a,
            b,
            c,
            d,
            e,
            f,
        ),
    }

    (out_w, out_h, min_x, min_y, output)
}

#[inline(never)]
fn compute_bounds(width: usize, height: usize, transform: Mat3) -> (Vec2, Vec2) {
    let corners = [
        Vec2::new(0.0, 0.0),
        Vec2::new(width as f32, 0.0),
        Vec2::new(0.0, height as f32),
        Vec2::new(width as f32, height as f32),
    ];

    let mut min = Vec2::splat(f32::INFINITY);
    let mut max = Vec2::splat(f32::NEG_INFINITY);

    for &c in &corners {
        let tc = transform * c.extend(1.0);
        min = min.min(tc.truncate());
        max = max.max(tc.truncate());
    }

    (min, max)
}

#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn transform_nearest_fast(
    src: &[u8],
    width: usize,
    height: usize,
    output: &mut [u8],
    out_w: usize,
    out_h: usize,
    min_x: i32,
    min_y: i32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
) {
    let width_f = width as f32;
    let height_f = height as f32;
    let min_xf = min_x as f32;
    let min_yf = min_y as f32;

    for oy in 0..out_h {
        let wy = min_yf + oy as f32 + 0.5;
        let row_start = oy * out_w * 4;

        for ox in 0..out_w {
            let wx = min_xf + ox as f32 + 0.5;

            let sx = a.mul_add(wx, b * wy) + c;
            let sy = d.mul_add(wx, e * wy) + f;

            if sx >= 0.0 && sx < width_f && sy >= 0.0 && sy < height_f {
                let color = sample_nearest(src, width, height, sx, sy);
                let dst_idx = row_start + ox * 4;

                unsafe {
                    *output.get_unchecked_mut(dst_idx) = color[0];
                    *output.get_unchecked_mut(dst_idx + 1) = color[1];
                    *output.get_unchecked_mut(dst_idx + 2) = color[2];
                    *output.get_unchecked_mut(dst_idx + 3) = color[3];
                }
            }
        }
    }
}

#[inline(never)]
#[allow(clippy::too_many_arguments)]
fn transform_bilinear_fast(
    src: &[u8],
    width: usize,
    height: usize,
    output: &mut [u8],
    out_w: usize,
    out_h: usize,
    min_x: i32,
    min_y: i32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
) {
    let width_f = width as f32;
    let height_f = height as f32;
    let min_xf = min_x as f32;
    let min_yf = min_y as f32;

    for oy in 0..out_h {
        let wy = min_yf + oy as f32 + 0.5;
        let row_start = oy * out_w * 4;

        for ox in 0..out_w {
            let wx = min_xf + ox as f32 + 0.5;

            let sx = a.mul_add(wx, b * wy) + c;
            let sy = d.mul_add(wx, e * wy) + f;

            if sx >= 0.0 && sx < width_f && sy >= 0.0 && sy < height_f {
                let color = sample_bilinear_premult(src, width, height, sx, sy);
                let dst_idx = row_start + ox * 4;

                unsafe {
                    *output.get_unchecked_mut(dst_idx) = color[0];
                    *output.get_unchecked_mut(dst_idx + 1) = color[1];
                    *output.get_unchecked_mut(dst_idx + 2) = color[2];
                    *output.get_unchecked_mut(dst_idx + 3) = color[3];
                }
            }
        }
    }
}

pub fn tint_rgba(pixels: &mut [u8], tint: Color) {
    if tint == Color::WHITE {
        return;
    }

    let r = u32::from(tint.r);
    let g = u32::from(tint.g);
    let b = u32::from(tint.b);
    let a = u32::from(tint.a);

    let chunks = pixels.chunks_exact_mut(4);
    for px in chunks {
        px[0] = ((u32::from(px[0]) * r + 127) / 255) as u8;
        px[1] = ((u32::from(px[1]) * g + 127) / 255) as u8;
        px[2] = ((u32::from(px[2]) * b + 127) / 255) as u8;
        px[3] = ((u32::from(px[3]) * a + 127) / 255) as u8;
    }
}
