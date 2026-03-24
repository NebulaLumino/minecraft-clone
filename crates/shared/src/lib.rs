//! VoxelCraft Shared Library
//!
//! Cross-cutting types and logic shared between client and server.
//! Contains no external dependencies — pure Rust types.

pub mod constants;
pub mod types;
pub mod world;
pub mod protocol;
pub mod utils;

pub use constants::*;
pub use types::*;

// Re-export world module contents, but NOT chunk since protocol also has chunk
pub use world::biome::*;
pub use world::generation::*;
pub use world::generation::terrain::WorldGenerator;

// Protocol items
pub use protocol::*;
