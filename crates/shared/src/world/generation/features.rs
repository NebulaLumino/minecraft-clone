//! Feature generation (caves, ore, trees, structures)
//!
//! Generates underground caves, ore veins, and surface features.

use crate::types::BlockState;
use crate::world::Chunk;
use crate::utils::random::{PerlinNoise, SeededRng};
use crate::constants::{CHUNK_SIZE, CHUNK_HEIGHT};

/// Feature generator for caves, ores, trees, etc.
pub struct FeatureGenerator {
    seed: u64,
    cave_noise: PerlinNoise,
    tree_noise: PerlinNoise,
    ore_noise: PerlinNoise,
}

impl FeatureGenerator {
    /// Create a new feature generator
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            cave_noise: PerlinNoise::new(seed.wrapping_add(100)),
            tree_noise: PerlinNoise::new(seed.wrapping_add(200)),
            ore_noise: PerlinNoise::new(seed.wrapping_add(300)),
        }
    }

    /// Apply caves to a chunk
    pub fn apply_caves(&self, chunk: &mut Chunk) {
        let cave_scale = 0.05;
        let threshold = 0.55;

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 5..(CHUNK_HEIGHT - 10) {
                    let world_x = (chunk.position.x * CHUNK_SIZE + x) as f64;
                    let world_z = (chunk.position.z * CHUNK_SIZE + z) as f64;

                    let noise = self.cave_noise.noise3d(
                        world_x * cave_scale,
                        y as f64 * cave_scale,
                        world_z * cave_scale,
                    );

                    // 3D Perlin can create organic cave shapes
                    // Spherical carving for larger caves
                    let dist = (x as f64 - 8.0).powi(2)
                        + (y as f64 - 40.0).powi(2)
                        + (z as f64 - 8.0).powi(2);
                    let sphere_carve = dist < 25.0;

                    if noise > threshold || sphere_carve {
                        // Don't carve through bedrock
                        let current = chunk.get_block(x, y, z);
                        if current.id != 7 {
                            chunk.set_block(x, y, z, BlockState::AIR);
                        }
                    }
                }
            }
        }
    }

    /// Apply ore veins to a chunk
    pub fn apply_ores(&self, chunk: &mut Chunk) {
        // Ore generation is handled in terrain.rs
        // This can be extended for more complex ore patterns
        let _ = chunk;
    }

    /// Check if a tree should be placed at position
    pub fn should_place_tree(&self, x: i32, z: i32, surface_y: i32, chunk: &Chunk) -> bool {
        if surface_y < 64 || surface_y > 100 {
            return false;
        }

        // Check surface is grass
        let surface_block = chunk.get_block(x, surface_y, z);
        if surface_block.id != 2 {
            return false;
        }

        // Use noise to determine tree placement
        let noise = self.tree_noise.noise3d(
            (chunk.position.x * CHUNK_SIZE + x) as f64 * 0.1,
            (chunk.position.z * CHUNK_SIZE + z) as f64 * 0.1,
            0.0,
        );

        noise > 0.7
    }

    /// Get tree trunk height
    pub fn get_tree_height(&self, _x: i32, _z: i32) -> i32 {
        // Deterministic height based on position
        let mut rng = SeededRng::new(
            self.seed
                .wrapping_add((_x as u64) << 16)
                .wrapping_add(_z as u64)
        );
        (rng.next_u32_bound(4) + 4) as i32 // 4-7 blocks tall
    }
}

impl Default for FeatureGenerator {
    fn default() -> Self {
        Self::new(0)
    }
}
