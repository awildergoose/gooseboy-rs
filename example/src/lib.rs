#![no_main]

use gooseboy::color::Color;
use gooseboy::framebuffer::{
    clear_framebuffer, get_framebuffer_height, get_framebuffer_width, init_fb, set_pixel,
};
use gooseboy::input::get_mouse_x;
use gooseboy::input::is_key_down;
use gooseboy::keys::{KEY_A, KEY_D, KEY_E, KEY_Q, KEY_S, KEY_W};

static mut LAST_NANO: i64 = 0;
static mut PREV_MOUSE_X: i32 = 0;
static mut MOUSE_INIT: bool = false;
static mut SMOOTH_FPS: f64 = 60.0;

static mut PLAYER_X: f64 = 3.5;
static mut PLAYER_Y: f64 = 3.5;
static mut DIR_X: f64 = -1.0;
static mut DIR_Y: f64 = 0.0;
static mut PLANE_X: f64 = 0.0;
static mut PLANE_Y: f64 = 0.66;

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;
const MAP: [i32; MAP_WIDTH * MAP_HEIGHT] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 0, 1, 1, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 3, 0, 1,
    1, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 3, 0, 1, 1, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

fn map_at(x: i32, y: i32) -> i32 {
    if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
        return 1;
    }
    MAP[(y as usize) * MAP_WIDTH + (x as usize)]
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    init_fb();
}

