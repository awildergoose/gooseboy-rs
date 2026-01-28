use crate::{
    bindings::{gpu_read, submit_gpu_commands},
    mem::alloc_bytes,
    unsafe_casts,
};

pub const GB_GPU_STATUS: u32 = 0;
pub const GB_GPU_RECORD_ID: u32 = 4;
pub const GB_GPU_TEXTURE_ID: u32 = 8;
pub const GB_GPU_MATRIX_DEPTH: u32 = 12;
pub const GB_STATUS_OK: u32 = 0;
pub const GB_STATUS_BAD_TEXTURE_SIZE: u32 = 1;
pub const GB_STATUS_BAD_TEXTURE: u32 = 2;
pub const GB_STATUS_MATRIX_TOO_SMALL: u32 = 3;
pub const GB_STATUS_MATRIX_TOO_BIG: u32 = 4;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
}

impl Vertex {
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub const fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self { x, y, z, u, v }
    }

    #[must_use]
    pub fn as_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        bytes[..4].copy_from_slice(&self.x.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.z.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.u.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.v.to_le_bytes());
        bytes
    }
}

#[repr(u8)]
pub enum PrimitiveType {
    Triangles,
    Quads,
}

impl PrimitiveType {
    #[must_use]
    pub const fn repr(&self) -> u8 {
        match self {
            Self::Triangles => 0,
            Self::Quads => 1,
        }
    }
}

#[repr(u8)]
pub enum GpuCommand<'a> {
    Push,
    Pop,
    PushRecord(PrimitiveType),
    PopRecord,
    DrawRecorded(u32),
    EmitVertex(Vertex),
    BindTexture(u32),
    RegisterTexture { w: u32, h: u32, rgba: &'a [u8] },
    Translate { x: f32, y: f32, z: f32 },
    RotateAxis { x: f32, y: f32, z: f32, angle: f32 },
    RotateEuler { yaw: f32, pitch: f32, roll: f32 },
    Scale { x: f32, y: f32, z: f32 },
    LoadMatrix([f32; 16]),
    MulMatrix([f32; 16]),
    Identity,
}

impl GpuCommand<'_> {
    #[must_use]
    pub const fn repr(&self) -> u8 {
        match self {
            GpuCommand::Push => 0x00,
            GpuCommand::Pop => 0x01,
            GpuCommand::PushRecord(_) => 0x02,
            GpuCommand::PopRecord => 0x03,
            GpuCommand::DrawRecorded(_) => 0x04,
            GpuCommand::EmitVertex(_) => 0x05,
            GpuCommand::BindTexture(_) => 0x06,
            GpuCommand::RegisterTexture { .. } => 0x07,
            GpuCommand::Translate { .. } => 0x08,
            GpuCommand::RotateAxis { .. } => 0x09,
            GpuCommand::RotateEuler { .. } => 0x0A,
            GpuCommand::Scale { .. } => 0x0B,
            GpuCommand::LoadMatrix(_) => 0x0C,
            GpuCommand::MulMatrix(_) => 0x0D,
            GpuCommand::Identity => 0x0E,
        }
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        buf.push(self.repr());
        match self {
            GpuCommand::PushRecord(p) => buf.extend_from_slice(&p.repr().to_le_bytes()),
            GpuCommand::DrawRecorded(id) | GpuCommand::BindTexture(id) => {
                buf.extend_from_slice(&id.to_le_bytes());
            }
            GpuCommand::EmitVertex(v) => buf.extend_from_slice(&v.as_bytes()),
            GpuCommand::RegisterTexture { rgba, w, h } => {
                buf.extend_from_slice(&w.to_le_bytes());
                buf.extend_from_slice(&h.to_le_bytes());
                buf.extend_from_slice(rgba);
            }
            GpuCommand::Translate { x, y, z } | GpuCommand::Scale { x, y, z } => {
                buf.extend_from_slice(&x.to_le_bytes());
                buf.extend_from_slice(&y.to_le_bytes());
                buf.extend_from_slice(&z.to_le_bytes());
            }
            GpuCommand::RotateAxis { x, y, z, angle } => {
                buf.extend_from_slice(&x.to_le_bytes());
                buf.extend_from_slice(&y.to_le_bytes());
                buf.extend_from_slice(&z.to_le_bytes());
                buf.extend_from_slice(&angle.to_le_bytes());
            }
            GpuCommand::RotateEuler { yaw, pitch, roll } => {
                buf.extend_from_slice(&yaw.to_le_bytes());
                buf.extend_from_slice(&pitch.to_le_bytes());
                buf.extend_from_slice(&roll.to_le_bytes());
            }
            GpuCommand::LoadMatrix(mat) | GpuCommand::MulMatrix(mat) => {
                for f in mat {
                    buf.extend_from_slice(&f.to_le_bytes());
                }
            }
            _ => {}
        }
    }
}

pub struct GpuCommandBuffer {
    buffer: Vec<u8>,
}

impl GpuCommandBuffer {
    #[must_use]
    pub const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn insert(&mut self, cmd: &GpuCommand) -> &mut Self {
        cmd.serialize(&mut self.buffer);
        self
    }

    pub fn upload(&mut self) {
        unsafe {
            submit_gpu_commands(self.buffer.as_ptr(), unsafe_casts::arr_len(&self.buffer));
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for GpuCommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub fn gpu_read_value<T: Copy>(offset: u32) -> T {
    unsafe {
        let ptr = alloc_bytes(size_of::<T>());
        gpu_read(
            unsafe_casts::u32_as_i32(offset),
            ptr,
            unsafe_casts::usize_as_i32(size_of::<T>()),
        );
        *(ptr as *const T)
    }
}

/// Do note; this method is very slow compared to creating your own mesh
/// file format, it's only here for quick testing, Only supports [v, f, vf]
/// in the `obj_data` string
#[must_use]
#[allow(clippy::similar_names)]
pub fn load_obj(obj_data: &str, flip_v: bool) -> Vec<Vertex> {
    fn parse_index(s: &str, len: usize) -> Option<usize> {
        if s.is_empty() {
            return None;
        }

        match s.parse::<isize>() {
            Ok(i) if i > 0 => {
                let idx = i.cast_unsigned().saturating_sub(1);
                Some(idx)
            }
            Ok(i) if i < 0 => {
                let abs = (-i).cast_unsigned();

                if abs == 0 || abs > len {
                    None
                } else {
                    Some(len - abs)
                }
            }
            _ => None,
        }
    }

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut texcoords: Vec<[f32; 2]> = Vec::new();

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
            for tok in &tokens {
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
                for &(v_idx_opt, vt_idx_opt) in &tri {
                    let pos = match v_idx_opt {
                        Some(idx) if idx < positions.len() => positions[idx],
                        _ => [0.0, 0.0, 0.0],
                    };

                    let (u, mut v) = vt_idx_opt.map_or_else(
                        || {
                            if positions.len() == texcoords.len() {
                                v_idx_opt.map_or((0.0f32, 0.0f32), |vidx| {
                                    if vidx < texcoords.len() {
                                        texcoords[vidx].into()
                                    } else {
                                        (0.0f32, 0.0f32)
                                    }
                                })
                            } else {
                                (0.0f32, 0.0f32)
                            }
                        },
                        |tidx| {
                            if tidx < texcoords.len() {
                                texcoords[tidx].into()
                            } else {
                                (0.0f32, 0.0f32)
                            }
                        },
                    );

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
