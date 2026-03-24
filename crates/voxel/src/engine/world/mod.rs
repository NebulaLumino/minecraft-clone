//! World module
//!
//! Client-side chunk management and rendering.

use voxel_shared::{ChunkPos, BlockPos, BlockState};
use std::collections::HashMap;

/// Chunk manager - manages loaded chunks for rendering
pub struct World {
    chunks: HashMap<ChunkPos, ChunkData>,
    load_radius: i32,
}

struct ChunkData {
    // Placeholder for chunk mesh data
    _placeholder: u8,
}

impl World {
    /// Create new world manager
    pub fn new(load_radius: i32) -> Self {
        Self {
            chunks: HashMap::new(),
            load_radius,
        }
    }

    /// Get a block at position
    pub fn get_block(&self, _pos: BlockPos) -> BlockState {
        // Placeholder - would look up from chunks
        BlockState::AIR
    }
}
