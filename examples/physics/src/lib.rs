#![no_main]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]

use std::collections::HashMap;

use gooseboy::color::{Color, hsv_to_rgb};
use gooseboy::framebuffer::{
    Surface, clear_framebuffer, draw_rect, get_framebuffer_height, get_framebuffer_width, init_fb,
};
use gooseboy::input::{
    get_mouse_x, get_mouse_y, is_mouse_button_down, is_mouse_button_just_pressed,
};
use gooseboy::system::get_time_nanos;
use gooseboy::text::draw_text;

use rapier2d::prelude::*;

static mut PHYSICS: Option<PhysicsState> = None;
static mut LAST_NANO: i64 = 0;

struct PhysicsState {
    pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    islands: IslandManager,
    broad_phase: BroadPhaseBvh,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    picked: Option<RigidBodyHandle>,
    colors: HashMap<rapier2d::data::Index, Color>,
    next_color_seed: u32,
}

impl PhysicsState {
    fn new() -> Self {
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();

        let screen_w = get_framebuffer_width() as f32;
        let ground_x_m = (screen_w * 0.5) * PX_TO_M;

        let ground_body = RigidBodyBuilder::fixed()
            .translation(vector![ground_x_m, 0.0])
            .build();
        let ground_handle = bodies.insert(ground_body);

        let ground_collider = ColliderBuilder::cuboid(50.0, 0.5).build();
        colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, -9.81],
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: BroadPhaseBvh::new(),
            narrow_phase: NarrowPhase::new(),
            bodies,
            colliders,
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            picked: None,
            colors: HashMap::new(),
            next_color_seed: get_time_nanos() as u32,
        }
    }

    pub fn cleanup_offscreen(&mut self) {
        let mut to_remove = vec![];

        for (handle, body) in self.bodies.iter() {
            let pos = body.position().translation.vector;

            if pos.y < -50.0 {
                to_remove.push(handle);
            }
        }

        for handle in to_remove {
            self.bodies.remove(
                handle,
                &mut self.islands,
                &mut self.colliders,
                &mut self.impulse_joints,
                &mut self.multibody_joints,
                true,
            );
        }
    }

    fn gen_unique_color(&mut self) -> Color {
        self.next_color_seed = self.next_color_seed.wrapping_add(1);

        let idx = (self.next_color_seed % 1000) as f32;
        let golden_ratio_conjugate = 0.618_034_f32;
        let hue = (idx * golden_ratio_conjugate) % 1.0;
        let (r, g, b) = hsv_to_rgb(hue, 0.7, 0.95);
        Color::new_opaque(r, g, b)
    }
}

const PX_PER_M: f32 = 16.0;
const PX_TO_M: f32 = 1.0 / PX_PER_M;
const M_TO_PX: f32 = PX_PER_M;

fn screen_to_world(x_px: i32, y_px: i32) -> (f32, f32) {
    let h = get_framebuffer_height() as i32;
    let xf = x_px as f32 * PX_TO_M;
    let yf = (h - y_px) as f32 * PX_TO_M;
    (xf, yf)
}

fn world_to_screen(x_m: f32, y_m: f32) -> (i32, i32) {
    let h = get_framebuffer_height() as i32;
    let xp = (x_m * M_TO_PX).round() as i32;
    let yp = h - (y_m * M_TO_PX).round() as i32;
    (xp, yp)
}

#[gooseboy::main]
fn main() {
    init_fb();

    unsafe {
        PHYSICS = Some(PhysicsState::new());
    }
}

