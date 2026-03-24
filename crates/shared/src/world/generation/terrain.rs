//! Terrain generation using Perlin noise
//!
//! Generates heightmaps and block placement for chunks.

use crate::constants::{CHUNK_SIZE, CHUNK_HEIGHT, SEA_LEVEL};
use crate::types::{BlockId, BlockState, ChunkPos};
use crate::world::{Chunk, BiomeId};
use crate::utils::random::PerlinNoise;

/// World generator for creating terrain
pub struct WorldGenerator {
    seed: u64,
    terrain_noise: PerlinNoise,
    biome_noise: PerlinNoise,
    temperature_noise: PerlinNoise,
    humidity_noise: PerlinNoise,
    cave_noise: PerlinNoise,
    ore_noise: PerlinNoise,
}

impl WorldGenerator {
    /// Create a new world generator with a seed
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            terrain_noise: PerlinNoise::new(seed),
            biome_noise: PerlinNoise::new(seed.wrapping_add(1)),
            temperature_noise: PerlinNoise::new(seed.wrapping_add(2)),
            humidity_noise: PerlinNoise::new(seed.wrapping_add(3)),
            cave_noise: PerlinNoise::new(seed.wrapping_add(4)),
            ore_noise: PerlinNoise::new(seed.wrapping_add(5)),
        }
    }

    /// Generate a chunk at the given position
    pub fn generate_chunk(&self, position: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new(position);

        // Generate heightmap for this chunk
        let heightmap = self.generate_heightmap(position);

        // Generate biomes for this chunk
        let biomes = self.generate_biomes(position);

        // Fill blocks based on heightmap and biomes
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = (position.x * CHUNK_SIZE + x) as f64;
                let world_z = (position.z * CHUNK_SIZE + z) as f64;
                let surface_height = heightmap[(x as usize) * CHUNK_SIZE as usize + (z as usize)] as i32;
                let biome_id = biomes[(x as usize / 4) * 4 + (z as usize / 4)];

                // Determine block at each Y level
                for y in 0..CHUNK_HEIGHT {
                    let world_y = y as i32;
                    let block_id = self.get_block_at_y(
                        world_x as f64,
                        world_y as f64,
                        world_z as f64,
                        surface_height,
                        biome_id,
                    );

                    if block_id != 0 {
                        chunk.set_block(x, y, z, BlockState::new(block_id, 0));
                    }
                }

                // Update heightmap
                chunk.heightmap[(x as usize) * CHUNK_SIZE as usize + (z as usize)] = surface_height as u16;
            }
        }

        // Copy biome data
        for (i, &biome_id) in biomes.iter().enumerate() {
            chunk.biome_data[i] = biome_id;
        }

        chunk
    }

    /// Generate heightmap for a chunk using 2D Perlin noise
    fn generate_heightmap(&self, position: ChunkPos) -> Vec<u16> {
        let mut heightmap = vec![0u16; (CHUNK_SIZE * CHUNK_SIZE) as usize];
        let scale = 0.01; // Terrain noise scale

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = (position.x * CHUNK_SIZE + x) as f64;
                let world_z = (position.z * CHUNK_SIZE + z) as f64;

                // Multi-octave noise for more interesting terrain
                let noise = self.octave_noise_2d(world_x * scale, world_z * scale, 4);

                // Convert noise from [-1, 1] to height
                let height = SEA_LEVEL as f64 + (noise * 32.0) as f64;
                let height = height.clamp(0.0, (CHUNK_HEIGHT - 1) as f64) as u16;
                heightmap[(x as usize) * CHUNK_SIZE as usize + (z as usize)] = height;
            }
        }

        heightmap
    }

    /// Generate biome IDs for a chunk (lower resolution)
    fn generate_biomes(&self, position: ChunkPos) -> Vec<u8> {
        // 4x4 biome resolution per chunk
        let mut biomes = vec![0u8; 16]; // 4x4 grid
        let scale = 0.005; // Biome noise scale

        for x in 0..4i32 {
            for z in 0..4i32 {
                let world_x = (position.x * 4 + x) as f64;
                let world_z = (position.z * 4 + z) as f64;

                let temp = self.temperature_noise.noise3d(world_x * scale, 0.0, world_z * scale);
                let humidity = self.humidity_noise.noise3d(world_x * scale, 100.0, world_z * scale);

                let biome_id = self.select_biome(temp, humidity);
                biomes[(x * 4 + z) as usize] = biome_id;
            }
        }

        biomes
    }

    /// Select biome based on temperature and humidity
    fn select_biome(&self, temperature: f64, humidity: f64) -> BiomeId {
        let temp = (temperature + 1.0) / 2.0; // Convert to [0, 1]
        let humid = (humidity + 1.0) / 2.0;

        if temp < 0.2 {
            if humid < 0.3 {
                BiomeId::from(5) // Ice (snowy_tundra)
            } else {
                BiomeId::from(4) // Taiga
            }
        } else if temp < 0.4 {
            if humid < 0.5 {
                BiomeId::from(2) // Mountains
            } else {
                BiomeId::from(4) // Taiga
            }
        } else if temp < 0.6 {
            if humid < 0.3 {
                BiomeId::from(1) // Desert
            } else if humid < 0.6 {
                BiomeId::from(0) // Plains
            } else {
                BiomeId::from(3) // Forest
            }
        } else if temp < 0.8 {
            if humid < 0.5 {
                BiomeId::from(0) // Plains
            } else {
                BiomeId::from(3) // Forest
            }
        } else {
            BiomeId::from(6) // Swamp
        }
    }

    /// Get block ID at a specific Y coordinate
    fn get_block_at_y(&self, x: f64, y: f64, z: f64, surface_height: i32, _biome_id: BiomeId) -> BlockId {
        let y = y as i32;

        // Above surface - air
        if y > surface_height {
            // Check for water
            if y > SEA_LEVEL && y <= 64 {
                return 8; // Water
            }
            return 0; // Air
        }

        // At surface
        if y == surface_height {
            // Surface block based on biome (simplified)
            if y <= SEA_LEVEL {
                return 12; // Sand (beach)
            }
            return 2; // Grass
        }

        // Below surface
        if y > surface_height - 4 {
            return 3; // Dirt
        }

        // Stone layer
        if y > 5 {
            // Cave detection using 3D noise
            let cave_scale = 0.05;
            let cave_noise = self.cave_noise.noise3d(x * cave_scale, y as f64 * cave_scale, z * cave_scale);

            if cave_noise > 0.55 {
                return 0; // Cave (air)
            }

            // Ore generation
            if self.should_place_ore(y) {
                return self.get_ore_type(y);
            }

            return 1; // Stone
        }

        // Bedrock layer (bottom 5 blocks)
        if y <= 4 {
            return 7; // Bedrock
        }

        1 // Stone
    }

    /// Check if ore should be placed at this depth
    fn should_place_ore(&self, y: i32) -> bool {
        let ore_scale = 0.15;
        let noise = self.ore_noise.noise3d(y as f64 * ore_scale, y as f64 * ore_scale, y as f64 * ore_scale);
        noise > 0.7
    }

    /// Get ore type based on depth
    fn get_ore_type(&self, y: i32) -> BlockId {
        // Diamond: y < 16
        if y < 16 {
            let noise = self.ore_noise.noise3d(y as f64 * 0.2, 0.0, 0.0);
            if noise > 0.85 {
                return 56; // Diamond ore
            }
        }

        // Gold: y < 32
        if y < 32 {
            let noise = self.ore_noise.noise3d(y as f64 * 0.18, 10.0, 0.0);
            if noise > 0.8 {
                return 14; // Gold ore
            }
        }

        // Iron: y < 64
        if y < 64 {
            let noise = self.ore_noise.noise3d(y as f64 * 0.16, 20.0, 0.0);
            if noise > 0.75 {
                return 15; // Iron ore
            }
        }

        // Coal: most depths
        let noise = self.ore_noise.noise3d(y as f64 * 0.14, 30.0, 0.0);
        if noise > 0.7 {
            return 16; // Coal ore
        }

        0 // No ore
    }

    /// Multi-octave 2D noise for more interesting terrain
    fn octave_noise_2d(&self, x: f64, z: f64, octaves: usize) -> f64 {
        let mut noise = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            noise += self.terrain_noise.noise3d(x * frequency, 0.0, z * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        noise / max_value
    }

    /// Get the generator seed
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self::new(0)
    }
}
