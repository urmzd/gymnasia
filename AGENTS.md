# AGENTS.md

## Identity

**gymnasia** -- OpenAI Gymnasium environments implemented in pure Rust.

## Architecture

The crate separates simulation from rendering across six top-level modules:

| Module | Purpose |
|--------|---------|
| `core` | `Env`, `Renderable`, `StepResult`, and `Flatten` traits/types. Zero supertraits on `Env`. |
| `envs` | Concrete environment implementations under `classical_control` (CartPole, MountainCar). |
| `spaces` | `BoxSpace<B: Bounded>`, `Discrete`, `MultiDiscrete`, `MultiBinary`, plus `Space`, `SampleSpace`, `Bounded` traits and `Tensor` type. |
| `render` | Drawing primitives (`DrawList`), macroquad backend (`Screen`), `Renderer` mode logic, and `RenderEnv` wrapper (implements `Env`). |
| `wrappers` | `Wrapper` trait, `delegate_env!` macro, and 13 composable wrappers: `TimeLimit`, `OrderEnforcing`, `Autoreset`, `RecordEpisodeStatistics`, `ClipReward`, `NormalizeReward`, `TransformReward`, `ClipAction`, `RescaleAction`, `TransformAction`, `FlattenObservation`, `NormalizeObservation`, `TransformObservation`. |
| `utils` | Seeding (`seeding`), clip function (`clip`), and `O64` type alias (`types`). |

### Design decisions

- **`Env` has zero supertraits** -- concrete types derive `Clone`, `Debug`, `Serialize` as needed. Wrappers with closures work without restriction.
- **Wrappers own their data** -- `RecordEpisodeStatistics` exposes `episode_return()`, `TimeLimit` exposes `steps_remaining()`. No info dict.
- **`BoxSpace<B: Bounded>`** -- generic over the bounds representation. Implement `Bounded` on observation structs, use `f64` for scalars, or `Tensor` for high-dimensional spaces.
- **`Flatten` is opt-in** -- bidirectional conversion for ML pipelines. Not a bound on `Env`.
- **`StepResult` uses `f64` reward** -- `O64` (OrderedFloat) stays internal to env physics.
- **`RenderEnv` implements `Env`** -- participates in wrapper chains.

## Key Files

| File | Description |
|------|-------------|
| `src/core/env.rs` | `Env` trait definition. |
| `src/core/step_result.rs` | `StepResult<O>` struct. |
| `src/core/flatten.rs` | `Flatten` trait for ML interop. |
| `src/core/render.rs` | `Renderable` trait. |
| `src/spaces/bounded.rs` | `Bounded` trait, `BoxSpace<B>`, `Tensor`. |
| `src/spaces/discrete.rs` | `Discrete` space with optional start offset. |
| `src/spaces/multi_discrete.rs` | `MultiDiscrete` space. |
| `src/spaces/multi_binary.rs` | `MultiBinary` space. |
| `src/wrappers/mod.rs` | `Wrapper` trait, `delegate_env!` macro, re-exports. |
| `src/wrappers/time_limit.rs` | `TimeLimit` wrapper (representative of pattern). |
| `src/envs/classical_control/cartpole.rs` | CartPole environment. |
| `src/envs/classical_control/mountain_car.rs` | MountainCar environment. |
| `src/render/render_env.rs` | `RenderEnv` wrapper (implements `Env`). |
| `src/render/draw.rs` | `DrawList`, `DrawCommand`, `Color`. |
| `src/render/screen.rs` | Macroquad rendering backend (feature-gated). |
| `Cargo.toml` | Package metadata, dependencies, and feature flags. |

## Commands

```bash
cargo build --no-default-features   # headless build
cargo build --features render       # with macroquad rendering
cargo test --no-default-features    # run tests
cargo bench --no-default-features   # run benchmarks
cargo doc --no-default-features     # build docs
```

## Code Style

- **Formatting**: `rustfmt` (default settings).
- **Linting**: `clippy` with all warnings enabled; `dead_code` is denied.
- **Commit convention**: Angular conventional commits enforced via `sr.yaml`.
- **Feature flags**: `render` for macroquad-based visualization.
