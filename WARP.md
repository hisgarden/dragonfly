# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**DragonFly** is a privacy-first macOS maintenance utility written in Rust. It provides disk analysis, duplicate file detection, system monitoring, cache cleaning, and health diagnostics—all with 100% local processing and zero network activity.

**Version**: 0.1.0 (Alpha MVP)  
**Status**: Development-ready  
**Architecture**: Clean Architecture with Domain-Driven Design (DDD) and Hexagonal Architecture

## Common Development Commands

### Build
```bash
# Debug build (fast, for development)
cargo build --workspace

# Release build (optimized, ~40x faster at runtime)
cargo build --release --workspace

# Specific crate
cargo build -p dragonfly-cli
cargo build -p dragonfly-core
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run tests with output (--nocapture shows println!)
cargo test --workspace -- --nocapture

# Test specific crate
cargo test -p dragonfly-core
cargo test -p dragonfly-cli

# Single test
cargo test test_name

# Coverage report (requires cargo-tarpaulin)
cargo tarpaulin --workspace --out Html
```

### Code Quality
```bash
# Format code
cargo fmt --all

# Lint (fails on warnings)
cargo clippy --workspace --all-targets -- -D warnings

# Auto-fix clippy warnings
cargo clippy --workspace --all-targets --fix --allow-dirty

# Security audit
cargo audit

# License compliance check
cargo deny check
```

### Using Taskfile (Recommended)
```bash
# View all tasks
task --list

# Common workflow
task build      # or: task b
task test       # or: task t
task check      # runs fmt-check, clippy, audit, sbom-check
task ci         # full CI simulation (build + test + quality)

# Run CLI
task run                    # shows help
task run CLI_ARGS="health"  # run specific command
task run-disk CLI_ARGS="--min-size 500MB ~/"

# Clean
task clean              # remove build artifacts
task clean-all          # deep clean (+ Cargo.lock)
```

## Architecture Overview

DragonFly uses **Clean Architecture** organized as a Rust workspace with 6 modular crates:

### Crate Dependency Graph
```
dragonfly-cli (Binary) ← depends on all modules
├── dragonfly-core (Domain Layer)
├── dragonfly-disk
├── dragonfly-duplicates
├── dragonfly-monitor
└── dragonfly-cleaner

dragonfly-core
├── Domain entities & value objects (pure logic, no external deps)
├── Ports (trait interfaces for dependency inversion)
└── Use cases (business logic orchestration)
```

### Design Principles

**SOLID Principles Applied**:
- **Single Responsibility**: Each module has one reason to change
- **Dependency Inversion**: Domain depends on abstractions (ports), not implementations
- **Interface Segregation**: Specific ports over generic ones

**Domain-Driven Design (DDD)**:
- **Ubiquitous Language**: Code uses domain expert terminology
- **Rich Domain Models**: Entities encapsulate behavior, not just data
- **Value Objects**: Immutable types (FileSize, FilePath, etc.) with type safety
- **Domain Events**: Capture important business occurrences

**Hexagonal Architecture (Ports & Adapters)**:
- **Ports**: Trait boundaries defined in dragonfly-core
- **Adapters**: Infrastructure implementations in each module (file system, CLI, etc.)
- **Domain Independence**: Core logic has NO dependencies on infrastructure

### Key Directories

```
crates/
├── dragonfly-cli/           # CLI entry point, command handlers
│   └── src/
│       ├── main.rs          # Clap CLI definition and command routing
│       ├── commands/        # Command handlers (analyze, clean, etc.)
│       └── error_tracking/  # Sentry/GlitchTip integration
│
├── dragonfly-core/          # Domain layer (PURE business logic)
│   └── src/
│       ├── domain/          # Entities, value objects, domain events
│       │   ├── entities/    # FileEntity, DirectoryEntity, etc.
│       │   └── value_objects/  # FileSize, FilePath, Percentage
│       ├── ports/           # Trait interfaces (Repository, etc.)
│       ├── use_cases/       # Business logic orchestration
│       └── error.rs         # Domain error types
│
├── dragonfly-disk/          # Disk analysis implementation
├── dragonfly-duplicates/    # Duplicate detection with Blake3 hashing
├── dragonfly-monitor/       # System metrics (CPU, memory, etc.)
└── dragonfly-cleaner/       # Cache/temp file cleaning
```

### Important Architecture Rules

**dragonfly-core MUST NOT**:
- Perform file system operations (use ports, implement in adapters)
- Make network calls
- Depend on external services
- Know about CLI or UI

**All Modules**:
- Implement port traits from dragonfly-core
- Keep domain logic separate from infrastructure
- Add tests (TDD approach)
- Document public APIs with doc comments

## Key Files and Their Roles

