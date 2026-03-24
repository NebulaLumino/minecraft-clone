//! Player-related types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Player UUID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub Uuid);

impl Default for PlayerId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Player gamemode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl Gamemode {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Survival),
            1 => Some(Self::Creative),
            2 => Some(Self::Adventure),
            3 => Some(Self::Spectator),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

/// Player abilities flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlayerAbilities {
    pub invulnerable: bool,
    pub flying: bool,
    pub may_fly: bool,
    pub instabuild: bool,
    pub may_edit: bool,
}

impl PlayerAbilities {
    pub fn creative() -> Self {
        Self {
            invulnerable: true,
            flying: true,
            may_fly: true,
            instabuild: true,
            may_edit: true,
        }
    }

    pub fn survival() -> Self {
        Self {
            invulnerable: false,
            flying: false,
            may_fly: false,
            instabuild: false,
            may_edit: false,
        }
    }
}

/// Player permissions (for server operators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PermissionLevel {
    Normal = 0,
    Moderator = 1,
    Admin = 2,
    Owner = 3,
}
