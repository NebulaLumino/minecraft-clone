//! Protocol connection states

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play,
}

impl ConnectionState {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Handshake),
            1 => Some(Self::Status),
            2 => Some(Self::Login),
            3 => Some(Self::Play),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Handshake => 0,
            Self::Status => 1,
            Self::Login => 2,
            Self::Play => 3,
        }
    }
}

/// Next state after handshake
pub const STATE_STATUS: u8 = 1;
pub const STATE_LOGIN: u8 = 2;
pub const STATE_PLAY: u8 = 3;
