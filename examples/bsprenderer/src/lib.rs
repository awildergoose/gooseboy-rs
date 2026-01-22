#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommand, GpuCommandBuffer, Vertex, gpu_read_value};
use gooseboy::input::grab_mouse;
use gooseboy::text::draw_text_formatted;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};
use qbsp::prelude::*;

mod sprites {
    include!("generated/sprites.rs");
}

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::gpu_main]
fn gpu_main() {
    grab_mouse();

    let bsp_bytes = include_bytes!("../e1m1.bsp");
    let bsp = BspData::parse(BspParseInput {
        bsp: bsp_bytes,
        lit: None,
        settings: BspParseSettings::default(),
    })
    .expect("failed to load bsp");

    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(GpuCommand::PushRecord);
    for face in bsp.faces.iter() {
        buffer.insert(GpuCommand::Push);
        let vertices = face.vertices(&bsp);

        for v in vertices {
            buffer.insert(GpuCommand::EmitVertex(Vertex::new(v.x, v.y, v.z, 0.0, 1.0)));
        }
        buffer.insert(GpuCommand::Pop);
    }
    buffer.insert(GpuCommand::PopRecord);

    buffer.upload();
}

#[gooseboy::update]
fn update(nano_time: i64) {
    let sens = 0.008;
    let speed = 3.0;

    gooseboy::camera::update_debug_camera(sens, speed);

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
    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(GpuCommand::Push);
    buffer.insert(GpuCommand::DrawRecorded(0));
    buffer.insert(GpuCommand::Pop);
    buffer.upload();
}
