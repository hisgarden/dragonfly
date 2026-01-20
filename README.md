# DragonFly

macOS maintenance tool. Written in Rust. Does what it says.

## What it does

Cleans your Mac. Finds duplicates. Shows disk usage. Monitors system. No network calls. Everything stays local.

## Build it

```bash
git clone https://github.com/hisgarden/dragonfly.git
cd dragonfly
cargo build --release
```

You need Rust. Get it from rustup.rs if you don't have it.

## Use it

```bash
# Check system health
./target/release/dragonfly health

# See what's eating disk space
./target/release/dragonfly disk analyze ~/Documents

# Find duplicate files
./target/release/dragonfly duplicates scan ~/Pictures

# Watch system metrics
./target/release/dragonfly monitor

# Clean caches
./target/release/dragonfly clean --dry-run
```

## Commands

### Disk analysis

Shows what's using space. Fast. Parallel scanning.

```bash
dragonfly disk analyze ~/
dragonfly disk analyze ~/ --json > report.json
dragonfly disk analyze ~/ --min-size 500MB
```

### Duplicates

Finds duplicate files. Uses Blake3 hashing. Fast.

```bash
dragonfly duplicates scan ~/Pictures
dragonfly duplicates scan ~/Documents --interactive
dragonfly duplicates scan ~/ --dry-run
```

### Monitor

Shows CPU, memory, disk, network. Updates every few seconds.

```bash
dragonfly monitor
dragonfly monitor --interval 1
dragonfly monitor --json --interval 60
```

### Clean

Removes caches and temp files. Safe. Shows what it'll do first.

```bash
dragonfly clean --dry-run
dragonfly clean --caches
dragonfly clean --all
```

### Health check

System diagnostics. Tells you what's wrong.

```bash
dragonfly health
dragonfly health --json
dragonfly health --component disk
```

## Development

```bash
# Build everything
cargo build --workspace

# Run tests
cargo test --workspace

# Format code
cargo fmt

# Lint
cargo clippy --workspace --all-targets

# Security audit
cargo audit
```

Or use the taskfile:

```bash
task build
task check
task test
```

## Structure

```
crates/
├── dragonfly-core/      # Core domain logic
├── dragonfly-disk/     # Disk analysis
├── dragonfly-duplicates/ # Duplicate finder
├── dragonfly-monitor/  # System monitoring
├── dragonfly-cleaner/  # Cache cleaning
└── dragonfly-cli/      # Command line interface
```

Clean architecture. Domain-driven design. SOLID principles. Tests first.

## Privacy

No network calls. No telemetry. No data collection. Everything runs local. Check the code yourself.

## Performance

Fast. Parallel processing. Optimized builds. Faster than shell scripts.

## License

MIT OR Apache-2.0. Your choice.

## Contributing

Fork it. Write tests. Make it work. Send a PR.

---

Built with Rust. No BS.
