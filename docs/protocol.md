# VoxelCraft Protocol Documentation

## Overview

VoxelCraft uses a Minecraft-inspired binary protocol with VarInt encoding for length prefixes and packet IDs.

## Connection Flow

1. **Handshake** (0x00): Client connects and sends handshake packet with protocol version
2. **Login** (0x01): Client sends login start, server verifies and sends login success
3. **Play** (0x02): Gameplay packets exchanged

## Packet Format

All packets follow this format:

```
[VarInt: Packet Length][VarInt: Packet ID][Packet Data]
```

## VarInt Encoding

VarInts are used for all integer types. They use 7 bits per byte for data, with the MSB indicating continuation.

- Single byte: 0-127
- Two bytes: 128-16383
- Three bytes: 16384-2097151
- Four bytes: 2097152-268435455
- Five bytes: -2147483648 to 2147483647

## Packet IDs

### Handshake (State: Handshake)
| Packet | ID | Description |
|--------|-----|-------------|
| Serverbound Handshake | 0x00 | Protocol version, server address, next state |

### Login (State: Login)
| Packet | ID | Description |
|--------|-----|-------------|
| Serverbound Login Start | 0x00 | Username, has sig data, sig data, salt |
| Clientbound Login Success | 0x02 | UUID, username |
| Clientbound Disconnect | 0x00 | Reason (chat component) |

### Play (State: Play)

#### Serverbound Packets
| Packet | ID | Description |
|--------|-----|-------------|
| KeepAlive | 0x0F | Ping response |
| ChatMessage | 0x01 | Message, timestamp, salt, signature |
| PlayerPosition | 0x1C | Position, on ground |
| PlayerRotation | 0x1B | Yaw, pitch, on ground |
| PlayerPositionRotation | 0x1D | Position, yaw, pitch, on ground |
| PlayerDigging | 0x09 | Status, location, face |
| PlayerBlockPlacement | 0x1E | Hand, location, face, cursor, inside |
| UseItemOn | 0x1F | Hand, location, face, cursor, inside |
| TeleportConfirm | 0x00 | Teleport ID |
| ClickSlot | 0x08 | Window ID, slot, button, action, clicked item |
| HeldItemChange | 0x25 | Slot |
| CreativeInventoryAction | 0x26 | Slot, clicked item |
| ClientStatus | 0x05 | Action ID |

#### Clientbound Packets
| Packet | ID | Description |
|--------|-----|-------------|
| SpawnPlayer | 0x00 | Entity ID, player UUID, username, position, yaw, pitch |
| SpawnEntity | 0x01 | Entity ID, UUID, type, position, pitch, yaw, head yaw, velocity |
| SpawnLivingEntity | 0x02 | Entity ID, UUID, type, position, yaw, pitch, head yaw, velocity |
| SpawnPainting | 0x03 | Entity ID, UUID, motive, location, direction |
| EntityPosition | 0x26 | Entity ID, delta X, delta Y, delta Z, on ground |
| EntityRotation | 0x27 | Entity ID, yaw, pitch, on ground |
| EntityPositionRotation | 0x28 | Entity ID, delta X, delta Y, delta Z, yaw, pitch, on ground |
| EntityVelocity | 0x29 | Entity ID, velocity X, Y, Z |
| EntityMetadata | 0x30 | Entity ID, metadata |
| EntityEffect | 0x31 | Entity ID, effect ID, amplifier, duration, flags |
| RemoveEntityEffect | 0x32 | Entity ID, effect ID |
| ChatMessage | 0x06 | Message, sender UUID, timestamp, signature |
| PlayerPositionLook | 0x30 | Position, yaw, pitch, teleport ID |
| ChunkData | 0x38 | Chunk X, Z, data, block entities |
| ChunkUpdate | 0x39 | Chunk X, Z, sections |
| BlockChange | 0x0B | Location, block ID |
| KeepAlive | 0x1F | ID |
| Disconnect | 0x19 | Reason |
| PlayerHealth | 0x36 | Health, food, saturation |
| Respawn | 0x38 | World, gamemode, dimension, difficulty, level type, data |
| PlayerListItem | 0x34 | Action, entries |
| PlayerAbilities | 0x31 | Flags, fly speed, walk speed |
| Title | 0x5F | Action, title, subtitle, action bar, fade in, stay, fade out |
| ServerDifficulty | 0x0E | Difficulty |
| CombatEvent | 0x40 | Event, duration, entity ID, message |
| PlayerInfo | 0x36 | Action, entries |

## Data Types

| Type | Size | Description |
|------|------|-------------|
| VarInt | 1-5 bytes | Variable-length integer |
| VarLong | 1-10 bytes | Variable-length 64-bit integer |
| UUID | 16 bytes | 128-bit UUID (big-endian) |
| String | VarInt + bytes | UTF-8 string with length prefix |
| Chat | VarInt + bytes | JSON chat component |
| Position | 8 bytes | X (26 bits), Y (12 bits), Z (26 bits) as signed integers |

## Compression

Optional zlib compression can be enabled:

1. Client sends `ServerboundHandshake` with `next_state = 2`
2. Server sends `ClientboundLoginSuccess`
3. Client sends `ClientboundFinishConfiguration` with compression threshold
4. All subsequent packets are compressed if packet data > threshold

## Encryption

TLS encryption can be enabled via the handshake `enable_encryption` field.

## Heartbeats

- Client sends `KeepAlive` every 30 seconds
- Server responds with `KeepAlive` pong
- If no response within 60 seconds, connection is terminated

## Example Packet Flow

### Login Flow
```
Client -> Server: Handshake (protocol_version=47, next_state=2)
Client -> Server: LoginStart (username="Steve")
Server -> Client: LoginSuccess (uuid=<uuid>, username="Steve")
```

### Game Flow
```
Client -> Server: KeepAlive (id=12345)
Client -> Server: PlayerPosition (x=0.0, y=80.0, z=0.0, on_ground=true)
Server -> Client: ChunkData (chunk_x=0, chunk_z=0, data=<compressed>)
Server -> Client: PlayerPositionLook (x=0.0, y=80.0, z=0.0, yaw=0.0, pitch=0.0)
```

## Error Handling

Disconnect packets contain a reason field:
```json
{
    "text": "Connection refused: invalid protocol version"
}
```

Common disconnect reasons:
- `"Outdated server!"` - Server version too old
- `"Outdated client!"` - Client version too new
- `"Invalid session!"` - Auth token invalid
- `"Kicked by an operator!"` - Admin kick
