#![no_main]

use core::f32;

use crate::{
    color::Color,
    framebuffer::{clear_framebuffer, get_framebuffer_height, get_framebuffer_width, init_fb},
};

pub mod bindings;
pub mod color;
pub mod framebuffer;
pub mod mem;
pub mod runtime;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init_fb();
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn update(nano_time: i64) {
    let w_u32 = get_framebuffer_width();
    let h_u32 = get_framebuffer_height();
    if w_u32 == 0 || h_u32 == 0 {
        return;
    }
    let width = w_u32;
    let height = h_u32;
    let wh = width * height;

    static mut PIXEL_BUF: Option<Vec<Color>> = None;
    static mut DEPTH_BUF: Option<Vec<f32>> = None;

    unsafe {
        if PIXEL_BUF.as_ref().is_none_or(|b| b.len() != wh) {
            PIXEL_BUF = Some(vec![Color::new_opaque(0, 0, 0); wh]);
        }
        if DEPTH_BUF.as_ref().is_none_or(|d| d.len() != wh) {
            DEPTH_BUF = Some(vec![f32::INFINITY; wh]);
        }
    }

    let depth = unsafe { DEPTH_BUF.as_mut().unwrap() };

    let t = nano_time as f32 * 1e-9;

    let verts = [
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
    ];

    let tris = [
        (0usize, 1usize, 2usize),
        (0, 2, 3),
        (4, 6, 5),
        (4, 7, 6),
        (0, 3, 7),
        (0, 7, 4),
        (1, 5, 6),
        (1, 6, 2),
        (0, 4, 5),
        (0, 5, 1),
        (3, 2, 6),
        (3, 6, 7),
    ];

    let rot_y = Mat3::rot_y(t * 0.9);
    let rot_x = Mat3::rot_x(t * 0.6);
    let model = rot_y.mul_mat(&rot_x);

    let cam_z = 2.5f32;
    let fov = 60.0f32.to_radians();
    let aspect = (width as f32) / (height as f32);
    let proj_factor = (fov * 0.5).tan().recip();

    let light_dir = Vec3::new(0.3, 0.5, -1.0).normalized();

    unsafe {
        mem::fill(
            depth.as_ptr() as i32,
            (depth.len() * size_of::<f32>()) as i32,
            f32::INFINITY.to_bits() as i32,
        );
    }
    clear_framebuffer(Color::new_opaque(127, 127, 127));

    let mut screen_vs: [Vec3; 8] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    ];
    let mut view_zs = [0.0f32; 8];

    for (i, v) in verts.iter().enumerate() {
        let mv = model.mul_vec(v);
        let view_z = mv.z + cam_z;
        let px = (mv.x / view_z) * proj_factor * aspect;
        let py = (mv.y / view_z) * proj_factor;
        let sx = (px * 0.5 + 0.5) * (width as f32 - 1.0);
        let sy = ((-py) * 0.5 + 0.5) * (height as f32 - 1.0);
        screen_vs[i] = Vec3::new(sx, sy, view_z);
        view_zs[i] = view_z;
    }

    let mut vnorms = [Vec3::new(0.0, 0.0, 0.0); 8];

    for &(i0, i1, i2) in tris.iter() {
        let mv0 = model.mul_vec(&verts[i0]);
        let mv1 = model.mul_vec(&verts[i1]);
        let mv2 = model.mul_vec(&verts[i2]);
        let fnorm = (mv1.sub(&mv0)).cross(&mv2.sub(&mv0));
        vnorms[i0].x += fnorm.x;
        vnorms[i0].y += fnorm.y;
        vnorms[i0].z += fnorm.z;
        vnorms[i1].x += fnorm.x;
        vnorms[i1].y += fnorm.y;
        vnorms[i1].z += fnorm.z;
        vnorms[i2].x += fnorm.x;
        vnorms[i2].y += fnorm.y;
        vnorms[i2].z += fnorm.z;
    }

    for vn in vnorms.iter_mut() {
        let len = vn.length();
        if len > 1e-6 {
            *vn = Vec3::new(vn.x / len, vn.y / len, vn.z / len);
        }
    }

    for &(i0, i1, i2) in tris.iter() {
        let p0 = screen_vs[i0];
        let p1 = screen_vs[i1];
        let p2 = screen_vs[i2];
        let edge1 = p1.sub(&p0);
        let edge2 = p2.sub(&p0);
        let face_norm_z = edge1.x * edge2.y - edge1.y * edge2.x;
        if face_norm_z >= 0.0 {
            continue;
        }

        let min_x = f32::min(f32::min(p0.x, p1.x), p2.x).floor().max(0.0) as i32;
        let max_x = f32::max(f32::max(p0.x, p1.x), p2.x)
            .ceil()
            .min((width - 1) as f32) as i32;
        let min_y = f32::min(f32::min(p0.y, p1.y), p2.y).floor().max(0.0) as i32;
        let max_y = f32::max(f32::max(p0.y, p1.y), p2.y)
            .ceil()
            .min((height - 1) as f32) as i32;

        let area = edge_func(&p0, &p1, &p2);
        if area.abs() < 1e-6 {
            continue;
        }
        let l0 = vnorms[i0].dot(&light_dir).max(0.0);
        let l1 = vnorms[i1].dot(&light_dir).max(0.0);
        let l2 = vnorms[i2].dot(&light_dir).max(0.0);

        let hue = (t * 0.15).fract();
        let (base_r, base_g, base_b) = hsv_to_rgb(hue, 0.9, 1.0);

        let iz0 = 1.0 / view_zs[i0];
        let iz1 = 1.0 / view_zs[i1];
        let iz2 = 1.0 / view_zs[i2];

        let inv_area = 1.0 / area;

        for py in min_y..=max_y {
            let base = (py as usize) * width;
            for px in min_x..=max_x {
                let sp = Vec3::new(px as f32 + 0.5, py as f32 + 0.5, 0.0);
                let w0 = edge_func(&p1, &p2, &sp) * inv_area;
                let w1 = edge_func(&p2, &p0, &sp) * inv_area;
                let w2 = edge_func(&p0, &p1, &sp) * inv_area;

                if w0 >= -0.0001 && w1 >= -0.0001 && w2 >= -0.0001 {
                    let z = w0 * view_zs[i0] + w1 * view_zs[i1] + w2 * view_zs[i2];
                    let idx = base + (px as usize);
                    if z < depth[idx] {
                        depth[idx] = z;

                        let denom = w0 * iz0 + w1 * iz1 + w2 * iz2;
                        let lit = if denom.abs() > 1e-9 {
                            (w0 * (l0 * iz0) + w1 * (l1 * iz1) + w2 * (l2 * iz2)) / denom
                        } else {
                            (l0 + l1 + l2) / 3.0
                        };

                        let r = (base_r * (0.25 + lit * 0.75)).clamp(0.0, 1.0);
                        let g = (base_g * (0.25 + lit * 0.75)).clamp(0.0, 1.0);
                        let b = (base_b * (0.25 + lit * 0.75)).clamp(0.0, 1.0);

                        unsafe {
                            Color::new_opaque(
                                (r * 255.0) as u8,
                                (g * 255.0) as u8,
                                (b * 255.0) as u8,
                            )
                            .blit(idx * 4)
                        };
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    fn sub(&self, o: &Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
    fn dot(&self, o: &Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    fn cross(&self, o: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    fn normalized(&self) -> Vec3 {
        let l = self.length();
        if l == 0.0 {
            *self
        } else {
            Vec3::new(self.x / l, self.y / l, self.z / l)
        }
    }
}

#[derive(Copy, Clone)]
struct Mat3 {
    m: [[f32; 3]; 3],
}
impl Mat3 {
    fn rot_y(a: f32) -> Mat3 {
        let (s, c) = a.sin_cos();
        Mat3 {
            m: [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
        }
    }
    fn rot_x(a: f32) -> Mat3 {
        let (s, c) = a.sin_cos();
        Mat3 {
            m: [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]],
        }
    }
    fn mul_vec(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }
    fn mul_mat(&self, o: &Mat3) -> Mat3 {
        let mut r = [[0.0f32; 3]; 3];
        for (i, r_row) in r.iter_mut().enumerate() {
            for (j, r_val) in r_row.iter_mut().enumerate() {
                *r_val =
                    self.m[i][0] * o.m[0][j] + self.m[i][1] * o.m[1][j] + self.m[i][2] * o.m[2][j];
            }
        }
        Mat3 { m: r }
    }
}

#[inline]
fn edge_func(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

#[inline]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let hh = (h.rem_euclid(1.0)) * 6.0;
    let i = hh.floor() as i32;
    let f = hh - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    match i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}
