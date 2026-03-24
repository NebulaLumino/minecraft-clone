//! Protocol packet definitions
//!
//! Minecraft-inspired binary protocol with VarInt encoding.

use serde::{Deserialize, Serialize};

use crate::types::{BlockPos, PlayerId};

/// Protocol version for this implementation
pub const PROTOCOL_VERSION: i32 = 1;

/// Maximum chat message length
pub const MAX_CHAT_LENGTH: usize = 256;

/// Maximum command length
pub const MAX_COMMAND_LENGTH: usize = 64;

/// VarInt-packed UUID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolUUID(pub [u8; 16]);

impl ProtocolUUID {
    pub fn new(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

// =============================================================================
// Clientbound Packets (Server -> Client)
// =============================================================================

/// Spawn Player (0x00)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientboundSpawnPlayer {
    pub entity_id: u32,
    pub player_id: PlayerId,
    pub username: String,
    pub position: [f64; 3],
    pub yaw: i8,
    pub pitch: i8,
}

/// Spawn Entity (0x01)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientboundSpawnEntity {
    pub entity_id: u32,
    pub uuid: [u8; 16],
    pub entity_type: u8,
    pub position: [f64; 3],
    pub pitch: i8,
    pub yaw: i8,
    pub head_yaw: i8,
    pub velocity: [i16; 3],
}

/// Chat Message (0x06)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientboundChatMessage {
    pub message: String,
    pub sender: [u8; 16],
    pub timestamp: i64,
    pub signature: Option<[u8; 32]>,
}

/// Chunk Data (0x38)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientboundChunkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: Vec<u8>,
    pub block_entities: Vec<u8>,
}

/// Player Position Look (0x30)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientboundPlayerPositionLook {
    pub position: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub teleport_id: u32,
}

/// Keepalive (Ping) (Clientbound)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientboundKeepAlive {
    pub id: u64,
}

/// Disconnect / Login Failure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientboundDisconnect {
    pub reason: String,
}

// =============================================================================
// Serverbound Packets (Client -> Server)
// =============================================================================

/// Handshake (0x00 on Login state)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundHandshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: u8,
}

/// Login Start (0x00)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundLoginStart {
    pub username: String,
    pub has_sig_data: bool,
    pub sig_data: Option<[u8; 32]>,
    pub salt: Option<u64>,
}

/// Chat Message (0x01)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundChatMessage {
    pub message: String,
    pub timestamp: i64,
    pub salt: u64,
    pub signature: Option<[u8; 32]>,
}

/// Player Position (0x1C)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundPlayerPosition {
    pub position: [f64; 3],
    pub on_ground: bool,
}

/// Player Rotation (0x1B)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

/// Player Position + Rotation (0x1D)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundPlayerPositionRotation {
    pub position: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

/// Use Item On Block (0x1E)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundUseItemOn {
    pub hand: u8,
    pub position: BlockPos,
    pub face: u8,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
}

/// Click Slot (0x08)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundClickSlot {
    pub window_id: u8,
    pub slot_index: i16,
    pub button: i8,
    pub action_type: i16,
    pub clicked_item: Vec<u8>,
}

/// Teleport Confirm (0x00)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundTeleportConfirm {
    pub teleport_id: u32,
}

/// Keepalive (Pong) (Serverbound)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerboundKeepAlive {
    pub id: u64,
}
