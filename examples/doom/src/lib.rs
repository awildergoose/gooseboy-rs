#![no_main]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_wrap)]

use gooseboy::color::Color;
use gooseboy::framebuffer::{
    clear_framebuffer, get_framebuffer_height, get_framebuffer_width, init_fb, set_pixel,
};
use gooseboy::input::{
    get_mouse_accumulated_dx, grab_mouse, is_key_down, is_key_just_pressed, is_mouse_grabbed,
};
use gooseboy::keys::{KEY_A, KEY_D, KEY_LEFT, KEY_Q, KEY_RIGHT, KEY_S, KEY_W};
use std::f64::consts::PI;

static mut PLAYER_X: f64 = 1.5;
static mut PLAYER_Y: f64 = 1.5;
static mut PLAYER_ANGLE: f64 = 0.0;
static mut PLAYER_FOV: f64 = PI / 3.0;
static mut MOVE_SPEED: f64 = 3.0;
static mut ROT_SPEED: f64 = 2.0;
static mut TEXTURES: Option<[[[Color; 64]; 64]; 9]> = None;
static mut LAST_NANO: i64 = 0;

static MAP: [[u8; 16]; 16] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 2, 2, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 1],
    [1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 4, 4, 4, 0, 0, 0, 0, 0, 5, 0, 1],
    [1, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 5, 5, 0, 1],
    [1, 0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 5, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 6, 0, 0, 1],
    [1, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 7, 7, 0, 0, 0, 0, 8, 8, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

fn generate_texture(id: u8) -> [[Color; 64]; 64] {
    let mut texture = [[Color::BLACK; 64]; 64];

    for (y, row) in texture.iter_mut().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let color = match id {
                1 => Color::new_opaque(200, 100, 100),
                2 => Color::new_opaque(100, 200, 100),
                3 => Color::new_opaque(100, 100, 200),
                4 => Color::new_opaque(200, 200, 100),
                5 => Color::new_opaque(200, 100, 200),
                6 => Color::new_opaque(100, 200, 200),
                7 => Color::new_opaque(150, 150, 150),
                8 => Color::new_opaque(180, 160, 140),
                _ => Color::new_opaque(255, 255, 255),
            };

            let pattern = ((x / 8) % 2) == ((y / 8) % 2);
            let final_color = if pattern {
                Color::new(
                    (f32::from(color.r) * 0.8) as u8,
                    (f32::from(color.g) * 0.8) as u8,
                    (f32::from(color.b) * 0.8) as u8,
                    255,
                )
            } else {
                color
            };

            *pixel = final_color;
        }
    }

    texture
}

fn generate_textures() -> [[[Color; 64]; 64]; 9] {
    // TODO
    #[allow(clippy::large_stack_arrays)]
    let mut textures = [[[Color::BLACK; 64]; 64]; 9];
    for (i, texture) in textures.iter_mut().enumerate() {
        *texture = generate_texture(i as u8);
    }
    textures
}

fn cast_ray(angle: f64) -> (f64, bool, f64, u8) {
    unsafe {
        let ray_dir_x = angle.cos();
        let ray_dir_y = angle.sin();

        let map_x = PLAYER_X as i32;
        let map_y = PLAYER_Y as i32;

        let delta_dist_x = if ray_dir_x == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir_x).abs()
        };
        let delta_dist_y = if ray_dir_y == 0.0 {
            1e30
        } else {
            (1.0 / ray_dir_y).abs()
        };

        let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
            (-1, (PLAYER_X - f64::from(map_x)) * delta_dist_x)
        } else {
            (1, (f64::from(map_x) + 1.0 - PLAYER_X) * delta_dist_x)
        };

        let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
            (-1, (PLAYER_Y - f64::from(map_y)) * delta_dist_y)
        } else {
            (1, (f64::from(map_y) + 1.0 - PLAYER_Y) * delta_dist_y)
        };

        let (mut current_x, mut current_y) = (map_x, map_y);
        let mut hit = 0;
        let mut side = false;

        while hit == 0 {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                current_x += step_x;
                side = false;
            } else {
                side_dist_y += delta_dist_y;
                current_y += step_y;
                side = true;
            }

            if !(0..16).contains(&current_x) || !(0..16).contains(&current_y) {
                hit = 1;
            } else if MAP[current_y as usize][current_x as usize] > 0 {
                hit = MAP[current_y as usize][current_x as usize];
            }
        }

        let perp_wall_dist = if side {
            (f64::from(current_y) - PLAYER_Y + f64::from(1 - step_y) / 2.0) / ray_dir_y
        } else {
            (f64::from(current_x) - PLAYER_X + f64::from(1 - step_x) / 2.0) / ray_dir_x
        };

        let mut wall_x = if side {
            PLAYER_X + perp_wall_dist * ray_dir_x
        } else {
            PLAYER_Y + perp_wall_dist * ray_dir_y
        };
        wall_x -= wall_x.floor();

        (perp_wall_dist, side, wall_x, hit)
    }
}

