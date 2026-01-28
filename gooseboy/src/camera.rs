use crate::{Vec2, Vec3, bindings, mem::alloc_bytes};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CameraTransform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}

#[must_use]
pub fn get_camera_transform() -> CameraTransform {
    let ptr = alloc_bytes(size_of::<CameraTransform>());
    unsafe {
        bindings::get_camera_transform(ptr);
        *(ptr as *const CameraTransform)
    }
}

pub fn set_camera_transform(transform: CameraTransform) {
    unsafe {
        bindings::set_camera_transform(
            transform.x,
            transform.y,
            transform.z,
            transform.yaw,
            transform.pitch,
        );
    }
}

#[must_use]
pub fn get_camera_x() -> f32 {
    get_camera_transform().x
}

#[must_use]
pub fn get_camera_y() -> f32 {
    get_camera_transform().y
}

#[must_use]
pub fn get_camera_z() -> f32 {
    get_camera_transform().z
}

#[must_use]
pub fn get_camera_yaw() -> f32 {
    get_camera_transform().yaw
}

#[must_use]
pub fn get_camera_pitch() -> f32 {
    get_camera_transform().pitch
}

#[must_use]
pub fn get_camera_position() -> Vec3<f32> {
    let transform = get_camera_transform();

    Vec3 {
        x: transform.x,
        y: transform.y,
        z: transform.z,
    }
}

#[must_use]
pub fn get_camera_rotation() -> Vec2<f32> {
    let transform = get_camera_transform();

    Vec2 {
        x: transform.yaw,
        y: transform.pitch,
    }
}

pub fn set_camera_x(x: f32) {
    let mut transform = get_camera_transform();
    transform.x = x;
    set_camera_transform(transform);
}

pub fn set_camera_y(y: f32) {
    let mut transform = get_camera_transform();
    transform.y = y;
    set_camera_transform(transform);
}

pub fn set_camera_z(z: f32) {
    let mut transform = get_camera_transform();
    transform.z = z;
    set_camera_transform(transform);
}

pub fn set_camera_yaw(yaw: f32) {
    let mut transform = get_camera_transform();
    transform.yaw = yaw;
    set_camera_transform(transform);
}

pub fn set_camera_pitch(pitch: f32) {
    let mut transform = get_camera_transform();
    transform.pitch = pitch;
    set_camera_transform(transform);
}

pub fn set_camera_position(position: Vec3<f32>) {
    let mut transform = get_camera_transform();
    transform.x = position.x;
    transform.y = position.y;
    transform.z = position.z;
    set_camera_transform(transform);
}

pub fn set_camera_rotation(position: Vec2<f32>) {
    let mut transform = get_camera_transform();
    transform.yaw = position.x;
    transform.pitch = position.y;
    set_camera_transform(transform);
}

#[must_use]
pub fn get_camera_forward_vector() -> Vec3<f32> {
    let transform = get_camera_transform();
    let yaw = transform.yaw;
    let pitch = transform.pitch;

    let cos_pitch = pitch.cos();
    let sin_pitch = pitch.sin();
    let cos_yaw = yaw.cos();
    let sin_yaw = yaw.sin();

    Vec3 {
        x: -sin_yaw * cos_pitch,
        y: sin_pitch,
        z: -cos_yaw * cos_pitch,
    }
}

#[must_use]
pub fn get_camera_right_vector() -> Vec3<f32> {
    let forward = get_camera_forward_vector();
    let up = Vec3::new(0.0, 1.0, 0.0);

    forward.cross(up).normalized()
}

pub fn update_debug_camera(sens: f64, speed: f32) {
    use crate::Vec3;
    use crate::camera::{
        get_camera_forward_vector, get_camera_pitch, get_camera_position, get_camera_right_vector,
        get_camera_yaw, set_camera_pitch, set_camera_position, set_camera_yaw,
    };
    use crate::input::{
        get_mouse_accumulated_dx, get_mouse_accumulated_dy, grab_mouse, is_key_down,
        is_key_just_pressed, release_mouse,
    };
    use crate::keys::{KEY_A, KEY_D, KEY_G, KEY_LEFT_SHIFT, KEY_R, KEY_S, KEY_SPACE, KEY_W};
    use std::ops::{Add, Mul};

    if is_key_just_pressed(KEY_G) {
        grab_mouse();
    }
    if is_key_just_pressed(KEY_R) {
        release_mouse();
    }

    set_camera_yaw(get_mouse_accumulated_dx().mul_add(-sens, f64::from(get_camera_yaw())) as f32);
    set_camera_pitch(
        get_mouse_accumulated_dy().mul_add(-sens, f64::from(get_camera_pitch())) as f32,
    );

    let mut position = get_camera_position();
    let forward = get_camera_forward_vector();
    let right = get_camera_right_vector();
    let up = Vec3::new(0.0, 1.0, 0.0);

    if is_key_down(KEY_W) {
        position = position.add(forward.mul(speed));
    }
    if is_key_down(KEY_S) {
        position = position.add(forward.mul(-speed));
    }

    if is_key_down(KEY_A) {
        position = position.add(right.mul(-speed));
    }
    if is_key_down(KEY_D) {
        position = position.add(right.mul(speed));
    }

    if is_key_down(KEY_SPACE) {
        position = position.add(up.mul(speed));
    }
    if is_key_down(KEY_LEFT_SHIFT) {
        position = position.add(up.mul(-speed));
    }

    set_camera_position(position);
}
