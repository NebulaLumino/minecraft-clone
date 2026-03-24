# ADR-007: Go for Matchmaking Microservice

## Status
Accepted

## Context
We need a separate service for player authentication, matchmaking, and lobby management.

## Decision
Use **Go** with the **Gin** web framework for the matchmaking service.

### Service Architecture
```
matchmaking/
├── cmd/server/main.go          # Entry point
└── internal/
    ├── api/
    │   ├── handlers.go         # HTTP handlers
    │   ├── middleware.go       # Auth middleware
    │   └── routes.go           # Route definitions
    ├── auth/
    │   ├── argon2.go           # Password hashing
    │   └── jwt.go              # Token management
    ├── lobby/
    │   └── manager.go          # Lobby CRUD operations
    ├── redis/
    │   └── client.go           # Session store
    └── models/
        └── types.go            # Domain models
```

### Authentication Flow
1. User registers with username/password
2. Server hashes password with Argon2id
3. User logs in, server validates password
4. Server issues JWT with player ID and expiry
5. Client stores JWT and sends in Authorization header
6. Server validates JWT on each request

### Lobby Flow
1. Player creates lobby (gets lobby ID)
2. Player shares lobby ID with friends
3. Friends join lobby using ID
4. Lobby owner starts game when ready
5. Matchmaking service returns server address
6. Players connect to game server

## Consequences

### Positive
- Go's concurrency model (goroutines) ideal for connection-heavy services
- Gin provides excellent HTTP performance
- Separate service isolates auth from game server
- Microservice architecture enables independent scaling
- Easy deployment with Docker containers

### Negative
- Two language codebases to maintain
- Inter-service communication complexity
- Need to keep Go and Rust code in sync
- Additional infrastructure (Docker, Redis)

## References
- [Go Documentation](https://go.dev/doc/)
- [Gin Web Framework](https://gin-gonic.com/)
- [Argon2 Password Hashing](https://password-hashing.net/)
- [JWT.io](https://jwt.io/)
