//! Game module
//!
//! Core game state and logic.

mod terrain;
mod player;
mod inventory;

pub use terrain::Terrain;
pub use player::{Player, BlockTarget};
pub use inventory::Inventory;
use voxel_shared::{BlockPos, BlockState};
use crate::storage::{AnvilStorage, PlayerData};

/// Core game state
pub struct Game {
    pub world: Terrain,
    pub player: Player,
    pub inventory: Inventory,
    save_path: Option<std::path::PathBuf>,
}

impl Game {
    /// Create new game
    pub fn new() -> Self {
        Self {
            world: Terrain::new(12345), // Default seed
            player: Player::new(),
            inventory: Inventory::new(),
            save_path: None,
        }
    }

    /// Create new game with save path
    pub fn with_save_path(save_path: std::path::PathBuf) -> Self {
        let storage = AnvilStorage::new(&save_path).ok();
        let seed = 12345; // Default seed

        let mut game = Self {
            world: Terrain::new(seed),
            player: Player::new(),
            inventory: Inventory::new(),
            save_path: Some(save_path.clone()),
        };

        // Try to load player data if save exists
        if let Some(ref storage) = storage {
            if let Ok(Some(player_data)) = Self::load_player_data_internal(storage) {
                game.player.position = player_data.position;
                game.player.velocity = player_data.velocity;
                game.player.rotation = player_data.rotation;
                game.player.on_ground = player_data.on_ground;
                game.player.sprinting = player_data.sprinting;
                game.player.sneaking = player_data.sneaking;
                tracing::info!("Loaded player data from save");
            }
        }

        game
    }

    /// Update game state
    pub fn update(&mut self, delta: f32) {
        self.player.update(delta, &self.world);
    }

    /// Break block that player is looking at
    /// Returns the block state of the broken block, or None if no block targeted
    pub fn break_targeted_block(&mut self) -> Option<BlockState> {
        if let Some(target) = self.player.get_targeted_block(&self.world, 5.0) {
            let block = self.world.get_block(target.position);
            if block.id != 0 {
                // Set block to air (0)
                self.world.set_block(target.position, BlockState::AIR);
                tracing::info!("Broke block at {:?}", target.position);
                return Some(block);
            }
        }
        None
    }

    /// Place block on the face of the targeted block
    /// Returns true if block was placed, false if no block targeted or no space
    pub fn place_block(&mut self, block_state: BlockState) -> bool {
        if let Some(target) = self.player.get_targeted_block(&self.world, 5.0) {
            // Calculate position to place block (adjacent to targeted face)
            let place_pos = match target.face {
                0 => BlockPos::new(target.position.x + 1, target.position.y, target.position.z),     // +X
                1 => BlockPos::new(target.position.x - 1, target.position.y, target.position.z),     // -X
                2 => BlockPos::new(target.position.x, target.position.y + 1, target.position.z),   // +Y
                3 => BlockPos::new(target.position.x, target.position.y - 1, target.position.z),   // -Y
                4 => BlockPos::new(target.position.x, target.position.y, target.position.z + 1),   // +Z
                5 => BlockPos::new(target.position.x, target.position.y, target.position.z - 1),   // -Z
                _ => return false,
            };

            // Check if place position is air
            if self.world.get_block(place_pos).id == 0 {
                self.world.set_block(place_pos, block_state);
                tracing::info!("Placed block at {:?}", place_pos);
                return true;
            }
        }
        false
    }

    /// Get current block target
    pub fn get_target(&self) -> Option<BlockTarget> {
        self.player.get_targeted_block(&self.world, 5.0)
    }

    /// Save game state to file
    pub fn save(&self) {
        // Save player data
        if let Some(ref save_path) = self.save_path {
            let storage = match AnvilStorage::new(save_path) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to create storage: {}", e);
                    return;
                }
            };

            let player_data = PlayerData::from_player(
                self.player.position,
                self.player.velocity,
                self.player.rotation,
                self.player.on_ground,
                self.player.sprinting,
                self.player.sneaking,
            );

            if let Err(e) = Self::save_player_data_internal(&storage, &player_data) {
                tracing::error!("Failed to save player data: {}", e);
                return;
            }

            // Save all loaded chunks
            for (chunk_pos, chunk) in self.world.chunks().iter() {
                if let Err(e) = storage.save_chunk(chunk) {
                    tracing::error!("Failed to save chunk {:?}: {}", chunk_pos, e);
                }
            }

            tracing::info!("Game saved to {:?}", save_path);
        } else {
            tracing::info!("No save path set, game not saved");
        }
    }

    fn save_player_data_internal(storage: &AnvilStorage, player_data: &PlayerData) -> std::io::Result<()> {
        let path = storage.save_dir().join("player.dat");
        let data = bincode::serialize(player_data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, data)?;
        Ok(())
    }

    fn load_player_data_internal(storage: &AnvilStorage) -> std::io::Result<Option<PlayerData>> {
        let path = storage.save_dir().join("player.dat");
        if !path.exists() {
            return Ok(None);
        }
        let data = std::fs::read(path)?;
        let player_data = bincode::deserialize(&data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(player_data))
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
