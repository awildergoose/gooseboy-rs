#![no_main]

use flate2::read::ZlibDecoder;
use gooseboy::framebuffer::{FRAMEBUFFER, get_framebuffer_height, get_framebuffer_width, init_fb};
use std::io::{Cursor, Read};

const VIDEO: &[u8] = include_bytes!("../video.dat");

static mut VIDEO_WIDTH: usize = 0;
static mut VIDEO_HEIGHT: usize = 0;
static mut VIDEO_FRAME: usize = 0;
static mut VIDEO_ENDED: bool = false;

static mut VIDEO_START_NANO: i64 = 0;
static mut VIDEO_STREAM: Option<ZlibDecoder<Cursor<&[u8]>>> = None;

const VIDEO_FPS: f64 = 30.0;

#[gooseboy::main]
fn main() {
    init_fb();

    unsafe {
        init_video_stream();
        VIDEO_START_NANO = 0;
    }
}

unsafe fn init_video_stream() {
    let compressed_data = &VIDEO[4..];

    let cursor = Cursor::new(compressed_data);
    let mut decoder = ZlibDecoder::new(cursor);

    let mut wh_buf = [0u8; 8];
    decoder.read_exact(&mut wh_buf).unwrap();

    unsafe {
        VIDEO_WIDTH = u32::from_le_bytes(wh_buf[0..4].try_into().unwrap()) as usize;
        VIDEO_HEIGHT = u32::from_le_bytes(wh_buf[4..8].try_into().unwrap()) as usize;
        VIDEO_STREAM = Some(decoder);
    };
}

#[gooseboy::update]
fn update(nano_time: i64) {
    unsafe {
        if VIDEO_ENDED {
            return;
        }

        if VIDEO_START_NANO == 0 {
            VIDEO_START_NANO = nano_time;
        }

        let elapsed_sec = (nano_time - VIDEO_START_NANO) as f64 / 1_000_000_000.0;
        let target_frame = (elapsed_sec * VIDEO_FPS).floor() as usize;

        while VIDEO_FRAME <= target_frame {
            if !decode_next_frame() {
                VIDEO_ENDED = true;
                break;
            }

            VIDEO_FRAME += 1;
        }
    }
}

#[allow(static_mut_refs)]
unsafe fn decode_next_frame() -> bool {
    let vw = unsafe { VIDEO_WIDTH };
    let vh = unsafe { VIDEO_HEIGHT };
    let bytes_per_row = vw.div_ceil(8);
    let frame_size = bytes_per_row * vh;

    let decoder = match unsafe { VIDEO_STREAM.as_mut() } {
        Some(d) => d,
        None => return false,
    };

    let mut frame_data = vec![0u8; frame_size];
    if decoder.read_exact(&mut frame_data).is_err() {
        return false;
    }

    let fb_w = get_framebuffer_width();
    let fb_h = get_framebuffer_height();

    let (render_w, render_h) = {
        let sx = fb_w * vh;
        let sy = fb_h * vw;

        if sx <= sy {
            let rw = fb_w;
            let rh = (vh * rw) / vw;
            (rw, rh)
        } else {
            let rh = fb_h;
            let rw = (vw * rh) / vh;
            (rw, rh)
        }
    };

    let offset_x = (fb_w.saturating_sub(render_w)) / 2;
    let offset_y = (fb_h.saturating_sub(render_h)) / 2;

    for fb_y in 0..fb_h {
        if fb_y < offset_y || fb_y >= offset_y + render_h {
            continue;
        }

        let local_y = fb_y - offset_y;
        let src_y = (local_y * vh) / render_h;
        let row_base = src_y * bytes_per_row;

        for fb_x in 0..fb_w {
            if fb_x < offset_x || fb_x >= offset_x + render_w {
                continue;
            }

            let local_x = fb_x - offset_x;
            let src_x = (local_x * vw) / render_w;
            let byte_index = row_base + (src_x / 8);
            let bit_index = 7 - (src_x & 7);

            if byte_index >= frame_data.len() {
                continue;
            }

            let byte = frame_data[byte_index];
            let bit = (byte >> bit_index) & 1;
            let pixel_index = fb_y
                .checked_mul(fb_w)
                .and_then(|r| r.checked_add(fb_x))
                .unwrap_or(0);
            let fb_idx = pixel_index.checked_mul(4).unwrap_or(0);

            if fb_idx + 3 >= unsafe { FRAMEBUFFER.len() } {
                continue;
            }

            unsafe {
                if bit == 1 {
                    FRAMEBUFFER[fb_idx] = 0xFF;
                    FRAMEBUFFER[fb_idx + 1] = 0xFF;
                    FRAMEBUFFER[fb_idx + 2] = 0xFF;
                    FRAMEBUFFER[fb_idx + 3] = 0xFF;
                } else {
                    FRAMEBUFFER[fb_idx] = 0x00;
                    FRAMEBUFFER[fb_idx + 1] = 0x00;
                    FRAMEBUFFER[fb_idx + 2] = 0x00;
                    FRAMEBUFFER[fb_idx + 3] = 0xFF;
                }
            }
        }
    }

    true
}
