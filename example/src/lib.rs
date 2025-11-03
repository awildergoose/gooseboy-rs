#![no_main]

use black::{DepthBuffer, FragmentProgram, Interpolate, Raster, TargetBuffer, VertexProgram};
use black::{Mat4, Vec3, Vec4};
use gooseboy::audio::Audio;
use gooseboy::color::Color;
use gooseboy::framebuffer::{
    clear_framebuffer, get_framebuffer_height, get_framebuffer_width, init_fb, set_pixel,
};
use gooseboy::input::is_key_just_pressed;
use gooseboy::keys::{KEY_F, KEY_N};
use gooseboy::make_audio;
use std::sync::LazyLock;

static mut DEPTH_BUFFER: Option<DepthBuffer> = None;
static mut ANGLE: f32 = 0.0;

struct Uniform {
    mvp: Mat4,
}
struct Vertex {
    position: Vec4,
    color: Vec3,
}
#[derive(Interpolate)]
struct Varying {
    position: Vec4,
    color: Vec3,
}

struct VertexShader;
impl VertexProgram for VertexShader {
    type Uniform = Uniform;
    type Varying = Varying;
    type Vertex = Vertex;

    fn main(&self, uniform: &Uniform, vertex: &Vertex, varying: &mut Varying) -> Vec4 {
        varying.position = vertex.position;
        varying.color = vertex.color;
        vertex.position * uniform.mvp
    }
}

struct FragmentShader;
impl FragmentProgram for FragmentShader {
    type Uniform = Uniform;
    type Varying = Varying;

    fn main(&self, _u: &Uniform, v: &Varying) -> Vec4 {
        Vec4::new(v.color.x, v.color.y, v.color.z, 1.0)
    }
}

struct ColorBuffer {
    w: i32,
    h: i32,
}
impl TargetBuffer for ColorBuffer {
    fn width(&self) -> i32 {
        self.w
    }
    fn height(&self) -> i32 {
        self.h
    }

    fn set(&mut self, x: i32, y: i32, color: Vec4) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            return;
        }

        set_pixel(
            x as usize,
            y as usize,
            Color::new(
                (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                (color.w.clamp(0.0, 1.0) * 255.0) as u8,
            ),
        );
    }
}

static VERTICES: LazyLock<[Vertex; 8]> = LazyLock::new(|| {
    [
        Vertex {
            position: Vec4::new(-1.0, -1.0, -1.0, 1.0),
            color: Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec4::new(1.0, -1.0, -1.0, 1.0),
            color: Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec4::new(1.0, 1.0, -1.0, 1.0),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
        Vertex {
            position: Vec4::new(-1.0, 1.0, -1.0, 1.0),
            color: Vec3::new(1.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec4::new(-1.0, -1.0, 1.0, 1.0),
            color: Vec3::new(1.0, 0.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, -1.0, 1.0, 1.0),
            color: Vec3::new(0.0, 1.0, 1.0),
        },
        Vertex {
            position: Vec4::new(1.0, 1.0, 1.0, 1.0),
            color: Vec3::new(1.0, 1.0, 1.0),
        },
        Vertex {
            position: Vec4::new(-1.0, 1.0, 1.0, 1.0),
            color: Vec3::new(0.5, 0.5, 0.5),
        },
    ]
});

const INDICES: [[usize; 3]; 12] = [
    [0, 1, 2],
    [0, 2, 3],
    [1, 5, 6],
    [1, 6, 2],
    [5, 4, 7],
    [5, 7, 6],
    [4, 0, 3],
    [4, 3, 7],
    [3, 2, 6],
    [3, 6, 7],
    [4, 5, 1],
    [4, 1, 0],
];

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init_fb();
}

static mut SOUND: LazyLock<Audio> = make_audio!(test);

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn update(_nano_time: i64) {
    unsafe {
        if is_key_just_pressed(KEY_F) {
            SOUND.play();
        } else if is_key_just_pressed(KEY_N) {
            SOUND.stop();
        }

        clear_framebuffer(Color::BLACK);

        let w = get_framebuffer_width() as i32;
        let h = get_framebuffer_height() as i32;

        if DEPTH_BUFFER.is_none() {
            DEPTH_BUFFER = Some(DepthBuffer::new(w as usize, h as usize));
        } else {
            let recreate = DEPTH_BUFFER
                .as_ref()
                .map(|d| d.width != w as usize || d.height != h as usize)
                .unwrap_or(true);
            if recreate {
                DEPTH_BUFFER = Some(DepthBuffer::new(w as usize, h as usize));
            }
        }

        let mut depth_buffer = DepthBuffer::new(w as usize, h as usize);

        ANGLE += 0.02;
        let rot_y = Mat4::rotation_y(ANGLE);
        let rot_x = Mat4::rotation_x(ANGLE / 2.0);
        let model = rot_y * rot_x;

        let view = Mat4::look_at(
            &Vec3::new(0.0, 0.0, 3.0),
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        );

        let fov_deg = 90.0_f32;
        let proj = Mat4::perspective_fov(fov_deg, (w as f32) / (h as f32), 0.1, 100.0);

        let mvp = model * view * proj;
        let uniform = Uniform { mvp };

        let mut color_buffer = ColorBuffer { w, h };

        for tri in INDICES {
            Raster::triangle(
                &VertexShader,
                &FragmentShader,
                &mut depth_buffer,
                &mut color_buffer,
                &uniform,
                &VERTICES[tri[0]],
                &VERTICES[tri[1]],
                &VERTICES[tri[2]],
            );
        }
    }
}
