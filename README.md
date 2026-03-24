# VoxelCraft

A Minecraft-inspired voxel sandbox game built with Rust and Go.

## Features

- **Voxel World Generation**: Procedurally generated terrain with biomes, caves, and ore veins using Perlin noise
- **Block Physics**: AABB collision detection, player movement with sprint/jump/sneak
- **Multiplayer Networking**: Minecraft-compatible protocol with VarInt encoding
- **Persistent Worlds**: PostgreSQL storage for chunks and player data
- **Matchmaking Service**: Go-based auth service with JWT and lobby support
- **GPU Rendering**: WGPU-based renderer with chunk meshes and texture atlas

## Architecture

```
minecraft-clone/
├── Cargo.toml              # Rust workspace
├── crates/
│   ├── shared/             # Shared types, protocol, world generation
│   ├── client/             # WGPU game client
│   └── server/             # Tokio-based game server
├── matchmaking/            # Go Gin matchmaking service
├── docker/                 # Docker configuration
└── docs/                   # Protocol and architecture docs
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Engine | Rust |
| Client Renderer | WGPU |
| Server | Tokio |
| Database | PostgreSQL 15+ |
| Cache/Sessions | Redis 7+ |
| Matchmaking | Go + Gin |
| Auth | Argon2 + JWT |

## Quick Start

### Prerequisites

- Rust 1.75+
- Go 1.21+
- Docker and Docker Compose
- PostgreSQL 15+ (or use Docker)
- Redis 7+ (or use Docker)

### 1. Clone and Setup

```bash
git clone https://github.com/voxelcraft/minecraft-clone.git
cd minecraft-clone
```

### 2. Start Infrastructure

```bash
docker compose -f docker/docker-compose.yml up -d
```

### 3. Configure Environment

```bash
cp .env.example .env
# Edit .env with your database credentials
```

### 4. Build

```bash
cargo build --workspace
```

### 5. Run

```bash
# Start the game server
cargo run -p voxel-server

# In another terminal, start the matchmaking service
cd matchmaking && go run ./cmd/server
```

## Development

### Building

```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p voxel-server

# Build with optimizations
cargo build --release --workspace
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p voxel-shared

# Run with coverage
cargo tarpaulin --workspace
```

### Linting

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all -- -D warnings
```

### Database Migrations

```bash
# Run migrations (requires DATABASE_URL env var)
sqlx migrate run
```

## Documentation

- [Protocol Documentation](docs/protocol.md)
- [Contributing Guidelines](docs/CONTRIBUTING.md)
- [Architecture Decision Records](docs/adr/)

## License

MIT OR Apache-2.0
