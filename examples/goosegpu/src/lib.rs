#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommand, GpuCommandBuffer, gpu_read_value, load_obj};
use gooseboy::input::grab_mouse;
use gooseboy::text::draw_text_formatted;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

mod sprites {
    include!("generated/sprites.rs");
}

const MODEL_OBJ: &str = include_str!("../teapot.obj");

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::gpu_main]
fn gpu_main() {
    grab_mouse();

    let obj_vertices = load_obj(MODEL_OBJ, false);
    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(GpuCommand::PushRecord);
    for vertex in obj_vertices {
        buffer.insert(GpuCommand::EmitVertex(vertex));
    }
    buffer.insert(GpuCommand::PopRecord);

    let spr = &sprites::CAT;

    buffer.insert(GpuCommand::RegisterTexture {
        rgba: &spr.rgba,
        w: spr.width as u32,
        h: spr.height as u32,
    });

    buffer.upload();
}

#[gooseboy::update]
fn update(nano_time: i64) {
    gooseboy::camera::update_debug_camera();

    clear_framebuffer(Color::TRANSPARENT);
    draw_text_formatted(
        0,
        0,
        format!(
            "GPU 0: {:#?}\nGPU 1: {:#?}",
            gpu_read_value::<i32>(0),
            gpu_read_value::<i32>(4)
        ),
        Color::RED,
    );

    let time_sec = (nano_time as f64 / 1_000_000_000.0) as f32;
    let angle = time_sec;
    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(GpuCommand::Push);
    buffer.insert(GpuCommand::RotateAxis {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        angle,
    });
    buffer.insert(GpuCommand::BindTexture(0));
    buffer.insert(GpuCommand::DrawRecorded(0));
    buffer.insert(GpuCommand::Pop);
    buffer.upload();
}
