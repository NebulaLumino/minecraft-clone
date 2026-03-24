# ADR-002: Entity-Component-System Architecture

## Status
Accepted

## Context
We need to decide on an architecture for managing game entities (players, mobs, items, blocks) and their behavior.

## Decision
Use **Entity-Component-System (ECS)** pattern with the `hecs` crate.

## Rationale

### Component Types (data-only)
- `Position { x, y, z }` - world coordinates
- `Velocity { x, y, z }` - movement vector
- `Rotation { yaw, pitch }` - facing direction
- `Health { current, max }` - damage tracking
- `Inventory { slots }` - item storage
- `ChunkComponent { chunk_x, chunk_z }` - chunk association

### System Types (logic-only)
- `PhysicsSystem` - applies velocity, gravity, collision
- `NetworkSystem` - syncs entity state to clients
- `AISystem` - mob behavior and pathfinding
- `InventorySystem` - item interactions
- `HealthSystem` - damage and death handling

### Why notnaive struct + enum?
- ECS allows composition without inheritance
- Systems can query entities by component combination
- Parallelism: independent systems run concurrently
- Cache-friendly: components of same type stored contiguously

## Consequences

### Positive
- Clean separation of data and logic
- Easy to add new entity types by combining components
- Natural parallelism for independent systems
- Familiar pattern (Unity, Unreal both use similar models)

### Negative
- Learning curve for contributors unfamiliar with ECS
- Debugging may require specialized tools
- Some overhead compared to naive struct access

## References
- [hecs crate documentation](https://docs.rs/hecs)
- [ECS architecture in game development](https://www.gamedev.net/tutorials/_/technical/game-programming/ecs-game-engine-design-r5116/)
