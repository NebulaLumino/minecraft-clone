//! HUD module
//!
//! Heads-up display: health, hotbar, debug info.

use crate::game::Game;

/// HUD overlay
pub struct Hud {
    pub show_debug: bool,
}

impl Hud {
    /// Create new HUD
    pub fn new() -> Self {
        Self {
            show_debug: true, // Default to showing debug info (F3 in Minecraft)
        }
    }

    /// Toggle debug screen
    pub fn toggle_debug(&mut self) {
        self.show_debug = !self.show_debug;
    }

    /// Update HUD state
    pub fn update(&mut self, _game: &Game) {
        // HUD updates happen every frame - could track stats here
    }

    /// Get debug info string for game state
    pub fn get_debug_info(&self, game: &Game) -> String {
        let player = &game.player;
        let pos = player.position();
        let chunk_x = pos.x as i32 / 16;
        let chunk_z = pos.z as i32 / 16;
        let local_x = ((pos.x as i32 % 16) + 16) % 16;
        let local_z = ((pos.z as i32 % 16) + 16) % 16;

        let target_info = if let Some(target) = player.get_targeted_block(&game.world, 5.0) {
            format!("{}, {}, {}", target.position.x, target.position.y, target.position.z)
        } else {
            "none".to_string()
        };

        format!(
            "VoxelCraft Debug (F3 to hide)\n\
             ========================\n\
             Player: ({:.2}, {:.2}, {:.2})\n\
             Chunk: {}, {}\n\
             Local: {}, {}\n\
             Velocity: {:.2}, {:.2}, {:.2}\n\
             On Ground: {}\n\
             Sprinting: {}\n\
             Sneaking: {}\n\
             Rotation: {:.1}, {:.1}\n\
             Terrain Height: {}\n\
             Loaded Chunks: {}\n\
             Target Block: {}\n\
             Seed: {}",
            pos.x, pos.y, pos.z,
            chunk_x, chunk_z,
            local_x, local_z,
            player.velocity.x, player.velocity.y, player.velocity.z,
            player.on_ground,
            player.sprinting,
            player.sneaking,
            player.rotation.x.to_degrees(),
            player.rotation.y.to_degrees(),
            game.world.get_height_at(pos.x as i32, pos.z as i32),
            game.world.loaded_chunks().len(),
            target_info,
            game.world.seed(),
        )
    }
}

impl Default for Hud {
    fn default() -> Self {
        Self::new()
    }
}
