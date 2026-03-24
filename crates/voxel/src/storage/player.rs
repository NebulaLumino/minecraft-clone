//! Player data module
//!
//! Serializable player data for saving/loading.

use serde::{Deserialize, Serialize};
use glam::Vec3;

/// Player data for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Vec3, // yaw, pitch, roll
    pub on_ground: bool,
    pub sprinting: bool,
    pub sneaking: bool,
    pub health: f32,
    pub food: i32,
}

impl PlayerData {
    /// Create new player data from current player state
    pub fn from_player(position: Vec3, velocity: Vec3, rotation: Vec3, on_ground: bool, sprinting: bool, sneaking: bool) -> Self {
        Self {
            position,
            velocity,
            rotation,
            on_ground,
            sprinting,
            sneaking,
            health: 20.0,  // Default health
            food: 20,     // Default food
        }
    }
}

/// World metadata for save files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMetadata {
    pub name: String,
    pub seed: u64,
    pub created_at: u64,
    pub last_played: u64,
    pub game_time: i64,
    pub day_time: i64,
}

impl Default for WorldMetadata {
    fn default() -> Self {
        Self {
            name: "Untitled World".to_string(),
            seed: 12345,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            last_played: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            game_time: 0,
            day_time: 6000, // Noon in Minecraft ticks
        }
    }
}
