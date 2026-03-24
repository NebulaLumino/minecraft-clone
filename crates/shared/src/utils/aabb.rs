//! Axis-Aligned Bounding Box utilities

use serde::{Deserialize, Serialize};

/// AABB in 3D space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Aabb {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

impl Aabb {
    pub fn new(min: [f64; 3], max: [f64; 3]) -> Self {
        Self {
            min_x: min[0],
            min_y: min[1],
            min_z: min[2],
            max_x: max[0],
            max_y: max[1],
            max_z: max[2],
        }
    }

    pub fn from_center_size(center: [f64; 3], size: [f64; 3]) -> Self {
        let half = [size[0] * 0.5, size[1] * 0.5, size[2] * 0.5];
        Self::new(
            [center[0] - half[0], center[1] - half[1], center[2] - half[2]],
            [center[0] + half[0], center[1] + half[1], center[2] + half[2]],
        )
    }

    pub fn center(&self) -> [f64; 3] {
        [
            (self.min_x + self.max_x) * 0.5,
            (self.min_y + self.max_y) * 0.5,
            (self.min_z + self.max_z) * 0.5,
        ]
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
            && self.min_z < other.max_z
            && self.max_z > other.min_z
    }

    pub fn contains_point(&self, x: f64, y: f64, z: f64) -> bool {
        x >= self.min_x
            && x < self.max_x
            && y >= self.min_y
            && y < self.max_y
            && z >= self.min_z
            && z < self.max_z
    }

    pub fn clip_velocity(&self, velocity: [f64; 3], friction: f64) -> [f64; 3] {
        [
            velocity[0] * friction,
            velocity[1] * friction,
            velocity[2] * friction,
        ]
    }

    pub fn expand(&self, x: f64, y: f64, z: f64) -> Self {
        Self {
            min_x: self.min_x - x,
            min_y: self.min_y - y,
            min_z: self.min_z - z,
            max_x: self.max_x + x,
            max_y: self.max_y + y,
            max_z: self.max_z + z,
        }
    }

    pub fn contract(&self, x: f64, y: f64, z: f64) -> Self {
        self.expand(-x, -y, -z)
    }
}

/// Player-sized AABB (1.8 blocks tall, 0.6 wide)
pub const PLAYER_AABB: Aabb = Aabb {
    min_x: -0.3,
    min_y: 0.0,
    min_z: -0.3,
    max_x: 0.3,
    max_y: 1.8,
    max_z: 0.3,
};
