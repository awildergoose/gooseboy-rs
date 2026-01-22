#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommandBuffer, Vertex};
use gooseboy::text::draw_text;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

mod sprites {
    include!("generated/sprites.rs");
}

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::gpu_main]
fn gpu_main() {
    let x0 = -8.0;
    let y0 = -8.0;
    let z0 = -8.0;
    let x1 = 8.0;
    let y1 = 8.0;
    let z1 = 8.0;

    let mut buffer = GpuCommandBuffer::new();
    buffer.insert(gooseboy::gpu::GpuCommand::PushRecord);
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y0, z1, 0.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y1, z1, 0.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y1, z1, 1.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y0, z1, 1.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y0, z0, 0.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y1, z0, 0.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y1, z0, 1.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y0, z0, 1.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y0, z0, 0.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y1, z0, 0.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y1, z1, 1.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y0, z1, 1.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y0, z1, 0.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y1, z1, 0.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y1, z0, 1.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y0, z0, 1.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y1, z1, 0.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y1, z0, 0.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y1, z0, 1.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y1, z1, 1.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y0, z0, 0.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x0, y0, z1, 0.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y0, z1, 1.0, 1.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::EmitVertex(Vertex::new(
        x1, y0, z0, 1.0, 0.0,
    )));
    buffer.insert(gooseboy::gpu::GpuCommand::PopRecord);

    let spr = &sprites::ICON;
    buffer.insert(gooseboy::gpu::GpuCommand::RegisterTexture {
        ptr: spr.rgba.as_ptr(),
        w: spr.width as u32,
        h: spr.height as u32,
    });

    buffer.upload();
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    clear_framebuffer(Color::TRANSPARENT);
    draw_text(0, 0, "Hello, world!", Color::RED);

    let mut buffer = GpuCommandBuffer::new();
    buffer.insert(gooseboy::gpu::GpuCommand::BindTexture(0));
    buffer.insert(gooseboy::gpu::GpuCommand::DrawRecorded(0));
    buffer.upload();
}
