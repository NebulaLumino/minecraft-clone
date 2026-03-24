//! Storage module
//!
//! File-based world persistence (Anvil format).

mod anvil;
mod player;

pub use anvil::AnvilStorage;
pub use player::PlayerData;
