use glam::{Mat3, Vec2};
use gooseboy::color::Color;

#[inline]
fn sample_bilinear(input: &[u8], width: usize, height: usize, x: f32, y: f32) -> [u8; 4] {
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

    let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;

    let mut out = [0u8; 4];
    for i in 0..4 {
        let c00f = c00[i] as f32;
        let c10f = c10[i] as f32;
        let c01f = c01[i] as f32;
        let c11f = c11[i] as f32;

        let top = lerp(c00f, c10f, fx);
        let bottom = lerp(c01f, c11f, fx);
        out[i] = lerp(top, bottom, fy).round() as u8;
    }
    out
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
) -> (usize, usize, Vec<u8>) {
    let (out_width, out_height) = get_output_dimensions(width, height);

    let mut output = vec![0u8; out_width * out_height * 4];
    let inv = transform.inverse();

    for y in 0..out_height {
        for x in 0..out_width {
            let uv = inv * Vec2::new(x as f32 + 0.5, y as f32 + 0.5).extend(1.0);
            let sx = uv.x;
            let sy = uv.y;

            if sx >= 0.0 && sx < width as f32 && sy >= 0.0 && sy < height as f32 {
                let color = sample_bilinear(input, width, height, sx, sy);
                let dst_idx = (y * out_width + x) * 4;
                output[dst_idx..dst_idx + 4].copy_from_slice(&color);
            }
        }
    }

    (out_width, out_height, output)
}

pub fn tint_rgba(pixels: &mut [u8], tint: Color) {
    for px in pixels.chunks_exact_mut(4) {
        px[0] = ((px[0] as u16 * tint.r as u16) / 255) as u8;
        px[1] = ((px[1] as u16 * tint.g as u16) / 255) as u8;
        px[2] = ((px[2] as u16 * tint.b as u16) / 255) as u8;
        px[3] = ((px[3] as u16 * tint.a as u16) / 255) as u8;
    }
}
