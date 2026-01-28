#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommand, GpuCommandBuffer, PrimitiveType, Vertex, gpu_read_value};
use gooseboy::input::grab_mouse;
use gooseboy::system::convert_nano_time_to_seconds;
use gooseboy::text::draw_text_formatted;
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
    grab_mouse();

    let quad_vertices = vec![
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0),
    ];
    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(&GpuCommand::PushRecord(PrimitiveType::Quads));
    for v in quad_vertices {
        buffer.insert(&GpuCommand::EmitVertex(v));
    }
    buffer.insert(&GpuCommand::PopRecord);

    buffer.upload();
}

#[gooseboy::update]
fn update(nano_time: i64) {
    let sens = 0.008;
    let speed = 0.5;

    gooseboy::camera::update_debug_camera(sens, speed);

    clear_framebuffer(Color::TRANSPARENT);
    draw_text_formatted(
        0,
        0,
        format!(
            "status: {:#?}\nlast record: {:#?}\nlast texture: {:#?}",
            gpu_read_value::<u32>(gooseboy::gpu::GB_GPU_STATUS),
            gpu_read_value::<u32>(gooseboy::gpu::GB_GPU_RECORD_ID),
            gpu_read_value::<u32>(gooseboy::gpu::GB_GPU_TEXTURE_ID)
        ),
        Color::RED,
    );

    let angle = convert_nano_time_to_seconds(nano_time);
    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(&GpuCommand::Push);
    buffer.insert(&GpuCommand::RotateAxis {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        angle,
    });
    buffer.insert(&GpuCommand::BindTexture(0));
    buffer.insert(&GpuCommand::DrawRecorded(0));
    buffer.insert(&GpuCommand::Pop);
    buffer.upload();
}
