//! Terrain module
//!
//! World generation using Perlin noise from voxel-shared.

use voxel_shared::{ChunkPos, BlockPos, BlockState};
use voxel_shared::world::Chunk;
use voxel_shared::world::generation::terrain::WorldGenerator;
use crate::engine::physics::WorldAccessor;
use std::collections::HashMap;

/// World terrain manager
pub struct Terrain {
    seed: u64,
    chunks: HashMap<ChunkPos, Chunk>,
    generator: WorldGenerator,
}

impl Terrain {
    /// Create new terrain with seed
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            chunks: HashMap::new(),
            generator: WorldGenerator::new(seed),
        }
    }

    /// Get block at position
    pub fn get_block(&self, pos: BlockPos) -> BlockState {
        let chunk_pos = ChunkPos::new(pos.x / 16, pos.z / 16);

        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            let local_x = ((pos.x % 16) + 16) % 16;
            let local_z = ((pos.z % 16) + 16) % 16;
            chunk.get_block(local_x as i32, pos.y as i32, local_z as i32)
        } else {
            BlockState::AIR
        }
    }

    /// Set block at position
    pub fn set_block(&mut self, pos: BlockPos, state: BlockState) {
        let chunk_pos = ChunkPos::new(pos.x / 16, pos.z / 16);

        // Generate chunk if not present
        if !self.chunks.contains_key(&chunk_pos) {
            let chunk = self.generate_chunk(chunk_pos);
            self.chunks.insert(chunk_pos, chunk);
        }

        // Get mutable reference to chunk
        let chunk = self.chunks.get_mut(&chunk_pos).unwrap();

        let local_x = ((pos.x % 16) + 16) % 16;
        let local_z = ((pos.z % 16) + 16) % 16;
        chunk.set_block(local_x as i32, pos.y as i32, local_z as i32, state);
    }

    /// Generate chunk at position using world generator
    fn generate_chunk(&self, pos: ChunkPos) -> Chunk {
        self.generator.generate_chunk(pos)
    }

    /// Get chunk at position (generates if not loaded)
    pub fn get_chunk(&mut self, pos: ChunkPos) -> Option<&Chunk> {
        if !self.chunks.contains_key(&pos) {
            let chunk = self.generate_chunk(pos);
            self.chunks.insert(pos, chunk);
        }
        self.chunks.get(&pos)
    }

    /// Get world seed
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Get loaded chunk positions
    pub fn loaded_chunks(&self) -> Vec<ChunkPos> {
        self.chunks.keys().cloned().collect()
    }

    /// Get reference to all loaded chunks
    pub fn chunks(&self) -> &HashMap<ChunkPos, Chunk> {
        &self.chunks
    }

    /// Get surface height at world position (X, Z)
    /// Returns the Y coordinate of the highest solid block
    pub fn get_height_at(&self, world_x: i32, world_z: i32) -> i32 {
        let chunk_pos = ChunkPos::new(world_x / 16, world_z / 16);

        if let Some(chunk) = self.chunks.get(&chunk_pos) {
            let local_x = ((world_x % 16) + 16) % 16;
            let local_z = ((world_z % 16) + 16) % 16;
            let index = (local_x as usize) * 16 + (local_z as usize);
            chunk.heightmap.get(index).copied().unwrap_or(64) as i32
        } else {
            64 // Default to sea level for unloaded chunks
        }
    }
}

impl WorldAccessor for Terrain {
    fn get_block(&self, pos: BlockPos) -> BlockState {
        self.get_block(pos)
    }
}
