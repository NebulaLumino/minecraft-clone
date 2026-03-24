# ADR-003: Chunk-Based World Storage

## Status
Accepted

## Context
Minecraft-style worlds contain trillions of voxels. We need a system that enables efficient storage, generation, and retrieval.

## Decision
Use a **chunk-based world system** with the following properties:

### Chunk Structure
- **Chunk size**: 16x256x16 blocks (configurable)
- **Chunks stored in hash map**: `HashMap<(x, z) -> Chunk>`
- **Chunk sections**: 16x16x16 sub-chunks for efficient updates
- **World height**: 256 blocks (fixed, Minecraft compatible)

### Chunk Data Layout
```
Chunk {
    sections: [Section; 16],        // Y-level sections
    block_entities: Vec<BlockEntity>, // Sign, chest, etc.
    biome_data: [u8; 256],          // 4-bit per block for biome
    heightmap: [u16; 256],          // Surface height for rendering
}
```

### Generation Pipeline
1. **Biome selection** via 2D noise (temperature, humidity)
2. **Heightmap** via 2D terrain noise
3. **Block fill** based on height and biome
4. **Cave carving** via 3D noise (threshold)
5. **Ore veins** via 3D noise with rarity
6. **Tree/feature placement** via populated noise

## Consequences

### Positive
- Memory-efficient: only loaded chunks in memory
- Disk-efficient: chunks serialize compactly
- Generation parallelizable: independent chunk generation
- LOD-friendly: distance-based chunk resolution possible

### Negative
- Chunk boundary artifacts possible
- 3D noise is memory-intensive
- World edits require chunk re-serialization

## References
- [Minecraft Wiki: Chunk format](https://minecraft.fandom.com/wiki/Chunk_format)
- [Minecraft Wiki: Terrain generation](https://minecraft.fandom.com/wiki/Terrain_generation)
