//! VarInt codec and data type encoders

use std::io::Write;

use bytes::Buf;
use bincode;

/// VarInt maximum bytes
pub const VARINT_MAX_BYTES: usize = 5;

/// Packet ID constants for clientbound packets
pub mod clientbound {
    

    pub const SPAWN_PLAYER: i32 = 0x00;
    pub const SPAWN_ENTITY: i32 = 0x01;
    pub const CHAT_MESSAGE: i32 = 0x06;
    pub const PLAYER_POSITION_LOOK: i32 = 0x30;
    pub const CHUNK_DATA: i32 = 0x38;
    pub const KEEPALIVE: i32 = 0x1F;
    pub const DISCONNECT: i32 = 0x19;
}

/// Packet ID constants for serverbound packets
pub mod serverbound {
    pub const HANDSHAKE: i32 = 0x00;
    pub const LOGIN_START: i32 = 0x00;
    pub const CHAT_MESSAGE: i32 = 0x01;
    pub const PLAYER_POSITION: i32 = 0x1C;
    pub const PLAYER_ROTATION: i32 = 0x1B;
    pub const PLAYER_POSITION_ROTATION: i32 = 0x1D;
    pub const USE_ITEM_ON: i32 = 0x1E;
    pub const CLICK_SLOT: i32 = 0x08;
    pub const TELEPORT_CONFIRM: i32 = 0x00;
    pub const KEEPALIVE: i32 = 0x0F;
}

/// VarInt error type
#[derive(Debug, thiserror::Error)]
pub enum VarIntError {
    #[error("VarInt overflow (more than {VARINT_MAX_BYTES} bytes)")]
    Overflow,
    #[error("Unexpected EOF while reading VarInt")]
    UnexpectedEof,
    #[error("Bincode serialization error: {0}")]
    SerializationError(String),
    #[error("Packet read error: {0}")]
    PacketError(String),
}

/// Encode a VarInt
pub fn encode_varint<W: Write>(mut value: i32, writer: &mut W) -> Result<(), VarIntError> {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        writer.write_all(&[byte]).map_err(|_| VarIntError::Overflow)?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

/// Decode a VarInt
pub fn decode_varint<B: Buf>(reader: &mut B) -> Result<i32, VarIntError> {
    let mut result: i32 = 0;
    let mut shift = 0;

    loop {
        if reader.remaining() < 1 {
            return Err(VarIntError::UnexpectedEof);
        }
        let byte = reader.get_u8();
        result |= ((byte & 0x7F) as i32) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 32 {
            return Err(VarIntError::Overflow);
        }
    }

    Ok(result)
}

/// Encode a 64-bit VarInt
pub fn encode_varint_64<W: Write>(mut value: i64, writer: &mut W) -> Result<(), VarIntError> {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        writer.write_all(&[byte]).map_err(|_| VarIntError::Overflow)?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

/// Decode a 64-bit VarInt
pub fn decode_varint_64<B: Buf>(reader: &mut B) -> Result<i64, VarIntError> {
    let mut result: i64 = 0;
    let mut shift = 0;

    loop {
        if reader.remaining() < 1 {
            return Err(VarIntError::UnexpectedEof);
        }
        let byte = reader.get_u8();
        result |= ((byte & 0x7F) as i64) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err(VarIntError::Overflow);
        }
    }

    Ok(result)
}

/// Read a UUID (16 bytes big-endian)
pub fn read_uuid<B: Buf>(reader: &mut B) -> Result<[u8; 16], VarIntError> {
    let mut uuid = [0u8; 16];
    reader.copy_to_slice(&mut uuid);
    Ok(uuid)
}

/// Write a UUID (16 bytes big-endian)
pub fn write_uuid<W: Write>(uuid: &[u8; 16], writer: &mut W) -> Result<(), VarIntError> {
    writer.write_all(uuid).map_err(|_| VarIntError::Overflow)?;
    Ok(())
}

/// Read a fixed-size string (length prefix as VarInt)
pub fn read_string<B: Buf>(reader: &mut B, max_len: usize) -> Result<String, VarIntError> {
    let len = decode_varint(reader)? as usize;
    if len > max_len {
        return Ok(String::new());
    }
    let mut buf = vec![0u8; len];
    reader.copy_to_slice(&mut buf);
    String::from_utf8(buf).map_err(|_| VarIntError::Overflow)
}

/// Write a fixed-size string (length prefix as VarInt)
pub fn write_string<W: Write>(s: &str, max_len: usize, writer: &mut W) -> Result<(), VarIntError> {
    let bytes = s.as_bytes();
    if bytes.len() > max_len {
        return Err(VarIntError::Overflow);
    }
    encode_varint(bytes.len() as i32, writer)?;
    writer.write_all(bytes).map_err(|_| VarIntError::Overflow)?;
    Ok(())
}

/// Write any serializable packet to bytes
pub fn write_packet<P: serde::Serialize>(packet: &P) -> Result<Vec<u8>, VarIntError> {
    let mut buf = Vec::new();
    let data = bincode::serialize(packet)
        .map_err(|e| VarIntError::SerializationError(e.to_string()))?;

    // Write packet length (VarInt)
    encode_varint(data.len() as i32, &mut buf)
        .map_err(|_| VarIntError::Overflow)?;

    // Write packet data
    buf.extend_from_slice(&data);

    Ok(buf)
}

/// Read any deserializable packet from buffer
pub fn read_packet<P: serde::de::DeserializeOwned, B: Buf>(
    packet_type: &str,
    reader: &mut B,
) -> Result<P, VarIntError> {
    // Read packet data length
    let len = decode_varint(reader)
        .map_err(|_| VarIntError::PacketError(format!("failed to read {} packet length", packet_type)))?;

    if reader.remaining() < len as usize {
        return Err(VarIntError::UnexpectedEof);
    }

    // Read packet data
    let mut data = vec![0u8; len as usize];
    reader.copy_to_slice(&mut data);

    // Deserialize
    bincode::deserialize(&data)
        .map_err(|e| VarIntError::PacketError(format!("failed to deserialize {}: {}", packet_type, e)))
}

/// PacketWriter for streaming packet writes
pub struct PacketWriter {
    buf: Vec<u8>,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Write packet ID and serialized packet data
    pub fn write_packet<P: serde::Serialize>(&mut self, packet_id: i32, packet: &P) -> Result<(), VarIntError> {
        // Reserve space for length (will fill in later)
        let len_pos = self.buf.len();
        self.buf.extend_from_slice(&[0, 0, 0, 0, 0]); // Max VarInt bytes

        // Write packet ID
        encode_varint(packet_id, &mut self.buf)?;

        // Serialize packet data
        let data = bincode::serialize(packet)
            .map_err(|e| VarIntError::SerializationError(e.to_string()))?;

        // Write data
        let _data_len = self.buf.len();
        self.buf.extend_from_slice(&data);

        // Calculate and write length
        let packet_len = (self.buf.len() - len_pos - 5) as i32; // Exclude the length prefix itself
        let _len_bytes = [0u8; 5];
        let mut temp = Vec::new();
        encode_varint(packet_len, &mut temp).unwrap();
        let _encoded_len = temp.len();

        // Replace placeholder with actual length encoding
        for (i, &b) in temp.iter().enumerate() {
            self.buf[len_pos + i] = b;
        }

        Ok(())
    }

    /// Get the written bytes
    pub fn take(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.buf)
    }
}

