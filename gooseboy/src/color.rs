use crate::framebuffer::FRAMEBUFFER;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const ORANGE: Self = Self {
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    };
    pub const PURPLE: Self = Self {
        r: 128,
        g: 0,
        b: 128,
        a: 255,
    };
    pub const PINK: Self = Self {
        r: 255,
        g: 192,
        b: 203,
        a: 255,
    };
    pub const BROWN: Self = Self {
        r: 165,
        g: 42,
        b: 42,
        a: 255,
    };
    pub const GRAY: Self = Self {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
    pub const LIGHT_GRAY: Self = Self {
        r: 211,
        g: 211,
        b: 211,
        a: 255,
    };
    pub const DARK_GRAY: Self = Self {
        r: 64,
        g: 64,
        b: 64,
        a: 255,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn new_opaque(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
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
