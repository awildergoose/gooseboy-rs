//! This is used to hold `GooseGPU` related functions and structs.
//!
//! Example:
//! ```rs
//! let mut buffer = GpuCommandBuffer::new();
//! buffer.insert(&GpuCommand::Push);
//! buffer.insert(&GpuCommand::RotateAxis {
//!     x: 0.0,
//!     y: 1.0,
//!     z: 0.0,
//!     angle: 30.0,
//! });
//! buffer.insert(&GpuCommand::DrawRecorded(0));
//! buffer.insert(&GpuCommand::Pop);
//! let _ = buffer.upload();
//! ```
use crate::{
    bindings::{self, gpu_read, submit_gpu_commands},
    error::GooseboyError,
    mem::alloc_bytes,
    sprite::Sprite,
    unsafe_casts,
};

/// `GooseGPU` virtual memory location of the last status.
pub const GB_GPU_STATUS: u32 = 0;
/// `GooseGPU` virtual memory location of the last record id.
pub const GB_GPU_RECORD_ID: u32 = 4;
/// `GooseGPU` virtual memory location of the last texture id.
pub const GB_GPU_TEXTURE_ID: u32 = 8;
/// `GooseGPU` virtual memory location of the last matrix depth.
pub const GB_GPU_MATRIX_DEPTH: u32 = 12;
/// `GooseGPU` status: OK
pub const GB_STATUS_OK: u32 = 0;
/// `GooseGPU` status for when the uploaded texture is too big.
pub const GB_STATUS_BAD_TEXTURE_SIZE: u32 = 1;
/// `GooseGPU` status for when the uploaded texture is malformed.
pub const GB_STATUS_BAD_TEXTURE: u32 = 2;
/// `GooseGPU` status for when the matrix is too small post-pop.
pub const GB_STATUS_MATRIX_TOO_SMALL: u32 = 3;
/// `GooseGPU` status for when the matrix is too big post-push.
pub const GB_STATUS_MATRIX_TOO_BIG: u32 = 4;
/// `GooseGPU` status for when failing to emit vertices because we never got a call to start recording.
pub const GB_STATUS_NOT_RECORDING: u32 = 5;

/// A vertex, with a position and UV.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    /// The X position.
    pub x: f32,
    /// The Y position.
    pub y: f32,
    /// The Z position.
    pub z: f32,
    /// The UV's U component.
    pub u: f32,
    /// The UV's V component.
    pub v: f32,
}

impl Vertex {
    /// Creates a new [`Vertex`].
    #[must_use]
    #[allow(clippy::many_single_char_names)]
    pub const fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self { x, y, z, u, v }
    }

    /// Returns the `Vertex` as a byte array.
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

/// The type of the primitive.
#[repr(u8)]
pub enum PrimitiveType {
    /// Triangles.
    Triangles,
    /// Quads.
    Quads,
}

impl PrimitiveType {
    /// Returns the representation of this [`PrimitiveType`].
    #[must_use]
    pub const fn repr(&self) -> u8 {
        match self {
            Self::Triangles => 0,
            Self::Quads => 1,
        }
    }
}

/// A `GooseGPU` command.
#[repr(u8)]
pub enum GpuCommand<'a> {
    /// Push onto the matrix.
    Push,
    /// Pop from the matrix.
    Pop,
    /// Push a label to start recording.
    PushRecord(PrimitiveType),
    /// Pop the latest recording label.
    PopRecord,
    /// Draw a recording with an id.
    DrawRecorded(u32),
    /// Emit a single vertex.
    EmitVertex(Vertex),
    /// Bind a texture with an id.
    BindTexture(u32),
    /// Registers a texture.
    RegisterTexture {
        /// The width of the texture.
        w: u32,
        /// The height of the texture.
        h: u32,
        /// The RGBA bytes of the texture.
        rgba: &'a [u8],
    },
    /// Translates the matrix by a position.
    Translate {
        /// The X position.
        x: f32,
        /// The Y position.
        y: f32,
        /// The Z position.
        z: f32,
    },
    /// Rotates the matrix by a rotation.
    RotateAxis {
        /// The X rotation.
        x: f32,
        /// The Y rotation.
        y: f32,
        /// The Z rotation.
        z: f32,
        /// The angle.
        angle: f32,
    },
    /// Rotates the matrix by Euler angles.
    RotateEuler {
        /// The yaw.
        yaw: f32,
        /// The pitch.
        pitch: f32,
        /// The roll.
        roll: f32,
    },
    /// Rescales the matrix.
    Scale {
        /// The X scale.
        x: f32,
        /// The Y scale.
        y: f32,
        /// The Z scale.
        z: f32,
    },
    /// Loads a matrix.
    LoadMatrix([f32; 16]),
    /// Multiplies a matrix.
    MulMatrix([f32; 16]),
    /// Resets the matrix.
    Identity,
    /// Emits an array of vertices.
    EmitVertices(Box<[Vertex]>),
}

impl GpuCommand<'_> {
    /// Returns the representation of this [`GpuCommand`].
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
            GpuCommand::EmitVertices { .. } => 0x0F,
        }
    }

    /// Serializes the GPU command onto a buffer.
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
            GpuCommand::EmitVertices(vertices) => {
                buf.extend_from_slice(&vertices.len().to_le_bytes());
                for v in vertices {
                    buf.extend_from_slice(&v.as_bytes());
                }
            }
            _ => {}
        }
    }
}

/// A GPU command buffer to hold all GPU commands to be sent to the `GooseGPU`.
pub struct GpuCommandBuffer {
    buffer: Vec<u8>,
}

impl GpuCommandBuffer {
    /// Creates a new [`GpuCommandBuffer`].
    #[must_use]
    pub const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Inserts a command onto the buffer.
    pub fn insert(&mut self, cmd: &GpuCommand) -> &mut Self {
        cmd.serialize(&mut self.buffer);
        self
    }

    /// Inserts a `GpuCommand` for registering a `Sprite`.
    pub fn insert_register_sprite(&mut self, sprite: &Sprite) -> &mut Self {
        (GpuCommand::RegisterTexture {
            w: unsafe { unsafe_casts::usize_as_u32(sprite.width) },
            h: unsafe { unsafe_casts::usize_as_u32(sprite.height) },
            rgba: &sprite.rgba,
        })
        .serialize(&mut self.buffer);
        self
    }

    /// Uploads the [`GpuCommandBuffer`] to the GPU.
    /// Requires [`Gpu`](crate::system::Permission::Gpu) permission
    ///
    /// # Errors
    ///
    /// This function will return an error if we aren't authorized to use the GPU
    pub fn upload(&mut self) -> Result<(), GooseboyError> {
        if unsafe { submit_gpu_commands(self.buffer.as_ptr(), unsafe_casts::arr_len(&self.buffer)) }
        {
            Ok(())
        } else {
            Err(GooseboyError::Unauthorized)
        }
    }

    /// Clears this [`GpuCommandBuffer`].
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for GpuCommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Reads a value from the `GooseGPU` virtual memory.
/// Requires [`Gpu`](crate::system::Permission::Gpu) permission
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

/// Defers until the queued GPU commands run.
pub fn defer_gpu() {
    unsafe {
        bindings::defer_gpu();
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