fn draw_view(textures: &[[[Color; 64]; 64]; 9]) {
    unsafe {
        let width = get_framebuffer_width();
        let height = get_framebuffer_height();

        for y in 0..height / 2 {
            let color = Color::new_opaque(100, 100, 150);
            for x in 0..width {
                set_pixel(x, y, color);
            }
        }

        for y in height / 2..height {
            let color = Color::new_opaque(80, 80, 80);
            for x in 0..width {
                set_pixel(x, y, color);
            }
        }

        for x in 0..width {
            let camera_x = 2.0 * x as f64 / width as f64 - 1.0;
            let ray_angle = (PLAYER_FOV / 2.0).mul_add(camera_x, PLAYER_ANGLE);

            let (wall_dist, side, wall_x, texture_id) = cast_ray(ray_angle);

            let line_height = (height as f64 / wall_dist) as i32;
            let draw_start = (-line_height / 2 + height as i32 / 2).max(0) as usize;
            let draw_end = (line_height / 2 + height as i32 / 2).min(height as i32 - 1) as usize;

            let tex = &textures[texture_id as usize];
            let tex_x = (wall_x * 64.0) as usize;

            for y in draw_start..draw_end {
                let d = y * 256 - height * 128 + line_height as usize * 128;
                let tex_y = ((d * 64) / line_height as usize) / 256;

                let mut color = tex[tex_y.min(63)][tex_x.min(63)];

                let shade =
                    if side { 0.7 } else { 1.0 } * (wall_dist / 16.0).min(1.0).mul_add(-0.3, 1.0);

                color = Color::new_opaque(
                    (f64::from(color.r) * shade) as u8,
                    (f64::from(color.g) * shade) as u8,
                    (f64::from(color.b) * shade) as u8,
                );

                set_pixel(x, y, color);
            }
        }
    }
}

fn handle_input(dt: f64) {
    unsafe {
        let move_speed = MOVE_SPEED * dt;
        let rot_speed = ROT_SPEED * dt;

        PLAYER_ANGLE += get_mouse_accumulated_dx() * 0.003;
        PLAYER_ANGLE %= 2.0 * PI;

        if is_key_down(KEY_LEFT) {
            PLAYER_ANGLE -= rot_speed;
        }
        if is_key_down(KEY_RIGHT) {
            PLAYER_ANGLE += rot_speed;
        }

        PLAYER_ANGLE %= 2.0 * PI;
        if PLAYER_ANGLE < 0.0 {
            PLAYER_ANGLE += 2.0 * PI;
        }

        let move_x = PLAYER_ANGLE.cos() * move_speed;
        let move_y = PLAYER_ANGLE.sin() * move_speed;

        let strafe_x = PLAYER_ANGLE.sin() * move_speed;
        let strafe_y = -PLAYER_ANGLE.cos() * move_speed;

        if is_key_down(KEY_W) {
            let new_x = PLAYER_X + move_x;
            let new_y = PLAYER_Y + move_y;
            if can_move_to(new_x, new_y) {
                PLAYER_X = new_x;
                PLAYER_Y = new_y;
            }
        }
        if is_key_down(KEY_S) {
            let new_x = PLAYER_X - move_x;
            let new_y = PLAYER_Y - move_y;
            if can_move_to(new_x, new_y) {
                PLAYER_X = new_x;
                PLAYER_Y = new_y;
            }
        }
        if is_key_down(KEY_D) {
            let new_x = PLAYER_X - strafe_x;
            let new_y = PLAYER_Y - strafe_y;
            if can_move_to(new_x, new_y) {
                PLAYER_X = new_x;
                PLAYER_Y = new_y;
            }
        }
        if is_key_down(KEY_A) {
            let new_x = PLAYER_X + strafe_x;
            let new_y = PLAYER_Y + strafe_y;
            if can_move_to(new_x, new_y) {
                PLAYER_X = new_x;
                PLAYER_Y = new_y;
            }
        }

        if is_key_just_pressed(KEY_Q) {
            let error_margin = 0.001;
            PLAYER_FOV = if (PLAYER_FOV - PI / 3.0).abs() < error_margin {
                PI / 2.0
            } else {
                PI / 3.0
            };
        }
    }
}

fn can_move_to(x: f64, y: f64) -> bool {
    let map_x = x as i32;
    let map_y = y as i32;

    if !(0..16).contains(&map_x) || !(0..16).contains(&map_y) {
        return false;
    }

    MAP[map_y as usize][map_x as usize] == 0
}

#[gooseboy::main]
fn main() {
    init_fb();

    unsafe {
        TEXTURES = Some(generate_textures());
        LAST_NANO = 0;
    }
}

#[gooseboy::update]
#[allow(static_mut_refs)]
fn update(nano_time: i64) {
    let mut dt = (nano_time - unsafe { LAST_NANO }) as f64 / 1_000_000_000.0;
    if dt <= 0.0 || dt > 0.5 {
        dt = 1.0 / 60.0;
    }
    unsafe { LAST_NANO = nano_time };

    if !is_mouse_grabbed() {
        grab_mouse();
    }

    handle_input(dt);
    clear_framebuffer(Color::BLACK);

    unsafe {
        if let Some(textures) = &TEXTURES {
            draw_view(textures);
        }
    }
}
