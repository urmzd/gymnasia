# AGENTS.md

## Identity

**gymnasia** -- OpenAI Gymnasium environments implemented in pure Rust.

## Architecture

The crate is organized into four top-level modules:

| Module | Purpose |
|--------|---------|
| `core` | The `Env` and `Renderable` traits that every environment implements, plus `ActionReward`. |
| `envs` | Concrete environment implementations (e.g. `cartpole`, `mountain_car`) under `classical_control`. |
| `spaces` | Value-space descriptors: `BoxR`, `Discrete`, and the `Space` trait. |
| `utils` | Rendering (`renderer`), seeding (`seeding`), and shared helpers (`custom/`). |

Environments implement `Env` (step, reset, render, close) and `EnvProperties` (action/observation spaces, metadata). An optional `sdl2` feature enables graphical rendering; without it the crate runs headless.

## Key Files

| File | Description |
|------|-------------|
| `src/lib.rs` | Crate root; re-exports all public modules. |
| `src/core.rs` | `Env` and `Renderable` traits, `ActionReward`. |
| `src/envs/classical_control/cartpole.rs` | CartPole environment. |
| `src/envs/classical_control/mountain_car.rs` | MountainCar environment. |
| `src/spaces/box_r.rs` | Continuous box space. |
| `src/spaces/discrete.rs` | Discrete action/observation space. |
| `src/utils/renderer.rs` | SDL2-based renderer and `RenderMode` enum. |
| `Cargo.toml` | Package metadata, dependencies, and feature flags. |
| `justfile` | Build, test, lint, and formatting tasks. |

## Commands

```bash
just init          # install deps + git hooks
just build         # cargo build (with SDL2)
just build-headless # cargo build --no-default-features
just test          # cargo test --no-default-features
just test-all      # cargo test (all features)
just lint          # cargo clippy (headless)
just lint-all      # cargo clippy (all features)
just fmt           # cargo fmt + taplo fmt + alejandra
just fmt-check     # check formatting without changes
just check         # fmt-check + lint + test + doc
just doc           # build docs
```

## Code Style

- **Formatting**: `rustfmt` (config in `rustfmt.toml`).
- **Linting**: `clippy` with all warnings enabled; `dead_code` is denied.
- **Commit convention**: Angular conventional commits enforced via `sr.yaml` / gitit.
- **Feature flags**: `sdl2` (default) for rendering, `bundled` to compile SDL2 from source.
