# ADR-004: Minecraft-Compatible Network Protocol

## Status
Accepted

## Context
We need a network protocol that supports both vanilla Minecraft clients (for compatibility) and our own custom features.

## Decision
Implement a **Minecraft-inspired binary protocol** with VarInt encoding, following the same connection flow.

### Protocol States
1. **Handshake** (0x00): Protocol version, server address, next state
2. **Login** (0x01): Authentication, player creation
3. **Play** (0x02): Gameplay

### Packet Format
```
[VarInt: Packet Length][VarInt: Packet ID][Packet Data]
```

### Compression
- Optional zlib compression after login
- Threshold configurable (typically 256 bytes)
- Data follows format: `[VarInt: uncompressed length][compressed data]`

### Encryption
- TLS encryption optional, negotiated in handshake
- Vanilla Minecraft uses offline mode (no encryption)

## Packet Categories

### Spawn Packets
- `SpawnPlayer` (0x00) - Create player entity
- `SpawnEntity` (0x01) - Generic entity (item, boat, etc.)
- `SpawnLivingEntity` (0x02) - Mob with AI
- `SpawnPainting` (0x03) - Decorative

### Movement Packets
- `PlayerPosition` (0x1C) - Position only
- `PlayerRotation` (0x1B) - Rotation only
- `PlayerPositionRotation` (0x1D) - Combined
- `EntityPosition` (0x26) - Delta position for other entities
- `EntityPositionRotation` (0x28) - Full delta for entities

### Gameplay Packets
- `KeepAlive` (0x1F/0x0F) - Heartbeat
- `ChatMessage` (0x01/0x06) - Chat
- `PlayerDigging` (0x09) - Block break
- `PlayerBlockPlacement` (0x1E) - Block place

## Consequences

### Positive
- Vanilla Minecraft client compatibility (with protocol version matching)
- Compact packets (VarInt saves bandwidth)
- Well-documented protocol (multiple resources available)
- Extensible: custom packets can use reserved packet IDs

### Negative
- Complex protocol state machine
- Must keep up with Minecraft protocol updates for compatibility
- VarInt encoding/decoding overhead (minimal)

## References
- [Protocol Documentation](https://wiki.vg/Protocol)
- [Protocol Numbers](https://gist.github.com/barthack/429b4f42c53e2accd15f26d668b1c7f7)
