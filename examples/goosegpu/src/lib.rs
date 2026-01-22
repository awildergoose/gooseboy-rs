#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommandBuffer, Vertex};
use gooseboy::text::draw_text;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

static mut FIRST_FRAME: bool = true;

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    if unsafe { FIRST_FRAME } {
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
        buffer.upload();

        unsafe {
            FIRST_FRAME = false;
        }
    }

    clear_framebuffer(Color::TRANSPARENT);
    draw_text(0, 0, "Hello, world!", Color::RED);

    let mut buffer = GpuCommandBuffer::new();
    buffer.insert(gooseboy::gpu::GpuCommand::DrawRecorded(0));
    buffer.upload();
}
