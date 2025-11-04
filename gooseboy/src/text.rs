use crate::{
    color::Color,
    font::FONT,
    framebuffer::{get_framebuffer_width, set_pixel},
};

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

pub fn draw_text_wrapped_ex<S: AsRef<str>>(
    x: usize,
    mut y: usize,
    text: S,
    color: Color,
    max_width: usize,
) {
    let text = text.as_ref();
    let mut cx = x;

    for ch in text.bytes() {
        match ch {
            b'\n' => {
                y += 8;
                cx = x;
            }
            _ => {
                if cx + 8 > x + max_width {
                    cx = x;
                    y += 8;
                }

                draw_char(cx, y, ch, color);
                cx += 8;
            }
        }
    }
}

pub fn draw_text<S: AsRef<str>>(x: usize, y: usize, text: S, color: Color) {
    draw_text_wrapped_ex(x, y, text.as_ref(), color, usize::MAX);
}

pub fn draw_text_wrapped<S: AsRef<str>>(x: usize, y: usize, text: S, color: Color) {
    draw_text_wrapped_ex(x, y, text.as_ref(), color, get_framebuffer_width())
}
