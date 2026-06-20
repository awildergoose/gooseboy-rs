#![no_main]

use gooseboy::{
    camera::{get_camera_pitch, get_camera_x, get_camera_y, get_camera_yaw, get_camera_z},
    color::Color,
    framebuffer::{clear_framebuffer, init_fb},
    gpu::{GpuCommand, GpuCommandBuffer, PrimitiveType, Vertex, gpu_read_value, load_obj},
    input::{grab_mouse, is_key_down},
    keys::{KEY_F, KEY_G},
    system::convert_nano_time_to_seconds,
    text::draw_text_formatted,
    vek::Wrap,
};

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

    let vertices = load_obj(include_str!("../teapot.obj"), true);
    let vertices2 = load_obj(include_str!("../teapot.obj"), true);
    let mut buffer = GpuCommandBuffer::new();
    buffer.insert_register_sprite(&sprites::CAT);
    buffer.insert(&GpuCommand::PushRecord(PrimitiveType::Triangles));
    buffer.insert(&GpuCommand::BindTexture(0));
    buffer.insert(&GpuCommand::EmitVertices(vertices.into()));
    buffer.insert(&GpuCommand::PopRecord);

    buffer.insert(&GpuCommand::PushRecord(PrimitiveType::Triangles));
    buffer.insert(&GpuCommand::BindTexture(0));
    buffer.insert(&GpuCommand::EmitVertices(vertices2.into()));
    buffer.insert(&GpuCommand::PopRecord);

    let quad_vertices = [
        Vertex::new(-0.5, -0.5, 0.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0),
        Vertex::new(0.5, 0.5, 0.0, 1.0, 1.0),
        Vertex::new(-0.5, 0.5, 0.0, 0.0, 1.0),
    ];

    buffer.insert_register_sprite(&sprites::ICON);
    buffer.insert(&GpuCommand::PushRecord(PrimitiveType::Quads));
    buffer.insert(&GpuCommand::BindTexture(1));
    buffer.insert(&GpuCommand::EmitVertices(quad_vertices.into()));
    buffer.insert(&GpuCommand::PopRecord);

    let _ = buffer.upload();
}

#[gooseboy::update]
fn update(nano_time: i64) {
    let sens = 0.008;
    let speed = 0.5;
    let angle = (convert_nano_time_to_seconds(nano_time) * 2.0).wrapped(360.0);

    gooseboy::camera::update_debug_camera(sens, speed);

    clear_framebuffer(Color::TRANSPARENT);
    draw_text_formatted(
        0,
        0,
        format!(
            "status: {:#?}\nlast record: {:#?}\nlast texture: {:#?}\nangle: {angle}\nrot: {} {}\npos: {} {} {} ",
            gpu_read_value::<u32>(gooseboy::gpu::GB_GPU_STATUS),
            gpu_read_value::<u32>(gooseboy::gpu::GB_GPU_RECORD_ID),
            gpu_read_value::<u32>(gooseboy::gpu::GB_GPU_TEXTURE_ID),
            get_camera_yaw(),
            get_camera_pitch(),
            get_camera_x(),
            get_camera_y(),
            get_camera_z()
        ),
        Color::RED,
    );

    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(&GpuCommand::Push);
    buffer.insert(&GpuCommand::RotateAxis {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        angle: (180.0_f32).to_radians(),
    });
    buffer.insert(&GpuCommand::DrawRecorded(0));
    buffer.insert(&GpuCommand::Pop);

    if is_key_down(KEY_F) {
        buffer.insert(&GpuCommand::Push);
        buffer.insert(&GpuCommand::Translate {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        });
        buffer.insert(&GpuCommand::DrawRecorded(1));
        buffer.insert(&GpuCommand::Pop);
    }
    if is_key_down(KEY_G) {
        buffer.insert(&GpuCommand::Push);
        buffer.insert(&GpuCommand::Translate {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        });
        buffer.insert(&GpuCommand::DrawRecorded(2));
        buffer.insert(&GpuCommand::Pop);
    }

    let _ = buffer.upload();
}
