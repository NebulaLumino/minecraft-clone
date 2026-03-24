# Contributing to VoxelCraft

Thank you for your interest in contributing to VoxelCraft!

## Project Structure

```
minecraft-clone/
├── Cargo.toml           # Workspace manifest
├── docker/              # Docker configuration
├── docs/                # Documentation
├── crates/
│   ├── shared/          # Shared types and logic (no external deps)
│   ├── client/          # WGPU-based game client
│   ├── server/          # Tokio-based game server
│   └── matchmaking/     # Go-based matchmaking service
```

## Development Setup

### Prerequisites

- Rust 1.75+
- Go 1.21+
- Docker and Docker Compose
- PostgreSQL 15+
- Redis 7+

### Quick Start

1. Clone the repository
2. Copy `.env.example` to `.env` and configure
3. Start infrastructure: `docker compose -f docker/docker-compose.yml up -d`
4. Build: `cargo build --workspace`
5. Run tests: `cargo test --workspace`

## Code Style

### Rust

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Public APIs should have documentation comments
- Error types should implement `std::error::Error`

### Go

- Run `go fmt` before committing
- Follow [Effective Go](https://go.dev/doc/effective_go)
- Use meaningful variable names
- Comment complex code

## Testing

### Rust

```bash
# Run all tests
cargo test --workspace

# Run tests with coverage
cargo tarpaulin --workspace

# Run specific crate tests
cargo test -p voxel-shared
```

### Go

```bash
cd crates/matchmaking
go test ./...
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests and ensure they pass
5. Commit with clear commit messages
6. Push to your fork
7. Open a Pull Request

### PR Guidelines

- Reference issues in your PR description
- Keep PRs focused on a single feature/fix
- Include tests for new functionality
- Update documentation if needed
- Follow the commit message format

## Commit Message Format

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Reporting Bugs

- Use the GitHub issue tracker
- Include Rust version (`rustc --version`)
- Include relevant error messages
- Include steps to reproduce
- Include a minimal reproduction if possible

## Suggesting Features

- Open a GitHub discussion first
- Describe the feature and its use case
- Explain why it would benefit the project
- Be open to feedback and alternatives

## Code of Conduct

- Be respectful and inclusive
- Be patient and helpful
- Assume good faith
- Focus on what is best for the community

## License

By contributing, you agree that your contributions will be licensed under the project's licenses (MIT OR Apache-2.0).
