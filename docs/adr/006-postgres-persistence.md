# ADR-006: PostgreSQL for World Persistence

## Status
Accepted

## Context
We need a robust database for storing player data, world chunks, and game state.

## Decision
Use **PostgreSQL 15+** as the primary database with **Redis** for caching and sessions.

### Schema Overview

#### worlds
```sql
CREATE TABLE worlds (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    seed BIGINT NOT NULL,
    generator VARCHAR(32) DEFAULT 'vanilla',
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

#### chunks
```sql
CREATE TABLE chunks (
    world_id INT REFERENCES worlds(id),
    chunk_x INT NOT NULL,
    chunk_z INT NOT NULL,
    data BYTEA NOT NULL,           -- Binary chunk data
    modified_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (world_id, chunk_x, chunk_z)
);
```

#### players
```sql
CREATE TABLE players (
    id UUID PRIMARY KEY,
    username VARCHAR(16) UNIQUE NOT NULL,
    display_name VARCHAR(32),
    world_id INT REFERENCES worlds(id),
    position_x DOUBLE PRECISION,
    position_y DOUBLE PRECISION,
    position_z DOUBLE PRECISION,
    yaw FLOAT,
    pitch FLOAT,
    health FLOAT DEFAULT 20.0,
    hunger FLOAT DEFAULT 20.0,
    experience INT DEFAULT 0,
    game_mode VARCHAR(16) DEFAULT 'survival',
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Why PostgreSQL?
- **ACID compliance**: Critical for player inventory, economy
- **Binary data**: BYTEA for efficient chunk storage
- **JSON support**: For player_data key-value store
- **Mature ecosystem**: sqlx provides excellent async support
- **PostGIS ready**: Could add geographic queries later

### Why Redis for sessions?
- **Sub-millisecond latency**: Session lookups
- **TTL support**: Automatic session expiration
- **Pub/sub**: Real-time player status updates
- **Hash operations**: Lobby state management

## Consequences

### Positive
- Persistent worlds survive server restart
- Player progress saved automatically
- Efficient chunk storage with compression
- ACID transactions for inventory/survival data

### Negative
- Database connection overhead
- Chunk serialization/deserialization cost
- Migration complexity with schema changes

## References
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [sqlx crate](https://docs.rs/sqlx)
- [Redis Documentation](https://redis.io/docs/)
