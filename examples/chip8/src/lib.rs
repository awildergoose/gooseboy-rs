#![no_main]

mod cpu;
mod misc;

use crate::cpu::{Cpu, WIDTH};
use gooseboy::color::Color;
use gooseboy::framebuffer::{
    clear_framebuffer, get_framebuffer_height, get_framebuffer_width, init_fb, set_pixel,
};
use gooseboy::input::is_key_down;
use gooseboy::keys::{
    KEY_1, KEY_2, KEY_3, KEY_4, KEY_A, KEY_C, KEY_D, KEY_E, KEY_F, KEY_Q, KEY_R, KEY_S, KEY_V,
    KEY_W, KEY_X, KEY_Z,
};

static mut CHIP: Option<Cpu> = None;

#[gooseboy::main]
fn main() {
    init_fb();

    let mut chip = Cpu::default();
    let rom_bytes: &[u8] = include_bytes!("../roms/Breakout.ch8");
    chip.load_rom(rom_bytes);

    unsafe {
        CHIP = Some(chip);
    }
}

static mut LAST_NANO: i64 = 0;

#[gooseboy::update]
#[allow(static_mut_refs)]
fn update(nano_time: i64) {
    let mut dt = (nano_time - unsafe { LAST_NANO }) as f64 / 1_000_000_000.0;
    if dt <= 0.0 || dt > 0.5 {
        dt = 1.0 / 60.0;
    }
    unsafe {
        LAST_NANO = nano_time;
    }

    unsafe {
        let chip = CHIP.as_mut().unwrap();

        let cycles_per_frame = (500.0f64 / 60.0).ceil() as usize;
        for _ in 0..cycles_per_frame {
            chip.step();
        }

        let mut timer_acc = (dt * 60.0) as i32;
        while timer_acc >= 1 {
            if chip.delay_timer > 0 {
                chip.delay_timer -= 1;
            }
            if chip.sound_timer > 0 {
                chip.sound_timer -= 1;
            }
            timer_acc -= 1;
        }

        let c = &mut chip.keys;
        c[0x0] = is_key_down(KEY_X);
        c[0x1] = is_key_down(KEY_1);
        c[0x2] = is_key_down(KEY_2);
        c[0x3] = is_key_down(KEY_3);
        c[0x4] = is_key_down(KEY_Q);
        c[0x5] = is_key_down(KEY_W);
        c[0x6] = is_key_down(KEY_E);
        c[0x7] = is_key_down(KEY_A);
        c[0x8] = is_key_down(KEY_S);
        c[0x9] = is_key_down(KEY_D);
        c[0xA] = is_key_down(KEY_Z);
        c[0xB] = is_key_down(KEY_C);
        c[0xC] = is_key_down(KEY_4);
        c[0xD] = is_key_down(KEY_R);
        c[0xE] = is_key_down(KEY_F);
        c[0xF] = is_key_down(KEY_V);

        clear_framebuffer(Color::BLACK);
        let fb_w = get_framebuffer_width();
        let fb_h = get_framebuffer_height();

        let chip_w = 64usize;
        let chip_h = 32usize;
        let scale_x = fb_w / chip_w;
        let scale_y = fb_h / chip_h;
        let scale = scale_x.min(scale_y).max(1);

        let x_off = (fb_w - chip_w * scale) / 2;
        let y_off = (fb_h - chip_h * scale) / 2;

        for y in 0..chip_h {
            for x in 0..chip_w {
                let on = chip.display[y * WIDTH + x] != 0;
                let col = if on { Color::WHITE } else { Color::BLACK };
                for sy in 0..scale {
                    for sx in 0..scale {
                        set_pixel(x_off + x * scale + sx, y_off + y * scale + sy, col);
                    }
                }
            }
        }
    }
}
