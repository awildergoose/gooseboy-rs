use crate::framebuffer::FRAMEBUFFER;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Color = Color {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const ORANGE: Color = Color {
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    };
    pub const PURPLE: Color = Color {
        r: 128,
        g: 0,
        b: 128,
        a: 255,
    };
    pub const PINK: Color = Color {
        r: 255,
        g: 192,
        b: 203,
        a: 255,
    };
    pub const BROWN: Color = Color {
        r: 165,
        g: 42,
        b: 42,
        a: 255,
    };
    pub const GRAY: Color = Color {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
    pub const LIGHT_GRAY: Color = Color {
        r: 211,
        g: 211,
        b: 211,
        a: 255,
    };
    pub const DARK_GRAY: Color = Color {
        r: 64,
        g: 64,
        b: 64,
        a: 255,
    };

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