impl Default for PacketWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// PacketReader for streaming packet reads
pub struct PacketReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> PacketReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { buf: data, pos: 0 }
    }

    /// Read a VarInt from the internal buffer
    fn read_varint(&mut self) -> Result<i32, VarIntError> {
        let mut result: i32 = 0;
        let mut shift = 0;

        loop {
            if self.pos >= self.buf.len() {
                return Err(VarIntError::UnexpectedEof);
            }
            let byte = self.buf[self.pos];
            self.pos += 1;
            result |= ((byte & 0x7F) as i32) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
            if shift >= 32 {
                return Err(VarIntError::Overflow);
            }
        }

        Ok(result)
    }

    /// Read packet ID and return remaining data for deserialization
    pub fn read_packet_header(&mut self) -> Result<(i32, usize), VarIntError> {
        // Read packet length
        let _packet_len = self.read_varint()?;

        let start = self.pos;

        // Read packet ID
        let packet_id = self.read_varint()?;

        Ok((packet_id, start))
    }

    /// Get remaining bytes in the packet
    pub fn remaining_packet_data(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// Read remaining data as specified type
    pub fn read_remaining<P: serde::de::DeserializeOwned>(&self) -> Result<P, VarIntError> {
        let data = &self.buf[self.pos..];
        bincode::deserialize(data)
            .map_err(|e| VarIntError::PacketError(format!("failed to deserialize packet: {}", e)))
    }
}

/// Read a specific packet type from reader buffer
pub fn read_packet_type<P: serde::de::DeserializeOwned, B: Buf>(
    packet_id: i32,
    expected_id: i32,
    packet_type: &str,
    reader: &mut B,
) -> Result<P, VarIntError> {
    if packet_id != expected_id {
        return Err(VarIntError::PacketError(format!(
            "expected packet id {} for {}, got {}",
            expected_id, packet_type, packet_id
        )));
    }

    read_packet(packet_type, reader)
}
