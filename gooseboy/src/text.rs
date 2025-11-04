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
    max_width: Option<usize>,
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
                if let Some(mw) = max_width
                    && cx + 8 > x + mw
                {
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
    draw_text_wrapped_ex(x, y, text.as_ref(), color, None);
}

pub fn draw_text_wrapped<S: AsRef<str>>(x: usize, y: usize, text: S, color: Color) {
    draw_text_wrapped_ex(x, y, text.as_ref(), color, Some(get_framebuffer_width()))
}

pub fn get_text_width<S: AsRef<str>>(text: S) -> usize {
    let text = text.as_ref();
    let mut max_width = 0;
    let mut current_width = 0;

    for ch in text.bytes() {
        match ch {
            b'\n' => {
                max_width = max_width.max(current_width);
                current_width = 0;
            }
            _ => {
                current_width += 8;
            }
        }
    }

    max_width.max(current_width)
}

pub fn get_text_height<S: AsRef<str>>(text: S) -> usize {
    let text = text.as_ref();
    let mut lines = 1;

    for ch in text.bytes() {
        if ch == b'\n' {
            lines += 1;
        }
    }

    lines * 8
}
