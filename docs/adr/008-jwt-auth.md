# ADR-008: JWT Authentication for API

## Status
Accepted

## Context
We need a stateless authentication mechanism for the matchmaking API that scales horizontally.

## Decision
Use **JWT (JSON Web Tokens)** with the following claims:

### Token Structure
```go
type Claims struct {
    PlayerID  string `json:"player_id"`
    Username  string `json:"username"`
    Exp      int64  `json:"exp"`
    Iat      int64  `json:"iat"`
}
```

### Token Lifecycle
1. **Generation**: On successful login, issue JWT with 24h expiry
2. **Validation**: Every API request validates JWT signature and expiry
3. **Refresh**: Client can refresh token before expiry
4. **Revocation**: Add to Redis blocklist on logout

### Security Measures
- **HS256 signing**: Symmetric key (stored in environment)
- **Short expiry**: 24 hours (balance UX vs security)
- **Blocklist**: Redis tracks revoked tokens
- **HTTPS only**: Cookies marked Secure

### Password Storage
- **Argon2id**: Memory-hard function (19MB, 2 iterations)
- **Unique salt**: Per-password random 16-byte salt
- **No plaintext**: Never store or log passwords

## Implementation Details

### JWT Claims
```go
claims := jwt.MapClaims{
    "player_id": playerID.String(),
    "username":  username,
    "exp":       time.Now().Add(24 * time.Hour).Unix(),
    "iat":       time.Now().Unix(),
}
```

### Middleware
```go
func AuthMiddleware() gin.HandlerFunc {
    return func(c *gin.Context) {
        tokenString := c.GetHeader("Authorization")
        // validate and set player_id in context
    }
}
```

## Consequences

### Positive
- Stateless: any server can validate tokens
- Scalable: horizontal scaling of API servers
- Performance: no database lookup on each request
- Standard: widely understood, well-tested

### Negative
- Token in URL can be logged
- Cannot revoke tokens before expiry (mitigated with blocklist)
- Large tokens add overhead to every request

## References
- [RFC 7519: JWT](https://tools.ietf.org/html/rfc7519)
- [RFC 2104: HMAC](https://tools.ietf.org/html/rfc2104)
- [Argon2 Parameters](https://tools.ietf.org/html/rfc9106)
