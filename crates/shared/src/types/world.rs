//! World-related types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// World identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldId(pub Uuid);

impl Default for WorldId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

/// World dimension
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum Dimension {
    Overworld = 0,
    Nether = 1,
    End = 2,
}

impl Dimension {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Overworld),
            1 => Some(Self::Nether),
            2 => Some(Self::End),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// World metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMeta {
    pub id: WorldId,
    pub name: String,
    pub seed: u64,
    pub dimension: Dimension,
    pub generator: WorldGenerator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldGenerator {
    Vanilla,
    Flat,
    LargeBiomes,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self::Vanilla
    }
}
