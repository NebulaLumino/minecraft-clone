//! Camera module
//!
//! First-person camera with mouse look and movement.

use glam::{Vec3, Mat4};

/// First-person camera
pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,   // Horizontal rotation (radians)
    pub pitch: f32, // Vertical rotation (radians)
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    /// Create new camera at position
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: 0.0,
            pitch: 0.0,
            fov: 70.0f32.to_radians(),
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Set yaw (horizontal rotation)
    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
    }

    /// Set pitch (vertical rotation), clamped to prevent flipping
    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(-89.0f32.to_radians(), 89.0f32.to_radians());
    }

    /// Add yaw
    pub fn add_yaw(&mut self, delta: f32) {
        self.yaw += delta;
    }

    /// Add pitch
    pub fn add_pitch(&mut self, delta: f32) {
        self.pitch = (self.pitch + delta).clamp(-89.0f32.to_radians(), 89.0f32.to_radians());
    }

    /// Get forward direction vector
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize()
    }

    /// Get right direction vector
    pub fn right(&self) -> Vec3 {
        Vec3::new(self.yaw.cos(), 0.0, self.yaw.sin()).normalize()
    }

    /// Get up direction vector
    pub fn up(&self) -> Vec3 {
        self.forward().cross(self.right())
    }

    /// Get view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.forward(), Vec3::Y)
    }

    /// Get projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Get view-projection matrix
    pub fn view_proj(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Set aspect ratio
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    /// Set camera position
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Move forward/backward
    pub fn move_forward(&mut self, amount: f32) {
        let mut dir = self.forward();
        dir.y = 0.0; // Keep horizontal
        self.position += dir.normalize() * amount;
    }

    /// Strafe left/right
    pub fn strafe(&mut self, amount: f32) {
        self.position += self.right() * amount;
    }

    /// Move up/down
    pub fn move_vertical(&mut self, amount: f32) {
        self.position.y += amount;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vec3::new(0.0, 80.0, 0.0))
    }
}
