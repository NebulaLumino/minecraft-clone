//! Biome definitions

use serde::{Deserialize, Serialize};

/// Biome ID
pub type BiomeId = u8;

/// Biome categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomeCategory {
    Plains,
    Forest,
    Mountain,
    Taiga,
    Desert,
    Jungle,
    Mesa,
    Savanna,
    Ice,
    Swamp,
    Mushroom,
    TheEnd,
    Nether,
}

/// Biome definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biome {
    pub id: BiomeId,
    pub name: &'static str,
    pub category: BiomeCategory,
    pub rainfall: f32,
    pub temperature: f32,
    pub grass_color: u32,
    pub foliage_color: u32,
    pub depth: f32,
    pub scale: f32,
}

impl Biome {
    pub fn get_biome(id: BiomeId) -> Option<Self> {
        Some(match id {
            0 => Biome::plains(),
            1 => Biome::desert(),
            2 => Biome::mountains(),
            3 => Biome::forest(),
            4 => Biome::taiga(),
            5 => Biome::snowy_tundra(),
            6 => Biome::swamp(),
            7 => Biome::river(),
            8 => Biome::nether(),
            24 => Biome::the_end(),
            _ => return None,
        })
    }

    pub fn plains() -> Self {
        Self {
            id: 0,
            name: "plains",
            category: BiomeCategory::Plains,
            rainfall: 0.4,
            temperature: 0.8,
            grass_color: 0x7E_BB58,
            foliage_color: 0x5_8A_29,
            depth: 0.125,
            scale: 0.05,
        }
    }

    pub fn desert() -> Self {
        Self {
            id: 1,
            name: "desert",
            category: BiomeCategory::Desert,
            rainfall: 0.0,
            temperature: 2.0,
            grass_color: 0xBD_B391,
            foliage_color: 0xBD_B391,
            depth: 0.125,
            scale: 0.05,
        }
    }

    pub fn mountains() -> Self {
        Self {
            id: 2,
            name: "mountains",
            category: BiomeCategory::Mountain,
            rainfall: 0.3,
            temperature: 0.4,
            grass_color: 0x8_8A_51,
            foliage_color: 0x5_6A_29,
            depth: 1.0,
            scale: 0.3,
        }
    }

    pub fn forest() -> Self {
        Self {
            id: 3,
            name: "forest",
            category: BiomeCategory::Forest,
            rainfall: 0.8,
            temperature: 0.7,
            grass_color: 0x5_9A_31,
            foliage_color: 0x4_7A_21,
            depth: 0.1,
            scale: 0.05,
        }
    }

    pub fn taiga() -> Self {
        Self {
            id: 4,
            name: "taiga",
            category: BiomeCategory::Taiga,
            rainfall: 0.4,
            temperature: 0.25,
            grass_color: 0x5_9A_31,
            foliage_color: 0x5_6A_29,
            depth: 0.2,
            scale: 0.2,
        }
    }

    pub fn snowy_tundra() -> Self {
        Self {
            id: 5,
            name: "snowy_tundra",
            category: BiomeCategory::Ice,
            rainfall: 0.0,
            temperature: 0.0,
            grass_color: 0xFF_FF_FF,
            foliage_color: 0xFF_FF_FF,
            depth: 0.125,
            scale: 0.05,
        }
    }

    pub fn swamp() -> Self {
        Self {
            id: 6,
            name: "swamp",
            category: BiomeCategory::Swamp,
            rainfall: 0.8,
            temperature: 0.8,
            grass_color: 0x68_8A_47,
            foliage_color: 0x57_A3_1F,
            depth: -0.2,
            scale: 0.1,
        }
    }

    pub fn river() -> Self {
        Self {
            id: 7,
            name: "river",
            category: BiomeCategory::Plains,
            rainfall: 0.5,
            temperature: 0.5,
            grass_color: 0x47_8A_85,
            foliage_color: 0x47_8A_85,
            depth: -0.5,
            scale: 0.05,
        }
    }

    pub fn nether() -> Self {
        Self {
            id: 8,
            name: "nether",
            category: BiomeCategory::Nether,
            rainfall: 0.0,
            temperature: 2.0,
            grass_color: 0xBF_5A_1F,
            foliage_color: 0xBF_5A_1F,
            depth: 0.0,
            scale: 0.2,
        }
    }

    pub fn the_end() -> Self {
        Self {
            id: 24,
            name: "the_end",
            category: BiomeCategory::TheEnd,
            rainfall: 0.0,
            temperature: 0.5,
            grass_color: 0xFF_FF_FF,
            foliage_color: 0xFF_FF_FF,
            depth: 0.0,
            scale: 0.2,
        }
    }
}
