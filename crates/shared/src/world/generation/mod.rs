//! World generation module
//!
//! Procedural terrain generation using Perlin noise.

pub mod terrain;
pub mod biome;
pub mod features;

pub use terrain::*;
pub use biome::*;
pub use features::*;
