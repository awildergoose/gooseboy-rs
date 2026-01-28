#![no_main]

use std::sync::LazyLock;

use gooseboy::framebuffer::init_fb;
use gooseboy::gpu::{GpuCommandBuffer, PrimitiveType, gpu_read_value};
use gooseboy::input::grab_mouse;
use gooseboy::text::draw_text_formatted;
use gooseboy::{color::Color, framebuffer::clear_framebuffer};

mod sprites {
    include!("generated/sprites.rs");
}

static mut GLOBAL_BUFFER: LazyLock<GpuCommandBuffer> = LazyLock::new(GpuCommandBuffer::new);
static mut ATLAS_BYTES: LazyLock<Vec<u8>> = LazyLock::new(Vec::new);
static mut ATLAS_SIZE: LazyLock<(u32, u32)> = LazyLock::new(|| (0, 0));

#[gooseboy::main]
fn main() {
    init_fb();
}

#[gooseboy::gpu_main]
#[allow(static_mut_refs)]
fn gpu_main() {
    use gooseboy::gpu::{GpuCommand, GpuCommandBuffer, Vertex};
    use qbsp::prelude::*;
    use std::collections::HashMap;

    grab_mouse();

    let bsp_bytes = include_bytes!("../e1m1.bsp");
    let bsp = BspData::parse(BspParseInput {
        bsp: bsp_bytes,
        lit: None,
        settings: BspParseSettings::default(),
    })
    .expect("failed to load bsp");

    let embedded: Vec<(String, qbsp::image::RgbImage)> = bsp
        .parse_embedded_textures(&QUAKE_PALETTE)
        .map(|(n, img)| (n.to_string(), img))
        .collect();

    let embedded = if embedded.is_empty() {
        vec![(
            "__fallback".to_string(),
            qbsp::image::RgbImage::from_raw(1, 1, vec![0xff, 0x00, 0xff]).unwrap(),
        )]
    } else {
        embedded
    };

    let atlas_w: u32 = embedded
        .iter()
        .map(|(_, img)| img.width())
        .max()
        .unwrap_or(1);
    let atlas_h: u32 = embedded
        .iter()
        .map(|(_, img)| img.height())
        .sum::<u32>()
        .max(1);

    let mut atlas_rgba: Vec<u8> = vec![0u8; (atlas_w * atlas_h * 4) as usize];
    let mut atlas_positions: HashMap<String, (u32, u32, u32, u32)> = HashMap::new();

    let mut cursor_y: u32 = 0;
    for (name, img) in embedded {
        let w = img.width();
        let h = img.height();

        for (x, y, pixel) in img.enumerate_pixels() {
            let dst_x = x;
            let dst_y = cursor_y + y;
            let dst_idx = ((dst_y * atlas_w + dst_x) * 4) as usize;
            atlas_rgba[dst_idx] = pixel[0];
            atlas_rgba[dst_idx + 1] = pixel[1];
            atlas_rgba[dst_idx + 2] = pixel[2];
            atlas_rgba[dst_idx + 3] = 255u8;
        }

        atlas_positions.insert(name.clone(), (0, cursor_y, w, h));
        cursor_y += h;
    }

    unsafe {
        *ATLAS_BYTES = atlas_rgba;
        *ATLAS_SIZE = (atlas_w, atlas_h);
    }

    let mut texidx_to_placement: HashMap<i32, (u32, u32, u32, u32)> = HashMap::new();
    let fallback = atlas_positions
        .values()
        .next()
        .copied()
        .unwrap_or((0u32, 0u32, 1u32, 1u32));

    for (i, opt_tex) in bsp.textures.iter().enumerate() {
        if let Some(tex) = opt_tex {
            let name = tex.header.name.to_string();
            if let Some(&(px, py, w, h)) = atlas_positions.get(&name) {
                texidx_to_placement.insert(i as i32, (px, py, w, h));
            } else {
                texidx_to_placement.insert(i as i32, fallback);
            }
        }
    }

    let mut buffer = GpuCommandBuffer::new();

    unsafe {
        let atlas_ref: &Vec<u8> = &ATLAS_BYTES;
        let (w, h) = *ATLAS_SIZE;
        buffer.insert(GpuCommand::RegisterTexture {
            rgba: atlas_ref.as_slice(),
            w,
            h,
        });
    }

    #[inline(always)]
    fn fract_positive(x: f32) -> f32 {
        let f = x - x.floor();
        if f < 0.0 { f + 1.0 } else { f }
    }

    fn triangulate_fan<I>(vertices: I) -> Vec<[qbsp::glam::Vec3; 3]>
    where
        I: IntoIterator<Item = qbsp::glam::Vec3>,
    {
        let verts: Vec<_> = vertices.into_iter().collect();
        let mut tris = Vec::new();
        if verts.len() < 3 {
            return tris;
        }
        let v0 = verts[0];
        for i in 1..(verts.len() - 1) {
            tris.push([v0, verts[i], verts[i + 1]]);
        }
        tris
    }

    buffer.insert(GpuCommand::PushRecord(PrimitiveType::Triangles));

    for face in &bsp.faces {
        let verts: Vec<qbsp::glam::Vec3> = face.vertices(&bsp).collect();
        if verts.len() < 3 {
            continue;
        }

        let tex_info = &bsp.tex_info[face.texture_info_idx.0 as usize];
        let tex_idx = tex_info.texture_idx.0.unwrap_or(0) as i32;

        let (px, py, tw, th) = texidx_to_placement
            .get(&tex_idx)
            .copied()
            .unwrap_or(fallback);
        let tex_w = tw.max(1);
        let tex_h = th.max(1);

        let proj = tex_info.projection;

        for tri in triangulate_fan(verts) {
            for v in &tri {
                let p = *v;
                let uv_world = proj.project(p);
                let u_world = uv_world.x;
                let v_world = uv_world.y;

                let u_tiled = fract_positive(u_world / (tex_w as f32));
                let v_tiled = fract_positive(v_world / (tex_h as f32));

                let atlas_u = (px as f32 + u_tiled * (tw as f32)) / (atlas_w as f32);
                let atlas_v = (py as f32 + v_tiled * (th as f32)) / (atlas_h as f32);

                buffer.insert(GpuCommand::EmitVertex(Vertex::new(
                    v.x, v.y, v.z, atlas_u, atlas_v,
                )));
            }
        }
    }

    buffer.insert(GpuCommand::PopRecord);
    buffer.upload();

    unsafe {
        GLOBAL_BUFFER.insert(GpuCommand::Push);
        GLOBAL_BUFFER.insert(GpuCommand::BindTexture(0));
        GLOBAL_BUFFER.insert(GpuCommand::DrawRecorded(0));
        GLOBAL_BUFFER.insert(GpuCommand::Pop);
    }
}

#[allow(static_mut_refs)]
#[gooseboy::update]
fn update(_nano_time: i64) {
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

    unsafe {
        GLOBAL_BUFFER.upload();
    }
}
