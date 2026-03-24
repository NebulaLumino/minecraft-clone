//! Player module
//!
//! Local player controller with physics.

use glam::Vec3;
use crate::engine::physics::Physics;
use crate::game::Terrain;
use voxel_shared::BlockPos;

/// Raycast hit result
pub struct BlockTarget {
    pub position: BlockPos,
    pub face: u8,
}

/// Player entity
pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Vec3, // yaw, pitch, roll
    pub physics: Physics,
    pub on_ground: bool,
    pub sprinting: bool,
    pub sneaking: bool,
}

impl Player {
    /// Create new player
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 100.0, 0.0), // Spawn high up, will fall to ground
            velocity: Vec3::ZERO,
            rotation: Vec3::ZERO,
            physics: Physics::new(),
            on_ground: false,
            sprinting: false,
            sneaking: false,
        }
    }

    /// Update player
    pub fn update(&mut self, delta: f32, world: &Terrain) {
        // Apply gravity
        self.physics.tick();

        // Apply physics velocity
        self.position += self.physics.velocity * delta;

        // Get terrain height at current position
        let ground_level = world.get_height_at(self.position.x as i32, self.position.z as i32) as f32 + 1.0;

        // Ground collision
        if self.position.y < ground_level {
            self.position.y = ground_level;
            self.physics.velocity.y = 0.0;
            self.on_ground = true;
            self.physics.on_ground = true;
        } else {
            self.on_ground = false;
            self.physics.on_ground = false;
        }

        // Simple friction when on ground
        if self.on_ground {
            self.physics.velocity.x *= 0.8;
            self.physics.velocity.z *= 0.8;
        }
    }

    /// Move player in direction relative to look
    pub fn move_relative(&mut self, forward: f32, strafe: f32) {
        // Apply movement force (similar to Minecraft: 0.02 per frame)
        self.physics.velocity.x += strafe * 0.15;
        self.physics.velocity.z += forward * 0.15;
    }

    /// Jump
    pub fn jump(&mut self) {
        if self.on_ground {
            self.physics.velocity.y = 0.4;
            self.on_ground = false;
        }
    }

    /// Move player
    pub fn move_to(&mut self, pos: Vec3) {
        self.position = pos;
    }

    /// Get position
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Set yaw
    pub fn set_yaw(&mut self, yaw: f32) {
        self.rotation.x = yaw;
    }

    /// Set pitch
    pub fn set_pitch(&mut self, pitch: f32) {
        self.rotation.y = pitch.clamp(-89.0, 89.0);
    }

    /// Get look direction based on yaw and pitch
    pub fn get_look_direction(&self) -> Vec3 {
        let yaw = self.rotation.x;
        let pitch = self.rotation.y;
        Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        ).normalize()
    }

    /// Get eye position (position + eye height)
    pub fn get_eye_position(&self) -> Vec3 {
        Vec3::new(self.position.x, self.position.y + 1.6, self.position.z)
    }

    /// Raycast to find targeted block
    pub fn get_targeted_block(&self, world: &Terrain, max_distance: f32) -> Option<BlockTarget> {
        let origin = self.get_eye_position();
        let direction = self.get_look_direction();

        let step = 0.1;
        let mut pos = origin;
        let direction = direction.normalize();

        for _ in 0..(max_distance / step) as i32 {
            pos += direction * step;

            let block_pos = BlockPos::new(pos.x as i32, pos.y as i32, pos.z as i32);
            let block = world.get_block(block_pos);

            if block.id != 0 {
                // Determine which face was hit based on position
                let face = Self::determine_hit_face(origin, pos);

                return Some(BlockTarget {
                    position: block_pos,
                    face,
                });
            }
        }

        None
    }

    /// Determine which face was hit based on hit position relative to origin
    fn determine_hit_face(origin: Vec3, hit_pos: Vec3) -> u8 {
        let dx = hit_pos.x - origin.x;
        let dy = hit_pos.y - origin.y;
        let dz = hit_pos.z - origin.z;

        // Find the dominant direction
        let ax = dx.abs();
        let ay = dy.abs();
        let az = dz.abs();

        if ax >= ay && ax >= az {
            // X is dominant
            if dx > 0.0 {
                0 // +X face
            } else {
                1 // -X face
            }
        } else if ay >= ax && ay >= az {
            // Y is dominant
            if dy > 0.0 {
                2 // +Y face
            } else {
                3 // -Y face
            }
        } else {
            // Z is dominant
            if dz > 0.0 {
                4 // +Z face
            } else {
                5 // -Z face
            }
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
