use crate::framebuffer::FRAMEBUFFER;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn new_opaque(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    /// # Safety
    /// This directly sets pixels in the framebuffer without checking the index
    pub unsafe fn blit(&self, index: usize) {
        unsafe {
            FRAMEBUFFER[index] = self.r;
            FRAMEBUFFER[index + 1] = self.g;
            FRAMEBUFFER[index + 2] = self.b;
            FRAMEBUFFER[index + 3] = self.a;
        }
    }
}