// TODO stop doing this
#[allow(static_mut_refs)]
#[allow(clippy::too_many_lines)]
#[gooseboy::update]
fn update(nano_time: i64) {
    let mut dt = (nano_time - unsafe { LAST_NANO }) as f64 / 1_000_000_000.0;
    if dt <= 0.0 || dt > 0.5 {
        dt = 1.0 / 60.0;
    }
    unsafe { LAST_NANO = nano_time };

    clear_framebuffer(Color::new_opaque(8, 12, 16));

    let state = unsafe { PHYSICS.as_mut().expect("physics not initialized") };
    state.integration_parameters.dt = dt as f32;

    if is_mouse_button_down(0) {
        let mx = get_mouse_x();
        let my = get_mouse_y();
        let (wx, wy) = screen_to_world(mx, my);
        let handle = spawn_box(&mut state.bodies, &mut state.colliders, wx, wy, 0.6, 0.6);
        let color = state.gen_unique_color();
        state.colors.insert(handle.0, color);
    }

    if is_mouse_button_just_pressed(1) {
        let mx = get_mouse_x();
        let my = get_mouse_y();
        let (wx, wy) = screen_to_world(mx, my);
        let mut best: Option<RigidBodyHandle> = None;
        let mut best_d2 = f32::INFINITY;

        for (h, body) in state.bodies.iter() {
            if body.is_fixed() {
                continue;
            }

            let pos = body.position().translation.vector;
            let dx = pos.x - wx;
            let dy = pos.y - wy;
            let d2 = dx.mul_add(dx, dy * dy);

            if d2 < best_d2 && d2 < (1.0f32).powi(2) {
                best_d2 = d2;
                best = Some(h);
            }
        }

        state.picked = best;
    }

    if is_mouse_button_down(1) {
        if let Some(handle) = state.picked {
            if let Some(body) = state.bodies.get_mut(handle) {
                let mx = get_mouse_x();
                let my = get_mouse_y();
                let (wx, wy) = screen_to_world(mx, my);
                let pos = body.position().translation.vector;
                let delta = vector![wx - pos.x, wy - pos.y];
                let follow_speed = 20.0;
                let target_vel = delta * follow_speed;
                body.set_linvel(target_vel, true);
                body.set_angvel(0.0, true);
            } else {
                state.picked = None;
            }
        }
    } else {
        state.picked = None;
    }

    if is_mouse_button_just_pressed(2) {
        let mx = get_mouse_x();
        let my = get_mouse_y();
        let (wx, wy) = screen_to_world(mx, my);

        let explosion_radius_m = 2.5;
        let explosion_strength = 60.0;

        apply_explosion(state, wx, wy, explosion_radius_m, explosion_strength);
    }

    state.pipeline.step(
        &state.gravity,
        &state.integration_parameters,
        &mut state.islands,
        &mut state.broad_phase,
        &mut state.narrow_phase,
        &mut state.bodies,
        &mut state.colliders,
        &mut state.impulse_joints,
        &mut state.multibody_joints,
        &mut state.ccd_solver,
        &(),
        &(),
    );

    state.cleanup_offscreen();

    let surface_guard = gooseboy::framebuffer::get_framebuffer_surface_mut();
    let surface: &mut Surface = surface_guard;

    let colors_ref = &state.colors;

    for (body_handle, body) in state.bodies.iter() {
        let pos = body.position().translation.vector;
        let (sx, sy) = world_to_screen(pos.x, pos.y);

        let mut drawn = false;

        for (_col_handle, col) in state.colliders.iter() {
            if let Some(parent) = col.parent()
                && parent == body_handle
            {
                if let Some(shape) = col.shape().as_cuboid() {
                    let hx = shape.half_extents.x;
                    let hy = shape.half_extents.y;
                    let w_px = (hx * M_TO_PX * 2.0).round() as usize;
                    let h_px = (hy * M_TO_PX * 2.0).round() as usize;
                    let x_px = sx - (w_px as i32 / 2);
                    let y_px = sy - (h_px as i32 / 2);
                    let render_col = if body.is_fixed() {
                        Color::WHITE
                    } else {
                        *colors_ref.get(&body_handle.0).unwrap_or(&Color::ORANGE)
                    };

                    draw_rect(surface, x_px, y_px, w_px, h_px, render_col, false);
                    drawn = true;
                }

                break;
            }
        }

        if !drawn {
            draw_rect(surface, sx - 6, sy - 6, 12, 12, Color::GREEN, false);
        }
    }

    let mx = get_mouse_x();
    let my = get_mouse_y();
    draw_text(
        mx as usize,
        my as usize,
        format!(
            "left-click: spawn\nright-click: drag\nmiddle-click: explode\nbodies: {}",
            state.bodies.len()
        ),
        Color::RED,
    );
}

fn spawn_box(
    bodies: &mut RigidBodySet,
    colliders: &mut ColliderSet,
    x: f32,
    y: f32,
    w_m: f32,
    h_m: f32,
) -> RigidBodyHandle {
    let rb = RigidBodyBuilder::dynamic()
        .translation(vector![x, y])
        .build();
    let handle = bodies.insert(rb);
    let col = ColliderBuilder::cuboid(w_m * 0.5, h_m * 0.5)
        .restitution(0.0)
        .friction(0.7)
        .build();

    colliders.insert_with_parent(col, handle, bodies);
    handle
}

fn apply_explosion(state: &mut PhysicsState, x: f32, y: f32, radius: f32, strength: f32) {
    let mut targets: Vec<RigidBodyHandle> = Vec::new();

    for (h, body) in state.bodies.iter() {
        if body.is_fixed() {
            continue;
        }

        let pos = body.position().translation.vector;
        let dx = pos.x - x;
        let dy = pos.y - y;
        let d2 = dx.mul_add(dx, dy * dy);

        if d2 <= radius * radius {
            targets.push(h);
        }
    }

    for h in targets {
        if let Some(body) = state.bodies.get_mut(h) {
            let pos = body.position().translation.vector;
            let mut dx = pos.x - x;
            let mut dy = pos.y - y;
            let dist = dx.hypot(dy).max(0.0001);

            dx /= dist;
            dy /= dist;

            let attenuation = 1.0 - (dist / radius);

            if attenuation <= 0.0 {
                continue;
            }

            let impulse_mag = attenuation * strength;
            let impulse = vector![dx * impulse_mag, dy * impulse_mag];
            body.apply_impulse(impulse, true);

            let seed = (h.0.into_raw_parts().0)
                .wrapping_mul(1_664_525)
                .wrapping_add(1_013_904_223);
            let sign = if (seed & 1) == 0 { 1.0 } else { -1.0 };
            let ang_impulse = sign * (impulse_mag * 0.02);
            body.apply_torque_impulse(ang_impulse, true);
        }
    }
}
