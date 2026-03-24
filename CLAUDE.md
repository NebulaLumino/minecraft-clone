# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**VoxelCraft** — A Minecraft clone built in Rust. Simple, single-binary design with file-based world storage. No microservices, no databases required.

**Philosophy:** Like the original Minecraft — a single JAR that just works. Iterative development: get it running first, add complexity later.

---

## Architecture

### Repository Structure

```
minecraft-clone/
├── crates/
│   ├── shared/       # Block types, world structures, protocol codec
│   └── voxel/        # Main game binary (single-player + optional server)
├── saves/            # World save files
└── docs/             # Protocol docs
```

### Design

**Single Binary (`voxel`):**
- Single-player by default
- `--server` flag for dedicated server
- `--client` flag for connecting to server
- File-based world storage (Anvil .mca format)

**No external dependencies:**
- No PostgreSQL
- No Redis
- No Docker
- No matchmaking service

---

## Build Commands

```bash
# Build
cargo build

# Run single-player
cargo run

# Run dedicated server
cargo run -- --server

# Connect to server
cargo run -- --client 192.168.1.100:25565

# Tests
cargo test --workspace

# Lint
cargo clippy --workspace

# Format
cargo fmt
```

---

## Key Crates

| Crate | Purpose |
|-------|---------|
| `voxel-shared` | Types, constants, world generation, protocol codec |
| `voxel` | Main binary: rendering, physics, game logic |

---

## Key Files

| File | Purpose |
|------|---------|
| `BLUEPRINT.md` | Architecture specification |
| `crates/shared/src/world/generation/terrain.rs` | Working terrain generation (Perlin noise) |
| `crates/shared/src/world/chunk.rs` | Chunk data structure |
| `crates/shared/src/constants/blocks.rs` | Block type definitions |
| `crates/voxel/src/game/terrain.rs` | World manager for client |

---

## Development Status

The project is in early development:

- [x] **World generation** — Perlin noise terrain (in voxel-shared)
- [x] **Chunk structures** — 16x16x256 chunks with sections
- [x] **Block types** — ~100 block definitions
- [ ] **Rendering** — WGPU integration in progress
- [ ] **Physics** — AABB collision in progress
- [ ] **UI** — HUD, menus to implement

---

## Naming Conventions

- Rust modules/functions: `snake_case`
- Rust structs/enums/traits: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`

---

## How Minecraft Was Built

Markus Persson built Minecraft as a hobby project in May 2009:

1. Single JAR file, no microservices
2. File-based world storage (not databases)
3. Direct TCP multiplayer, no matchmaking
4. Iterative development: blocks → terrain → mobs → inventory

VoxelCraft follows the same approach.
