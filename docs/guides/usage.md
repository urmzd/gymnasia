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
gymnasia = "3"
```

**With rendering:**
```toml
[dependencies]
gymnasia = { version = "3", features = ["render"] }
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
cargo run --example=cartpole_headless --no-default-features
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
just fetch        # Fetch dependencies
just build        # Build the project
just fmt          # Format code
just lint         # Run clippy
just check        # Run all CI checks (fmt + lint + test)
```

## CI Verification

To run the same checks CI performs locally:

```bash
just check
```

Or individually:

```bash
cargo fmt --all -- --check
cargo clippy --no-default-features -- -D warnings
cargo test --no-default-features
```
