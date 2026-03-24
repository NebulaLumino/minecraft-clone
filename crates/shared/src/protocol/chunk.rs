//! Chunk data serialization for network transfer

use bytes::BufMut;
use crate::world::Chunk;
use crate::types::BlockState;

/// Serialize a chunk for network transfer
/// Format:
/// - Chunk position (x, z) as VarInt
/// - Number of sections (i16)
/// - For each section:
///   - Block count (i16)
///   - If non-air blocks exist:
///     - Section y (i8)
///     - Block states (16x16x16 = 4096 u16s as nibble pairs)
///     - Sky light (16x16x16 = 4096 u8s, packed)
///     - Block light (16x16x16 = 4096 u8s, packed)
/// - Heightmap (256 u16s)
/// - Biome data (256 u8s)
pub fn serialize_chunk(chunk: &Chunk) -> Vec<u8> {
    let mut buf = Vec::new();

    // Write chunk position
    write_varint(chunk.position.x, &mut buf);
    write_varint(chunk.position.z, &mut buf);

    // Count non-empty sections
    let non_empty_sections: Vec<(usize, &crate::world::ChunkSection)> = chunk
        .sections
        .iter()
        .enumerate()
        .filter(|(_, s)| s.non_air_count > 0)
        .collect();

    // Write number of sections
    write_varint(non_empty_sections.len() as i32, &mut buf);

    // Write each non-empty section
    for (section_index, section) in non_empty_sections {
        let section_y = section_index as u8;
        buf.push(section_y);

        // Write block states as 4096 u16s
        for block in &section.blocks {
            buf.put_u16_le(block.id);
        }

        // Write sky light (packed nibbles)
        write_packed_nibbles(&section.sky_light, &mut buf);

        // Write block light (packed nibbles)
        write_packed_nibbles(&section.block_light, &mut buf);
    }

    // Write heightmap
    for &h in &chunk.heightmap {
        buf.put_u16_le(h);
    }

    // Write biome data
    for &b in &chunk.biome_data {
        buf.push(b);
    }

    buf
}

/// Write a VarInt to a byte buffer
fn write_varint(mut value: i32, buf: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 {
            break;
        }
    }
}

/// Write packed nibbles (2 per byte)
fn write_packed_nibbles(values: &[u8], buf: &mut Vec<u8>) {
    for chunk in values.chunks(2) {
        let byte = (chunk[0] & 0xF) | ((chunk.get(1).unwrap_or(&0) & 0xF) << 4);
        buf.push(byte);
    }
}

/// Read packed nibbles (2 per byte) back to u8 array
fn read_packed_nibbles(buf: &[u8], count: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(count);
    for &byte in buf.iter().take((count + 1) / 2) {
        result.push(byte & 0xF);
        if result.len() < count {
            result.push((byte >> 4) & 0xF);
        }
    }
    result
}

/// Deserialize chunk data from network format
pub fn deserialize_chunk(data: &[u8], chunk_x: i32, chunk_z: i32) -> Option<Chunk> {
    let mut pos = 0;

    // Read chunk position (we already have x, z)
    let _read_x = read_varint_at(&data, &mut pos)?;
    let _read_z = read_varint_at(&data, &mut pos)?;

    // Read number of sections
    let section_count = read_varint_at(&data, &mut pos)? as usize;

    let mut chunk = Chunk::new(crate::types::ChunkPos::new(chunk_x, chunk_z));

    // Read sections
    for _ in 0..section_count {
        let section_y = data.get(pos).copied()? as i8;
        pos += 1;

        let section_index = section_y as usize;
        if section_index >= chunk.sections.len() {
            continue;
        }

        // Read 4096 block IDs
        for i in 0..4096 {
            let block_id = u16::from_le_bytes([
                *data.get(pos)?,
                *data.get(pos + 1)?,
            ]);
            pos += 2;

            if block_id != 0 {
                chunk.sections[section_index].blocks[i] = BlockState::new(block_id, 0);
                chunk.sections[section_index].non_air_count += 1;
            }
        }

        // Read sky light
        let sky_light = read_packed_nibbles(&data[pos..], 4096);
        pos += (4096 + 1) / 2;
        chunk.sections[section_index].sky_light = sky_light;

        // Read block light
        let block_light = read_packed_nibbles(&data[pos..], 4096);
        pos += (4096 + 1) / 2;
        chunk.sections[section_index].block_light = block_light;

        chunk.sections[section_index].valid = true;
    }

    // Read heightmap
    for i in 0..256 {
        if pos + 2 > data.len() {
            return None;
        }
        chunk.heightmap[i] = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
    }

    // Read biome data
    for i in 0..64 {
        if pos >= data.len() {
            break;
        }
        chunk.biome_data[i] = data[pos];
        pos += 1;
    }

    Some(chunk)
}

/// Read a VarInt at a position and advance
fn read_varint_at(data: &[u8], pos: &mut usize) -> Option<i32> {
    let mut result: i32 = 0;
    let mut shift = 0;

    loop {
        let byte = *data.get(*pos)?;
        *pos += 1;
        result |= ((byte & 0x7F) as i32) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
        if shift >= 32 {
            return None;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChunkPos;

    #[test]
    fn test_chunk_serialization_roundtrip() {
        let chunk = Chunk::new(ChunkPos::new(0, 0));
        let serialized = serialize_chunk(&chunk);
        let deserialized = deserialize_chunk(&serialized, 0, 0);
        assert!(deserialized.is_some());
    }
}
