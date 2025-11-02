use crate::{color::Color, font::FONT, framebuffer::set_pixel};

pub fn draw_char(x: usize, y: usize, c: u8, color: Color) {
    for row in 0..8 {
        let bits = FONT[c as usize][row];
        for col in 0..8 {
            if bits & (1 << (7 - col)) != 0 {
                set_pixel(x + col, y + row, color);
            }
        }
    }
}

pub fn draw_text(x: usize, mut y: usize, text: &str, color: Color) {
    let mut cx = x;

    for ch in text.bytes() {
        match ch {
            b'\n' => {
                y += 8;
                cx = x;
            }
            _ => {
                draw_char(cx, y, ch, color);
                cx += 8;
            }
        }
    }
}
