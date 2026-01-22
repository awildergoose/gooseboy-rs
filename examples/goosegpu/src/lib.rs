#![no_main]

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommand, GpuCommandBuffer, Vertex, gpu_read_value};
use gooseboy::log;
use gooseboy::text::draw_text_formatted;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

mod sprites {
    include!("generated/sprites.rs");
}

const MODEL_OBJ: &str = include_str!("../cat.obj");

fn load_obj(obj_data: &str, flip_v: bool) -> Vec<Vertex> {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut texcoords: Vec<[f32; 2]> = Vec::new();

    fn parse_index(s: &str, len: usize) -> Option<usize> {
        if s.is_empty() {
            return None;
        }

        match s.parse::<isize>() {
            Ok(i) if i > 0 => {
                let idx = (i as usize).saturating_sub(1);
                Some(idx)
            }
            Ok(i) if i < 0 => {
                let abs = (-i) as usize;

                if abs == 0 || abs > len {
                    None
                } else {
                    Some(len - abs)
                }
            }
            _ => None,
        }
    }

    for line in obj_data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix("v ") {
            let mut parts = rest.split_whitespace();
            let x: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let y: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let z: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
            positions.push([x, y, z]);
        } else if let Some(rest) = line.strip_prefix("vt ") {
            let mut parts = rest.split_whitespace();
            let u: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let v: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
            texcoords.push([u, v]);
        } else if let Some(rest) = line.strip_prefix("f ") {
            let tokens: Vec<&str> = rest.split_whitespace().collect();
            if tokens.len() < 3 {
                continue;
            }

            let mut face_indices: Vec<(Option<usize>, Option<usize>)> =
                Vec::with_capacity(tokens.len());
            for tok in tokens.iter() {
                let comps: Vec<&str> = tok.split('/').collect();
                let v_idx_opt = comps.first().and_then(|s| parse_index(s, positions.len()));
                let vt_idx_opt = comps.get(1).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        parse_index(s, texcoords.len())
                    }
                });

                face_indices.push((v_idx_opt, vt_idx_opt));
            }

            let n = face_indices.len();
            for i in 1..(n - 1) {
                let tri = [face_indices[0], face_indices[i], face_indices[i + 1]];
                for &(v_idx_opt, vt_idx_opt) in tri.iter() {
                    let pos = match v_idx_opt {
                        Some(idx) if idx < positions.len() => positions[idx],
                        _ => [0.0, 0.0, 0.0],
                    };

                    let (u, mut v) = if let Some(tidx) = vt_idx_opt {
                        if tidx < texcoords.len() {
                            let tc = texcoords[tidx];
                            (tc[0], tc[1])
                        } else {
                            (0.0f32, 0.0f32)
                        }
                    } else if positions.len() == texcoords.len() {
                        if let Some(vidx) = v_idx_opt {
                            if vidx < texcoords.len() {
                                let tc = texcoords[vidx];
                                (tc[0], tc[1])
                            } else {
                                (0.0f32, 0.0f32)
                            }
                        } else {
                            (0.0f32, 0.0f32)
                        }
                    } else {
                        (0.0f32, 0.0f32)
                    };

                    if flip_v {
                        v = 1.0 - v;
                    }

                    vertices.push(Vertex::new(pos[0], pos[1], pos[2], u, v));
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
    let obj_vertices = load_obj(MODEL_OBJ, false);
    let mut buffer = GpuCommandBuffer::new();

    buffer.insert(GpuCommand::PushRecord);
    log!("pushing {} vertices", obj_vertices.len());
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
fn update(_nano_time: i64) {
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

    sprites::ICON.blit(0, 0);

    let mut buffer = GpuCommandBuffer::new();
    buffer.insert(GpuCommand::BindTexture(0));
    buffer.insert(GpuCommand::DrawRecorded(0));
    buffer.upload();
}
