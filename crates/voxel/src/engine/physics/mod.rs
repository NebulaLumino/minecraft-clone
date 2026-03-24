//! Physics module
//!
//! AABB collision, gravity, movement, raycast.

use glam::Vec3;
use voxel_shared::{BlockPos, BlockState};

/// Physics state for a player
pub struct Physics {
    pub on_ground: bool,
    pub velocity: Vec3,
    pub width: f32,
    pub height: f32,
}

impl Physics {
    /// Create new physics state
    pub fn new() -> Self {
        Self {
            on_ground: false,
            velocity: Vec3::ZERO,
            width: 0.6,
            height: 1.8,
        }
    }

    /// Apply gravity each tick
    pub fn tick(&mut self) {
        // Minecraft-like gravity: -0.08 blocks per tick²
        self.velocity.y -= 0.08;
        if self.velocity.y < -3.92 {
            self.velocity.y = -3.92;
        }
    }

    /// Check if player is on ground
    pub fn check_ground(&mut self, world: &impl WorldAccessor) {
        let feet_y = self.velocity.y;
        // Simple ground check - if there's a block at feet level
        let block_pos = BlockPos::new(0, (feet_y - 0.1) as i32, 0);
        let block = world.get_block(block_pos);
        self.on_ground = block.id != 0;
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}

/// Raycast result
pub struct RaycastHit {
    pub position: BlockPos,
    pub face: u8, // 0-5 for +X, -X, +Y, -Y, +Z, -Z
}

/// World accessor trait for physics
pub trait WorldAccessor {
    fn get_block(&self, pos: BlockPos) -> BlockState;
}

/// Perform raycast from origin in direction
pub fn raycast(
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    world: &impl WorldAccessor,
) -> Option<RaycastHit> {
    let step = 0.05;
    let mut pos = origin;
    let direction = direction.normalize();

    for _ in 0..(max_distance / step) as i32 {
        pos += direction * step;

        let block_pos = BlockPos::new(pos.x as i32, pos.y as i32, pos.z as i32);
        let block = world.get_block(block_pos);

        if block.id != 0 {
            return Some(RaycastHit {
                position: block_pos,
                face: 0, // Simplified
            });
        }
    }

    None
}

/// AABB for collision
#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn from_position(pos: Vec3, width: f32, height: f32) -> Self {
        let half = width / 2.0;
        Self {
            min: Vec3::new(pos.x - half, pos.y, pos.z - half),
            max: Vec3::new(pos.x + half, pos.y + height, pos.z + half),
        }
    }

    /// Check if this AABB intersects with another
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x < other.max.x && self.max.x > other.min.x
            && self.min.y < other.max.y && self.max.y > other.min.y
            && self.min.z < other.max.z && self.max.z > other.min.z
    }
}
