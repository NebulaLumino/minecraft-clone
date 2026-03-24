//! Biome definitions and generation
//!
//! Defines biomes and their properties for terrain generation.

use crate::types::BlockId;
use crate::world::BiomeId;

/// Biome properties for generation
#[derive(Debug, Clone, Copy)]
pub struct BiomeProperties {
    /// Surface block (grass, sand, etc.)
    pub surface_block: BlockId,
    /// Underground block (dirt, etc.)
    pub underground_block: BlockId,
    /// Whether this biome has water
    pub has_water: bool,
    /// Water block ID
    pub water_block: BlockId,
    /// Tree density per chunk (0.0 - 1.0)
    pub tree_density: f32,
    /// Grass color (RGB)
    pub grass_color: [u8; 3],
    /// Foliage color (RGB)
    pub foliage_color: [u8; 3],
}

/// Get biome properties by biome ID
pub fn get_biome_properties(biome_id: BiomeId) -> BiomeProperties {
    match biome_id {
        0 => BiomeProperties { // Plains
            surface_block: 2,  // Grass
            underground_block: 3, // Dirt
            has_water: false,
            water_block: 8,     // Water
            tree_density: 0.02,
            grass_color: [0x7E, 0xBB, 0x58],
            foliage_color: [0x5A, 0x8A, 0x29],
        },
        1 => BiomeProperties { // Desert
            surface_block: 12, // Sand
            underground_block: 12, // Sand
            has_water: false,
            water_block: 8,
            tree_density: 0.0,
            grass_color: [0xBD, 0xB3, 0x91],
            foliage_color: [0xBD, 0xB3, 0x91],
        },
        2 => BiomeProperties { // Mountains
            surface_block: 1,  // Stone
            underground_block: 1, // Stone
            has_water: false,
            water_block: 8,
            tree_density: 0.0,
            grass_color: [0x88, 0x8A, 0x51],
            foliage_color: [0x56, 0x6A, 0x29],
        },
        3 => BiomeProperties { // Forest
            surface_block: 2,  // Grass
            underground_block: 3, // Dirt
            has_water: false,
            water_block: 8,
            tree_density: 0.1,
            grass_color: [0x5A, 0x9A, 0x31],
            foliage_color: [0x4A, 0x7A, 0x21],
        },
        4 => BiomeProperties { // Taiga
            surface_block: 2,  // Grass (snow will cover)
            underground_block: 3, // Dirt
            has_water: false,
            water_block: 8,
            tree_density: 0.08,
            grass_color: [0x5A, 0x9A, 0x31],
            foliage_color: [0x56, 0x6A, 0x29],
        },
        5 => BiomeProperties { // Snowy Tundra
            surface_block: 80, // Snow block
            underground_block: 3, // Dirt
            has_water: false,
            water_block: 8,
            tree_density: 0.0,
            grass_color: [0xFF, 0xFF, 0xFF],
            foliage_color: [0xFF, 0xFF, 0xFF],
        },
        6 => BiomeProperties { // Swamp
            surface_block: 2,  // Grass
            underground_block: 3, // Dirt
            has_water: true,
            water_block: 8,
            tree_density: 0.05,
            grass_color: [0x6A, 0x8A, 0x47],
            foliage_color: [0x5A, 0x7A, 0x31],
        },
        7 => BiomeProperties { // River
            surface_block: 12, // Sand
            underground_block: 12,
            has_water: true,
            water_block: 8,
            tree_density: 0.0,
            grass_color: [0x47, 0x8A, 0x85],
            foliage_color: [0x47, 0x8A, 0x85],
        },
        8 => BiomeProperties { // Nether
            surface_block: 87, // Soul sand
            underground_block: 1,
            has_water: false,
            water_block: 0,
            tree_density: 0.0,
            grass_color: [0xBF, 0x5A, 0x1F],
            foliage_color: [0xBF, 0x5A, 0x1F],
        },
        24 => BiomeProperties { // The End
            surface_block: 121, // End stone
            underground_block: 121,
            has_water: false,
            water_block: 0,
            tree_density: 0.0,
            grass_color: [0xFF, 0xFF, 0xFF],
            foliage_color: [0xFF, 0xFF, 0xFF],
        },
        _ => BiomeProperties { // Default
            surface_block: 2,
            underground_block: 3,
            has_water: false,
            water_block: 8,
            tree_density: 0.0,
            grass_color: [0x7E, 0xBB, 0x58],
            foliage_color: [0x5A, 0x8A, 0x29],
        },
    }
}
