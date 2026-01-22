use crate::{
    bindings::{gpu_read, submit_gpu_commands},
    mem::alloc_bytes,
};

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
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self { x, y, z, u, v }
    }

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
pub enum GpuCommand<'a> {
    Push,
    Pop,
    PushRecord,
    PopRecord,
    DrawRecorded(u32),
    EmitVertex(Vertex),
    BindTexture(u32),
    RegisterTexture { w: u32, h: u32, rgba: &'a [u8] },
}

impl GpuCommand<'_> {
    pub fn repr(&self) -> u8 {
        match self {
            GpuCommand::Push => 0x00,
            GpuCommand::Pop => 0x01,
            GpuCommand::PushRecord => 0x02,
            GpuCommand::PopRecord => 0x03,
            GpuCommand::DrawRecorded(_) => 0x04,
            GpuCommand::EmitVertex(_) => 0x05,
            GpuCommand::BindTexture(_) => 0x06,
            GpuCommand::RegisterTexture { .. } => 0x07,
        }
    }

    pub fn serialize(&self, buf: &mut Vec<u8>) {
        buf.push(self.repr());
        match self {
            GpuCommand::DrawRecorded(id) => buf.extend_from_slice(&id.to_le_bytes()),
            GpuCommand::EmitVertex(v) => buf.extend_from_slice(&v.as_bytes()),
            GpuCommand::BindTexture(id) => buf.extend_from_slice(&id.to_le_bytes()),
            GpuCommand::RegisterTexture { rgba, w, h } => {
                buf.extend_from_slice(&w.to_le_bytes());
                buf.extend_from_slice(&h.to_le_bytes());
                buf.extend_from_slice(rgba);
            }
            _ => {}
        }
    }
}

pub struct GpuCommandBuffer {
    buffer: Vec<u8>,
}

impl GpuCommandBuffer {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn insert(&mut self, cmd: GpuCommand) -> &mut Self {
        cmd.serialize(&mut self.buffer);
        self
    }

    pub fn upload(&mut self) {
        unsafe { submit_gpu_commands(self.buffer.as_ptr(), self.buffer.len() as i32) };
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

pub fn gpu_read_value<T: Copy>(offset: i32) -> T {
    let ptr = alloc_bytes(size_of::<T>());
    unsafe {
        gpu_read(offset, ptr, size_of::<T>() as i32);
        *(ptr as *const T)
    }
}

/// Do note; this method is very slow compared to creating your own mesh
/// file format, it's only here for quick testing, Only supports [v, f, vf]
/// in the obj_data string
pub fn load_obj(obj_data: &str, flip_v: bool) -> Vec<Vertex> {
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