#[unsafe(no_mangle)]
pub extern "C" fn update(nano_time: i64) {
    let dt = unsafe {
        if LAST_NANO == 0 {
            LAST_NANO = nano_time;
            0.016
        } else {
            let delta = (nano_time - LAST_NANO) as f64 / 1_000_000_000.0;
            LAST_NANO = nano_time;
            if delta <= 0.0 { 0.016 } else { delta }
        }
    };

    let mouse_x = get_mouse_x();
    let mut mouse_dx: i32 = 0;
    unsafe {
        if !MOUSE_INIT {
            PREV_MOUSE_X = mouse_x;
            MOUSE_INIT = true;
        } else {
            mouse_dx = mouse_x - PREV_MOUSE_X;
            PREV_MOUSE_X = mouse_x;
        }
    }

    let move_speed = 3.5;
    let rot_speed = 2.5;
    let mouse_sens = 0.0065_f64;

    if mouse_dx != 0 {
        let angle = -(mouse_dx as f64) * mouse_sens;
        rotate_player(angle);
    }

    if is_key_down(KEY_E) {
        rotate_player(-rot_speed * dt);
    }
    if is_key_down(KEY_Q) {
        rotate_player(rot_speed * dt);
    }

    let step = move_speed * dt;
    let mut new_x: f64;
    let mut new_y: f64;

    if is_key_down(KEY_W) {
        new_x = unsafe { PLAYER_X } + unsafe { DIR_X } * step;
        new_y = unsafe { PLAYER_Y } + unsafe { DIR_Y } * step;
        if map_at(new_x.floor() as i32, unsafe { PLAYER_Y }.floor() as i32) == 0 {
            unsafe {
                PLAYER_X = new_x;
            }
        }
        if map_at(unsafe { PLAYER_X }.floor() as i32, new_y.floor() as i32) == 0 {
            unsafe {
                PLAYER_Y = new_y;
            }
        }
    }
    if is_key_down(KEY_S) {
        new_x = unsafe { PLAYER_X } - unsafe { DIR_X } * step;
        new_y = unsafe { PLAYER_Y } - unsafe { DIR_Y } * step;
        if map_at(new_x.floor() as i32, unsafe { PLAYER_Y }.floor() as i32) == 0 {
            unsafe {
                PLAYER_X = new_x;
            }
        }
        if map_at(unsafe { PLAYER_X }.floor() as i32, new_y.floor() as i32) == 0 {
            unsafe {
                PLAYER_Y = new_y;
            }
        }
    }

    let perp_x = -unsafe { DIR_Y };
    let perp_y = unsafe { DIR_X };
    if is_key_down(KEY_A) {
        new_x = unsafe { PLAYER_X } + perp_x * step;
        new_y = unsafe { PLAYER_Y } + perp_y * step;
        if map_at(new_x.floor() as i32, unsafe { PLAYER_Y }.floor() as i32) == 0 {
            unsafe {
                PLAYER_X = new_x;
            }
        }
        if map_at(unsafe { PLAYER_X }.floor() as i32, new_y.floor() as i32) == 0 {
            unsafe {
                PLAYER_Y = new_y;
            }
        }
    }
    if is_key_down(KEY_D) {
        new_x = unsafe { PLAYER_X } - perp_x * step;
        new_y = unsafe { PLAYER_Y } - perp_y * step;
        if map_at(new_x.floor() as i32, unsafe { PLAYER_Y }.floor() as i32) == 0 {
            unsafe {
                PLAYER_X = new_x;
            }
        }
        if map_at(unsafe { PLAYER_X }.floor() as i32, new_y.floor() as i32) == 0 {
            unsafe {
                PLAYER_Y = new_y;
            }
        }
    }

    clear_framebuffer(Color::BLACK);

    let width = get_framebuffer_width() as i32;
    let height = get_framebuffer_height() as i32;

    let ceiling_color = Color {
        r: 80,
        g: 80,
        b: 120,
        a: 255,
    };
    let floor_color = Color {
        r: 45,
        g: 45,
        b: 45,
        a: 255,
    };

    for x in 0..width {
        let camera_x = 2.0 * x as f64 / width as f64 - 1.0;
        let ray_dir_x = unsafe { DIR_X } + unsafe { PLANE_X } * camera_x;
        let ray_dir_y = unsafe { DIR_Y } + unsafe { PLANE_Y } * camera_x;

        let mut map_x = unsafe { PLAYER_X.floor() as i32 };
        let mut map_y = unsafe { PLAYER_Y.floor() as i32 };

        let delta_dist_x = if ray_dir_x.abs() < 1e-9 {
            1e30
        } else {
            (1.0 / ray_dir_x).abs()
        };
        let delta_dist_y = if ray_dir_y.abs() < 1e-9 {
            1e30
        } else {
            (1.0 / ray_dir_y).abs()
        };

        let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
            (-1, (unsafe { PLAYER_X } - map_x as f64) * delta_dist_x)
        } else {
            (1, (map_x as f64 + 1.0 - unsafe { PLAYER_X }) * delta_dist_x)
        };
        let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
            (-1, (unsafe { PLAYER_Y } - map_y as f64) * delta_dist_y)
        } else {
            (1, (map_y as f64 + 1.0 - unsafe { PLAYER_Y }) * delta_dist_y)
        };

        let mut hit = 0;
        let mut side = 0;
        while hit == 0 {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            let val = map_at(map_x, map_y);
            if val > 0 {
                hit = val;
            }
        }

        let perp_wall_dist: f64 = if side == 0 {
            (map_x as f64 - unsafe { PLAYER_X } + (1 - step_x) as f64 / 2.0) / ray_dir_x
        } else {
            (map_y as f64 - unsafe { PLAYER_Y } + (1 - step_y) as f64 / 2.0) / ray_dir_y
        };

        let mut line_height = (height as f64 / perp_wall_dist) as i32;
        if line_height < 1 {
            line_height = 1;
        }
        let mut draw_start = -line_height / 2 + height / 2;
        if draw_start < 0 {
            draw_start = 0;
        }
        let mut draw_end = line_height / 2 + height / 2;
        if draw_end >= height {
            draw_end = height - 1;
        }

        let mut base_color = match hit {
            1 => (200u8, 30u8, 30u8),
            2 => (30u8, 200u8, 30u8),
            3 => (30u8, 30u8, 200u8),
            4 => (200u8, 200u8, 30u8),
            _ => (150u8, 150u8, 150u8),
        };

        if side == 1 {
            base_color.0 = ((base_color.0 as f32) * 0.6).round() as u8;
            base_color.1 = ((base_color.1 as f32) * 0.6).round() as u8;
            base_color.2 = ((base_color.2 as f32) * 0.6).round() as u8;
        }

        let fog: f32 = (1.0_f32 / (1.0_f32 + (perp_wall_dist as f32) * 0.08_f32)).clamp(0.0, 1.0);
        base_color.0 = ((base_color.0 as f32 * fog).round().clamp(0.0, 255.0)) as u8;
        base_color.1 = ((base_color.1 as f32 * fog).round().clamp(0.0, 255.0)) as u8;
        base_color.2 = ((base_color.2 as f32 * fog).round().clamp(0.0, 255.0)) as u8;

        for y in 0..height {
            if y < draw_start {
                set_pixel(x as usize, y as usize, ceiling_color);
            } else if y > draw_end {
                set_pixel(x as usize, y as usize, floor_color);
            } else {
                let c = Color {
                    r: base_color.0,
                    g: base_color.1,
                    b: base_color.2,
                    a: 255,
                };
                set_pixel(x as usize, y as usize, c);
            }
        }
    }

    gooseboy::text::draw_text(
        4,
        4,
        format!(
            "pos: {:.2},{:.2}  dt: {:.3}",
            unsafe { PLAYER_X },
            unsafe { PLAYER_Y },
            dt
        )
        .as_str(),
        Color::WHITE,
    );

    let instant_fps = if dt > 1e-9 {
        1.0 / dt
    } else {
        unsafe { SMOOTH_FPS }
    };
    let alpha = 0.12_f64;
    unsafe {
        SMOOTH_FPS = SMOOTH_FPS * (1.0 - alpha) + instant_fps * alpha;
    }
    let fps_display = unsafe { SMOOTH_FPS.round() as i32 };
    gooseboy::text::draw_text(
        4,
        16,
        format!("fps: {}", fps_display).as_str(),
        Color::WHITE,
    );
}

fn rotate_player(angle: f64) {
    let old_dir_x = unsafe { DIR_X };
    let cosv = angle.cos();
    let sinv = angle.sin();
    unsafe {
        DIR_X = DIR_X * cosv - DIR_Y * sinv;
        DIR_Y = old_dir_x * sinv + DIR_Y * cosv;

        let old_plane_x = PLANE_X;
        PLANE_X = PLANE_X * cosv - PLANE_Y * sinv;
        PLANE_Y = old_plane_x * sinv + PLANE_Y * cosv;
    }
}
