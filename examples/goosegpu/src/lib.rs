#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommand, GpuCommandBuffer, Vertex, gpu_read_value};
use gooseboy::log;
use gooseboy::text::draw_text_formatted;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

mod sprites {
    include!("generated/sprites.rs");
}

const TEAPOT_OBJ: &str = include_str!("../teapot.obj");
const CUBE_OBJ: &str = include_str!("../cube.obj");

fn load_obj(obj_data: &str) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let mut positions = Vec::new();

    for line in obj_data.lines() {
        let line = line.trim();

        if let Some(parts) = line.strip_prefix("v ") {
            let mut parts = parts.split_whitespace();
            let x: f32 = parts.next().unwrap().parse().unwrap();
            let y: f32 = parts.next().unwrap().parse().unwrap();
            let z: f32 = parts.next().unwrap().parse().unwrap();
            positions.push([x, y, z]);
        } else if let Some(indices) = line.strip_prefix("f ") {
            let indices: Vec<usize> = indices
                .split_whitespace()
                .map(|s| s.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                .collect();

            if indices.len() >= 3 {
                let first = indices[0];
                for i in 1..indices.len() - 1 {
                    for &idx in &[first, indices[i], indices[i + 1]] {
                        let p = positions[idx];
                        vertices.push(Vertex::new(p[0], p[1], p[2], 0.0, 0.0));
                    }
                }
            }
        }
    }

    vertices
}

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::gpu_main]
fn gpu_main() {
    let obj_vertices = load_obj(TEAPOT_OBJ);
    let obj2_vertices = load_obj(CUBE_OBJ);
    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(GpuCommand::PushRecord);
    log!("pushing {} vertices", obj_vertices.len());
    for vertex in obj_vertices {
        buffer.insert(GpuCommand::EmitVertex(vertex));
    }
    buffer.insert(GpuCommand::PopRecord);

    buffer.insert(GpuCommand::PushRecord);
    log!("push2 {} vertices", obj2_vertices.len());
    for vertex in obj2_vertices {
        buffer.insert(GpuCommand::EmitVertex(vertex));
    }
    buffer.insert(GpuCommand::PopRecord);

    buffer.insert(GpuCommand::RegisterTexture {
        rgba: &[0xFF, 0x00, 0x00, 0xFF], //spr.rgba,
        w: 1,                            //spr.width as u32,
        h: 1,                            //spr.height as u32,
    });

    buffer.upload();
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    clear_framebuffer(Color::TRANSPARENT);
    draw_text_formatted(
        0,
        0,
        format!(
            "GPU 0: {:#?}\nGPU 1: {:#?}",
            gpu_read_value::<u32>(0),
            gpu_read_value::<u32>(4)
        ),
        Color::RED,
    );

    sprites::ICON.blit(0, 0);

    let mut buffer = GpuCommandBuffer::new();
    buffer.insert(GpuCommand::BindTexture(0));
    buffer.insert(GpuCommand::DrawRecorded(0));
    buffer.upload();
}
