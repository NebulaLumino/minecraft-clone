//! World generation and dimension constants

/// Chunk width in blocks
pub const CHUNK_SIZE: i32 = 16;

/// Chunk height in blocks (Overworld)
pub const CHUNK_HEIGHT: i32 = 256;

/// Chunk section height in blocks (16 in vanilla)
pub const CHUNK_SECTION_SIZE: i32 = 16;

/// Number of chunks loaded around player (default view distance)
pub const DEFAULT_VIEW_DISTANCE: i32 = 10;

/// Maximum view distance in chunks
pub const MAX_VIEW_DISTANCE: i32 = 32;

/// Minimum view distance in chunks
pub const MIN_VIEW_DISTANCE: i32 = 2;

/// Maximum world height in blocks
pub const MAX_WORLD_HEIGHT: i32 = 320;

/// Minimum world height in blocks (bedrock floor)
pub const MIN_WORLD_HEIGHT: i32 = -64;

/// Sea level Y coordinate
pub const SEA_LEVEL: i32 = 64;

/// World border radius (in blocks from center)
pub const WORLD_BORDER_RADIUS: i32 = 30_000_000;

/// Default spawn Y (find suitable ground)
pub const DEFAULT_SPAWN_Y: i32 = 64;

/// Maximum entities per chunk
pub const MAX_ENTITIES_PER_CHUNK: usize = 256;

/// Number of heightmap entries per chunk column
pub const HEIGHTMAP_SIZE: usize = 256;

/// Biome noise scale
pub const BIOME_NOISE_SCALE: f64 = 0.1;

/// Terrain noise scale
pub const TERRAIN_NOISE_SCALE: f64 = 0.01;

/// Cave noise threshold
pub const CAVE_THRESHOLD: f64 = 0.5;

/// Number of ticks per second (20 TPS)
pub const TICKS_PER_SECOND: u32 = 20;

/// Tick duration in milliseconds
pub const TICK_DURATION_MS: u64 = 50;

/// Maximum chunk batch size for network transfer
pub const MAX_CHUNK_BATCH_SIZE: usize = 64;

/// Number of vertical sections per chunk
pub const SECTIONS_PER_CHUNK: i32 = CHUNK_HEIGHT / CHUNK_SECTION_SIZE;
