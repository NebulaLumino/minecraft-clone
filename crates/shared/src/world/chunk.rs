//! Chunk data structures

use serde::{Deserialize, Serialize};

use crate::types::{BlockLight, BlockPos, BlockState, ChunkPos, SkyLight};
use crate::constants::{CHUNK_SIZE, CHUNK_HEIGHT, CHUNK_SECTION_SIZE};

/// A 16x16x256 chunk of blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub position: ChunkPos,
    pub sections: Vec<ChunkSection>,
    #[serde(rename = "heightmap")]
    pub heightmap: Vec<u16>,
    #[serde(rename = "biomeData")]
    pub biome_data: Vec<u8>,
}

impl Chunk {
    pub fn new(position: ChunkPos) -> Self {
        let section_count = CHUNK_HEIGHT as usize / CHUNK_SECTION_SIZE as usize;
        Self {
            position,
            sections: vec![ChunkSection::empty(); section_count],
            heightmap: vec![0; 256],
            biome_data: vec![0; 64],
        }
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> BlockState {
        let section_index = (y >> 4) as usize;
        if section_index >= self.sections.len() {
            return BlockState::AIR;
        }

        let section = &self.sections[section_index];
        let local_y = (y & 0xF) as usize;
        let block_index = local_y * (CHUNK_SIZE as usize * CHUNK_SIZE as usize) + (z as usize * CHUNK_SIZE as usize) + x as usize;

        if block_index >= section.blocks.len() {
            return BlockState::AIR;
        }

        section.blocks[block_index]
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, state: BlockState) {
        let section_index = (y >> 4) as usize;
        if section_index >= self.sections.len() {
            return;
        }

        let section = &mut self.sections[section_index];
        let local_y = (y & 0xF) as usize;
        let block_index = local_y * (CHUNK_SIZE as usize * CHUNK_SIZE as usize) + (z as usize * CHUNK_SIZE as usize) + x as usize;

        if block_index < section.blocks.len() {
            section.blocks[block_index] = state;
        }
    }

    pub fn get_sky_light(&self, x: i32, y: i32, z: i32) -> SkyLight {
        let section_index = (y >> 4) as usize;
        if section_index >= self.sections.len() {
            return 15;
        }

        let section = &self.sections[section_index];
        let local_y = (y & 0xF) as usize;
        let light_index = local_y * (CHUNK_SIZE as usize * CHUNK_SIZE as usize) + (z as usize * CHUNK_SIZE as usize) + x as usize;

        section.sky_light.get(light_index).copied().unwrap_or(15)
    }

    pub fn get_block_light(&self, x: i32, y: i32, z: i32) -> BlockLight {
        let section_index = (y >> 4) as usize;
        if section_index >= self.sections.len() {
            return 0;
        }

        let section = &self.sections[section_index];
        let local_y = (y & 0xF) as usize;
        let light_index = local_y * (CHUNK_SIZE as usize * CHUNK_SIZE as usize) + (z as usize * CHUNK_SIZE as usize) + x as usize;

        section.block_light.get(light_index).copied().unwrap_or(0)
    }
}

/// A 16x16x16 section within a chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkSection {
    pub blocks: Vec<BlockState>,
    pub sky_light: Vec<SkyLight>,
    pub block_light: Vec<BlockLight>,
    pub non_air_count: u32,
    pub valid: bool,
}

impl ChunkSection {
    pub fn empty() -> Self {
        let size = (CHUNK_SECTION_SIZE as usize).pow(3);
        Self {
            blocks: vec![BlockState::AIR; size],
            sky_light: vec![15; size],
            block_light: vec![0; size],
            non_air_count: 0,
            valid: false,
        }
    }
}

/// Convert world BlockPos to chunk-relative position
pub fn world_to_chunk(pos: BlockPos) -> (ChunkPos, BlockPos) {
    let chunk_x = pos.x >> 4;
    let chunk_z = pos.z >> 4;
    let local_x = pos.x & 0xF;
    let local_y = pos.y;
    let local_z = pos.z & 0xF;
    (ChunkPos::new(chunk_x, chunk_z), BlockPos::new(local_x, local_y, local_z))
}
