//! Anvil storage module
//!
//! Minecraft Anvil (.mca) file format for world storage.

use voxel_shared::ChunkPos;
use voxel_shared::world::Chunk;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};

/// Anvil file format storage
pub struct AnvilStorage {
    save_dir: PathBuf,
}

impl AnvilStorage {
    /// Create new storage at path
    pub fn new(save_dir: &Path) -> std::io::Result<Self> {
        let save_dir = save_dir.to_path_buf();
        std::fs::create_dir_all(save_dir.join("chunks"))?;
        std::fs::create_dir_all(save_dir.join("region"))?;
        Ok(Self { save_dir })
    }

    /// Get chunk file path
    fn chunk_path(&self, pos: ChunkPos) -> PathBuf {
        // Store in chunks directory as individual files
        // chunks/X_Y.chunk
        self.save_dir.join("chunks").join(format!("{}_{}.chunk", pos.x, pos.z))
    }

    /// Save a chunk
    pub fn save_chunk(&self, chunk: &Chunk) -> std::io::Result<()> {
        let path = self.chunk_path(chunk.position);
        let data = bincode::serialize(chunk).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        // Write length prefix for easy reading
        let mut file = std::fs::File::create(&path)?;
        file.write_all(&(data.len() as u32).to_le_bytes())?;
        file.write_all(&data)?;
        Ok(())
    }

    /// Load a chunk
    pub fn load_chunk(&self, pos: ChunkPos) -> std::io::Result<Option<Chunk>> {
        let path = self.chunk_path(pos);
        if !path.exists() {
            return Ok(None);
        }
        let mut file = std::fs::File::open(&path)?;
        let mut len_bytes = [0u8; 4];
        file.read_exact(&mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;
        let mut data = vec![0u8; len];
        file.read_exact(&mut data)?;
        let chunk = bincode::deserialize(&data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(Some(chunk))
    }

    /// Check if a chunk exists on disk
    pub fn has_chunk(&self, pos: ChunkPos) -> bool {
        self.chunk_path(pos).exists()
    }

    /// Delete a chunk file
    pub fn delete_chunk(&self, pos: ChunkPos) -> std::io::Result<()> {
        let path = self.chunk_path(pos);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Get save directory
    pub fn save_dir(&self) -> &Path {
        &self.save_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_save_load() {
        let temp_dir = std::env::temp_dir().join("voxel_test_chunks");
        let storage = AnvilStorage::new(&temp_dir).unwrap();

        // Create a test chunk at position (0, 0)
        let chunk_pos = ChunkPos::new(0, 0);
        let chunk = Chunk::new(chunk_pos);

        // Save it
        storage.save_chunk(&chunk).unwrap();

        // Load it back
        let loaded = storage.load_chunk(chunk_pos).unwrap();
        assert!(loaded.is_some());

        // Verify it's the same
        let loaded_chunk = loaded.unwrap();
        assert_eq!(loaded_chunk.position, chunk_pos);

        // Clean up
        storage.delete_chunk(chunk_pos).unwrap();
        std::fs::remove_dir_all(temp_dir).ok();
    }
}
