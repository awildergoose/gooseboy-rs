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

pub fn get_camera_x() -> f32 {
    get_camera_transform().x
}

pub fn get_camera_y() -> f32 {
    get_camera_transform().y
}

pub fn get_camera_z() -> f32 {
    get_camera_transform().z
}

pub fn get_camera_yaw() -> f32 {
    get_camera_transform().yaw
}

pub fn get_camera_pitch() -> f32 {
    get_camera_transform().pitch
}

pub fn get_camera_position() -> Vec3<f32> {
    let transform = get_camera_transform();

    Vec3 {
        x: transform.x,
        y: transform.y,
        z: transform.z,
    }
}

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

pub fn get_camera_right_vector() -> Vec3<f32> {
    let forward = get_camera_forward_vector();
    let up = Vec3::new(0.0, 1.0, 0.0);

    forward.cross(up).normalized()
}
