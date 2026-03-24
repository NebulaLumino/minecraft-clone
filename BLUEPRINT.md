# VoxelCraft — Architectural Blueprint

**Version:** 2.0.0
**Last Updated:** 2026-03-22
**Status:** Simplified — Minecraft-style development

---

## 1. Project Overview

**VoxelCraft** — A Minecraft clone built the right way: single binary, file-based storage, iterative development.

**Core Philosophy:** Like the original Minecraft, start simple. Get it running first, add complexity later. No microservices, no databases, no Docker.

---

## 2. Architecture

### Single Binary Design

```
voxel/
├── src/
│   ├── main.rs              # Entry: winit + WGPU init, game loop
│   ├── engine/              # Rendering, physics, audio
│   │   ├── renderer/        # WGPU pipeline
│   │   ├── camera/         # FPS camera
│   │   ├── physics/        # Collision, movement, raycast
│   │   └── world/          # Chunk management
│   ├── game/               # Game logic
│   │   ├── terrain.rs      # World generation
│   │   ├── player.rs       # Player controller
│   │   ├── inventory.rs    # Hotbar
│   │   └── block.rs        # Block interactions
│   ├── ui/                 # HUD, menus
│   └── storage/            # File-based persistence
│       └── anvil.rs        # Anvil .mca format
└── saves/                  # World saves
```

**Three Modes:**
```bash
cargo run                    # Single-player (default)
cargo run -- --server       # Dedicated server on :25565
cargo run -- --client IP    # Connect to server
```

### How Minecraft Was Built (May 2009)

Markus "Notch" Persson built Minecraft as a weekend hobby project:

1. **Single JAR file** - Everything in one executable
2. **File-based storage** - Worlds saved as files, not databases
3. **No microservices** - No PostgreSQL, Redis, or matchmaking
4. **Iterative development** - Blocks first, then terrain, then mobs, then inventory
5. **Direct TCP multiplayer** - No auth server, just LAN play

**VoxelCraft follows this same path.**

---

## 3. Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Rendering** | WGPU + winit | GPU rendering, window/input |
| **Physics** | Custom AABB | Voxel collision, gravity |
| **World Gen** | Perlin noise | Procedural terrain |
| **Math** | glam | Vec3, Mat4 SIMD |
| **Serialization** | bincode | Chunk/entity serialization |
| **Audio** | rodio | Spatial sound |

**No external dependencies for single-player.**

---

## 4. World Storage

### Anvil Format (like Minecraft)

```
saves/<world_name>/
├── world.dat      # World metadata (seed, time, game rules)
├── player.dat     # Player position, inventory, health
└── region/
    ├── r.0.0.mca  # Region file (32x32 chunks)
    ├── r.1.0.mca
    └── ...
```

### Region File Format (.mca)

```
[Header: 4096 bytes]
  Location table: 32x32 × 4 bytes (offset + size)
  Timestamps: 32x32 × 4 bytes

[Chunks]
  Each chunk: zlib-compressed NBT
    - Block states (palette + packed indices)
    - Biome data
    - Heightmap
```

**Why not PostgreSQL?**
- Minecraft itself uses files — proven at scale
- Copy world folder to share
- No database server to run
- Works offline

---

## 5. World Generation

### Terrain Algorithm

1. **Heightmap** — Multi-octave Perlin noise for base terrain
2. **Biomes** — Temperature/humidity noise blend (plains, desert, taiga, etc.)
3. **Caves** — 3D Perlin carve-through
4. **Ores** — Blob noise for coal, iron, gold, diamond
5. **Trees** — Random placement with spacing rules

### Block Types

Defined in `src/constants/blocks.rs`:
- Air (0), Stone (1), Grass (2), Dirt (3)
- Sand (4), Water (5), Wood (6), Leaves (7)
- And ~100 more...

---

## 6. Rendering Pipeline

### WGPU Setup

1. **Device/Queue** — Request adapter, create device
2. **Texture Atlas** — 16x16 block textures in single texture
3. **Render Pipeline** — Vertex buffer + index buffer per chunk
4. **Depth Buffer** — 24-bit depth for 3D

### Chunk Mesh Generation

For each 16x16x256 chunk section:
1. Iterate all blocks
2. For solid blocks, add face if neighbor is air
3. Apply texture UVs from atlas
4. Calculate ambient occlusion
5. Upload to GPU as vertex/index buffers

### Camera

- First-person, mouse look (pitch clamped ±89°)
- WASD movement relative to look direction
- FOV: 70°, Render distance: 8 chunks

---

## 7. Physics

### AABB Collision

Player hitbox: 0.6 × 1.8 × 0.6 blocks

**On move request:**
1. Expand hitbox by movement vector
2. Check all blocks overlapping expanded AABB
3. Resolve by axis (X, Y, Z)
4. Apply gravity (0.08 blocks/tick²)

### Raycast

For block targeting:
1. Step along look direction (0.01 block increments)
2. Find first non-air block
3. Return block position + hit face

---

## 8. Multiplayer (Phase 2)

### Direct TCP

```
voxel --server     # Listen on 25565
voxel --client IP  # Connect to server
```

### Protocol

- **VarInt encoding** for packet IDs and most fields
- **Big-endian** for fixed multi-byte values
- **Zlib compression** for chunk data
- **Packet structure:** `[PacketID] [Payload...]`

### Core Packets

| ID | Name | Direction |
|----|------|-----------|
| 0x00 | Handshake | C→S |
| 0x01 | LoginStart | C→S |
| 0x02 | LoginSuccess | S→C |
| 0x03 | ChatMessage | Both |
| 0x04 | ChunkData | S→C |
| 0x05 | BlockUpdate | Both |
| 0x06 | PlayerPosition | C→S |
| 0x07 | PlayerLook | C→S |
| 0x08 | SpawnPlayer | S→C |

*Full protocol in `docs/protocol.md`*

---

## 9. File Structure

```
voxelcraft/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── renderer/
│   │   ├── camera/
│   │   ├── physics/
│   │   └── world/
│   ├── game/
│   │   ├── mod.rs
│   │   ├── terrain.rs
│   │   ├── player.rs
│   │   └── inventory.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── hud.rs
│   │   └── menu.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   └── anvil.rs
│   └── network/
│       ├── mod.rs
│       ├── client.rs
│       └── server.rs
├── assets/
│   └── textures/
└── saves/
```

---

## 10. Development Phases

### Phase 1: Core Rendering ✓
- [x] WGPU + winit setup
- [ ] Render textured blocks
- [ ] First-person camera
- [ ] Chunk mesh generation

### Phase 2: World Integration
- [ ] Connect terrain generation
- [ ] Chunk loading radius
- [ ] Player collision
- [ ] Block break/place

### Phase 3: Persistence
- [ ] Anvil file format
- [ ] Save/load chunks
- [ ] Save/load player
- [ ] World select menu

### Phase 4: Polish
- [ ] Day/night cycle
- [ ] Basic inventory UI
- [ ] Sound effects
- [ ] Debug screen (F3)

### Phase 5: Multiplayer
- [ ] TCP server
- [ ] TCP client
- [ ] Player sync
- [ ] Chunk sync

---

## 11. Build Commands

```bash
# Build and run single-player
cargo build
cargo run

# Run dedicated server
cargo run -- --server

# Connect to server
cargo run -- --client 192.168.1.100:25565

# Tests
cargo test
```

---

## 12. Why This Works

Original Minecraft worked because:
1. **No dependencies** — Download, run, play
2. **Iterative** — Add one feature at a time
3. **Simple saves** — File copy = world backup

This architecture follows the same principles.

---

*End of BLUEPRINT.md*
