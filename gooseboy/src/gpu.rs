use crate::bindings::submit_gpu_commands;

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
pub enum GpuCommand {
    Push,
    Pop,
    PushRecord,
    PopRecord,
    DrawRecorded(u32),
    EmitVertex(Vertex),
    BindTexture(u32),
    RegisterTexture { ptr: *const u8, w: u32, h: u32 },
}

impl GpuCommand {
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
            GpuCommand::RegisterTexture { ptr, w, h } => {
                let ptr_val = *ptr as usize;
                buf.extend_from_slice(&ptr_val.to_le_bytes());
                buf.extend_from_slice(&w.to_le_bytes());
                buf.extend_from_slice(&h.to_le_bytes());
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
