use crate::framebuffer::{Surface, get_framebuffer_surface_mut};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[inline(always)]
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let mut h = h - h.floor();
    h *= 6.0;

    let i = h.floor() as i32;
    let f = h - i as f32;

    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    let (r, g, b) = match i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    #[inline(always)]
    fn to_byte(x: f32) -> u8 {
        let y = x * 255.0 + 0.5;
        if y <= 0.0 {
            0
        } else if y >= 255.0 {
            255
        } else {
            y as u8
        }
    }

    (to_byte(r), to_byte(g), to_byte(b))
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
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
            self.blit_ex(get_framebuffer_surface_mut(), index);
        }
    }

    /// # Safety
    /// This directly sets pixels in the framebuffer without checking the index
    pub unsafe fn blit_ex(&self, surface: &mut Surface, index: usize) {
        surface.rgba[index] = self.r;
        surface.rgba[index + 1] = self.g;
        surface.rgba[index + 2] = self.b;
        surface.rgba[index + 3] = self.a;
    }
}