- **Cargo.toml** (workspace root): Defines all members and shared dependencies
- **crates/*/Cargo.toml**: Individual crate dependencies
- **README.md**: Feature overview and usage examples
- **SETUP.md**: Detailed build, test, and development guide
- **Taskfile.yml**: All development tasks (preferred over raw cargo)
- **docs/**: Architecture and strategy documents

## Testing Strategy

### Test Organization
Tests live in the same files as code using `#[cfg(test)]` modules:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        // Test implementation
    }
}
```

### Test Types
- **Unit Tests**: Test individual functions/types in isolation
- **Property-Based Tests**: Use `proptest` for value objects to verify invariants
- **Integration Tests**: Test module interactions
- **Mocks**: Use `mockall` to mock port implementations

### Running Tests
```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p dragonfly-core

# Single test with output
cargo test test_name -- --nocapture
```

## Dependencies Overview

### Core Dependencies
- **tokio**: Async runtime (full feature set)
- **clap**: CLI argument parsing (with derive macros)
- **serde/serde_json**: Serialization
- **thiserror/anyhow**: Error handling

### Specialized Libraries
- **blake3**: Fast hashing for duplicate detection (~5GB/s)
- **rayon**: Parallel processing (all CPU cores)
- **sysinfo**: System information
- **walkdir/jwalk**: File system traversal
- **tracing**: Structured logging

### Testing Tools
- **proptest**: Property-based testing
- **mockall**: Mock implementations
- **rstest**: Parameterized tests
- **criterion**: Benchmarking

### Security & Compliance
- **sentry**: Error tracking (opt-in only, self-hosted)
- **cargo-audit**: Security vulnerability scanning
- **cargo-deny**: License compliance checking

## Error Handling Pattern

Use strong types with `thiserror`:

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid size: {0}")]
    InvalidSize(#[from] std::num::TryFromIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
```

All domain errors should:
1. Be typed (not string-based)
2. Provide context for debugging
3. Be convertible from underlying errors with `#[from]`

## Adding New Features

### 1. Start in dragonfly-core
- Define domain entities/value objects
- Define port traits (interfaces)
- Implement business logic in use cases
- Write tests first (TDD)

### 2. Implement adapters in feature modules
- Create implementation in specific crate (dragonfly-disk, etc.)
- Implement port traits from core
- Add infrastructure-specific logic

### 3. Wire to CLI
- Create command handler in dragonfly-cli/src/commands/
- Add subcommand to main.rs Clap enum
- Route in match statement

### 4. Test end-to-end
```bash
task test
task check
task ci
```

## Privacy & Opt-In Error Tracking

DragonFly is privacy-first (100% local processing). Error tracking is **opt-in only**:

```bash
# Enable GlitchTip error tracking (must be explicitly requested)
dragonfly health --enable-error-tracking
```

**Rules**:
- Error tracking disabled by default
- Only works with self-hosted GlitchTip or local server
- No data sent to external cloud services
- Users must explicitly enable with flag

## Build Profiles

- **dev**: Fast compilation, debug info, slow runtime
- **release**: Optimized for speed (~40x faster), stripped
- **bench**: Release build with debug info (benchmarking)

Use release build when:
- Measuring performance
- Running on large directories
- Shipping to users

## Performance Notes

- Blake3 hashing: ~5 GB/s on modern CPUs
- Parallel scanning uses Rayon (leverages all cores)
- Release builds are ~40x faster than debug
- JSON output suitable for streaming to AI models

## Common Patterns

### Parallel Processing
```rust
use rayon::prelude::*;
items.par_iter().map(|item| process(item)).collect()
```

### Async/Await
```rust
#[tokio::main]
async fn main() {
    let result = async_function().await?;
}
```

### Error Propagation
```rust
fn operation() -> Result<Output> {
    let value = fallible_operation()?; // Auto-convert using thiserror
    Ok(output)
}
```

## Debugging

Enable debug logging:
```bash
task run CLI_ARGS="health --debug"
# or directly
RUST_LOG=debug cargo run -p dragonfly-cli -- health
```

Check specific module logging:
```bash
RUST_LOG=dragonfly_core=debug cargo test --workspace -- --nocapture
```

## CI/CD Workflow

Run full CI locally:
```bash
task ci
# Equivalent to:
# - task ci-build (release build)
# - task ci-test (run tests)
# - task ci-quality (fmt-check, clippy, sbom)
```

This catches issues before pushing.

## When Modifying Code

1. **Before**: Run `task check` to establish baseline
2. **Modify**: Make your changes
3. **Test**: `cargo test --workspace`
4. **Lint**: `cargo clippy --workspace --all-targets -- -D warnings`
5. **Format**: `cargo fmt --all`
6. **Verify**: `task ci`

## Important Notes for AI Agents

- **Tests are authoritative**: Code must pass all tests
- **Types are your friends**: Rust's type system catches errors at compile time
- **Ports first**: When unsure about implementation location, check port traits in dragonfly-core
- **Value objects**: FileSize, FilePath, etc. have invariants—respect them
- **Async context**: main() is async; commands return futures
- **Error context**: Always provide context in error messages for debugging
- **Commit style**: Include `Co-Authored-By: Warp <agent@warp.dev>` in commit messages

## Running Single Commands

```bash
# Health check
task run-health

# Disk analysis with size filter
task run-disk CLI_ARGS="--min-size 500MB ~/"

# Cleanup (dry-run, doesn't delete)
task run-clean

# Time Machine snapshots
task run CLI_ARGS="time-machine snapshots"

# Monitor system (10s intervals)
task run CLI_ARGS="monitor --interval 10"
```

## Feature Flags

DragonFly supports optional features that can be enabled at compile time:

### Skills Command (cheat sheet)
```bash
# Build with skills feature
cargo build -p dragonfly-cli --features skills

# Run skills command
./target/debug/dragonfly skills
./target/debug/dragonfly skills --json
```

### TUI - Retro Defrag Animation
```bash
# Build with TUI feature
cargo build -p dragonfly-cli --features tui

# Run defrag TUI
./target/debug/dragonfly defrag ~/
./target/debug/dragonfly defrag ~/Downloads

# Features:
# - Full-screen terminal UI (alternate screen buffer)
# - Animated "80s defrag" style block visualization
# - Real-time progress display
# - Press Q or Ctrl+C to quit
# - Terminal state always restored on exit
```

## Additional Resources

- **README.md**: Feature overview and usage examples
- **SETUP.md**: Comprehensive build, test, and setup guide
- **docs/**: Architecture decisions and strategies
- `cargo doc --workspace --no-deps --open`: Generated API documentation
