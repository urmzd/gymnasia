# Usage

## Prerequisites

| Tool | Required | Notes |
|------|----------|-------|
| [Rust](https://rustup.rs/) | Yes | 1.82.0+ (2021 edition) |
| [just](https://github.com/casey/just) | Recommended | Task runner for common commands |

No system C libraries are required. Rendering uses macroquad (pure Rust, compiled from source via the `render` feature).

## Installation

### As a dependency

**Headless (default):**
```toml
[dependencies]
gymnasia = "2"
```

**With rendering:**
```toml
[dependencies]
gymnasia = { version = "2", features = ["render"] }
```

### From source

```bash
git clone https://github.com/urmzd/gymnasia.git
cd gymnasia
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `render` | No | macroquad-based window rendering and pixel capture |

**Common configurations:**

```bash
# Headless (default — no graphics deps)
cargo build

# With rendering
cargo build --features render
```

## Running Examples

### Headless

```bash
cargo run --example=cartpole_headless
```

### With rendering

```bash
cargo run --example=cartpole --features render
cargo run --example=mountain_car --features render
```

## Common Tasks

All tasks are available via `just`:

```bash
just              # List all available recipes
just check        # Run fmt + clippy + test + doc
just test         # Run tests (headless)
just test-all     # Run tests (all features)
just fmt          # Format Rust + TOML
just lint         # Clippy with warnings as errors
just doc          # Build documentation
just clean        # Remove build artifacts
```

## CI Verification

To run the same checks CI performs locally:

```bash
just check
```

Or individually:

```bash
cargo check
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
cargo fmt --all -- --check
```
