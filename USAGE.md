# Usage

## Prerequisites

| Tool | Required | Notes |
|------|----------|-------|
| [Rust](https://rustup.rs/) | Yes | 1.82.0+ (2021 edition) |
| [SDL2 + SDL2_gfx](https://wiki.libsdl.org/Installation) | Only with rendering | Skippable via `default-features = false` |
| [just](https://github.com/casey/just) | Recommended | Task runner for common commands |
| [Nix](https://nixos.org/download/) | Optional | Reproducible dev environment (includes all deps) |

## Installation

### As a dependency

**With rendering (default):**
```toml
[dependencies]
gymnasia = "1.0.0"
```

**Headless (no SDL2 required):**
```toml
[dependencies]
gymnasia = { version = "1.0.0", default-features = false }
```

### From source

```bash
git clone https://github.com/urmzd/gym-rs.git
cd gym-rs
```

## Environment Setup

### Nix (any OS — recommended)

Nix provides all system dependencies automatically. Nothing else to install.

```bash
# One-time: enable flakes if you haven't already
# https://nixos.wiki/wiki/Flakes#Enable_flakes

nix develop          # Enter dev shell with all deps
just check           # Verify everything works
```

If you have [direnv](https://direnv.net/) installed, the `.envrc` activates the flake automatically when you `cd` into the project.

### Ubuntu / Debian

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# SDL2 (only needed for rendering)
sudo apt install libsdl2-dev libsdl2-gfx-dev cmake

# just (task runner)
cargo install just
```

### Arch Linux

```bash
# Rust
sudo pacman -S rustup
rustup default stable

# SDL2 (only needed for rendering)
sudo pacman -S sdl2 sdl2_gfx cmake

# just (task runner)
cargo install just
```

### macOS

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# SDL2 (only needed for rendering)
brew install sdl2 sdl2_gfx cmake

# just (task runner)
brew install just
```

### Windows

SDL2 rendering on Windows requires extra setup. The simplest path is headless mode:

```toml
[dependencies]
gymnasia = { version = "1.0.0", default-features = false }
```

If you need rendering, use the `bundled` feature which compiles SDL2 from source (requires CMake and a C compiler):

```bash
# Install Rust via https://rustup.rs
# Install CMake via https://cmake.org/download or `winget install cmake`
# Install Visual Studio Build Tools (C++ workload)

cargo build  # The `bundled` feature (on by default) handles SDL2
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `sdl2` | Yes | Enables SDL2 rendering and nalgebra |
| `bundled` | Yes | Compiles SDL2 from source (no system install needed) |

**Common configurations:**

```bash
# Full build (rendering + bundled SDL2)
cargo build

# Headless only (no SDL2 at all)
cargo build --no-default-features

# System SDL2 (use your OS package, don't bundle)
cargo build --no-default-features --features sdl2
```

## Running Examples

### With rendering (requires SDL2)

```bash
just cartpole         # or: cargo run --example=cartpole
just mountain-car     # or: cargo run --example=mountain_car
```

### Headless (no SDL2)

```bash
just cartpole-headless  # or: cargo run --example=cartpole_headless --no-default-features
```

## Common Tasks

All tasks are available via `just`:

```bash
just              # List all available recipes
just check        # Run fmt + clippy + test + doc
just test         # Run tests (headless)
just test-all     # Run tests (all features)
just fmt          # Format Rust + TOML + Nix
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
cargo check --no-default-features
cargo clippy --no-default-features -- -D warnings
cargo test --no-default-features
cargo doc --no-deps --no-default-features
cargo fmt --all -- --check
```
