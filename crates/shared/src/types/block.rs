//! Block and position types

use serde::{Deserialize, Serialize};

/// Block position (world coordinates)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn chunk_pos(&self) -> ChunkPos {
        ChunkPos::new(self.x >> 4, self.z >> 4)
    }

    pub fn section_index(&self) -> i32 {
        self.y >> 4
    }

    pub fn section_local_y(&self) -> i32 {
        self.y & 0xF
    }
}

/// Chunk position (chunk coordinates)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub const fn zero() -> Self {
        Self { x: 0, z: 0 }
    }

    pub fn block_to_chunk(block: i32) -> i32 {
        block >> 4
    }

    pub fn chunk_to_block(chunk: i32) -> i32 {
        chunk << 4
    }
}

/// Block ID type
pub type BlockId = u16;

/// Block state ID (includes properties)
pub type BlockStateId = u32;

/// Sky light value (0-15)
pub type SkyLight = u8;

/// Block light value (0-15)
pub type BlockLight = u8;

/// Block state with properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockState {
    pub id: BlockId,
    pub properties: u16,
}

impl BlockState {
    pub const AIR: Self = Self { id: 0, properties: 0 };

    pub const fn new(id: BlockId, properties: u16) -> Self {
        Self { id, properties }
    }
}

/// Direction/facing for blocks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

impl Direction {
    pub fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Down),
            1 => Some(Self::Up),
            2 => Some(Self::North),
            3 => Some(Self::South),
            4 => Some(Self::West),
            5 => Some(Self::East),
            _ => None,
        }
    }

    pub fn offset(&self) -> BlockPos {
        match self {
            Self::Down => BlockPos::new(0, -1, 0),
            Self::Up => BlockPos::new(0, 1, 0),
            Self::North => BlockPos::new(0, 0, -1),
            Self::South => BlockPos::new(0, 0, 1),
            Self::West => BlockPos::new(-1, 0, 0),
            Self::East => BlockPos::new(1, 0, 0),
        }
    }
}

/// Raycast hit result
#[derive(Debug, Clone)]
pub struct BlockHitResult {
    pub position: BlockPos,
    pub direction: Direction,
    pub fraction: f32,
}
