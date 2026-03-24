# ADR-001: Rust as the Primary Engine Language

## Status
Accepted

## Context
We need to choose the primary language for the game engine. The options considered were C++, Rust, and Zig.

## Decision
Use **Rust** as the primary engine language for the following reasons:

1. **Memory Safety**: Rust's ownership system prevents entire classes of bugs (null pointers, use-after-free, data races) at compile time, critical for a long-running game server.

2. **Performance**: Rust achieves C-level performance with zero-cost abstractions, essential for voxel rendering and world simulation.

3. **Ecosystem**: WGPU (our renderer) and Tokio (our async runtime) are first-class Rust libraries with excellent documentation.

4. **Tooling**: Cargo provides unified build, test, and dependency management across all crates.

5. **Interoperability**: Rust can call C libraries directly and can be called from Go via cgo, enabling our microservices architecture.

## Consequences

### Positive
- Single language for engine, server, and most tooling
- Memory-safe server code reduces security vulnerabilities
- Excellent async support via Tokio for network handling
- Cross-platform support (Windows, macOS, Linux)

### Negative
- Steeper learning curve for contributors unfamiliar with Rust
- Longer compile times compared to dynamic languages
- Some game-specific libraries may be missing (fallback to C libraries)

## References
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Why Rust for game development](https://ryhl.io/blog/rust-for-games/)
