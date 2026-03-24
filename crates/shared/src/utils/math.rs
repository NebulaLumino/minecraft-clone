//! Math utilities

use glam::Vec3;

/// 3D float position
pub type Position = Vec3;

/// 3D integer position (block position)
pub type IPosition = [i32; 3];

/// Linear interpolation
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Smooth interpolation (smoothstep)
pub fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Inverse smoothstep
pub fn inverse_smoothstep(t: f32) -> f32 {
    if t <= 0.0 || t >= 1.0 {
        return t;
    }
    0.5 - ((0.5 - t) * (1.5 - t)).sqrt()
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Lerp for Vec3
pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3::new(
        lerp(a.x, b.x, t),
        lerp(a.y, b.y, t),
        lerp(a.z, b.z, t),
    )
}

/// Square of a value
pub fn square<T: std::ops::Mul<Output = T>>(value: T) -> T
where
    T: Copy,
{
    value * value
}

/// Floor a f32 to i32
pub fn floor_f32(value: f32) -> i32 {
    value as i32
}

/// Distance squared between two positions
pub fn distance_squared_3d(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let dz = b[2] - a[2];
    dx * dx + dy * dy + dz * dz
}

/// Angle interpolation
pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut delta = b - a;
    while delta < -180.0 {
        delta += 360.0;
    }
    while delta > 180.0 {
        delta -= 360.0;
    }
    a + delta * t
}
