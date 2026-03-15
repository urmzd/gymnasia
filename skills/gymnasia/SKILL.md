# Skill: gymnasia

## Description

Work with the gymnasia crate -- a pure-Rust implementation of OpenAI Gymnasium environments for reinforcement learning.

## When to Use

- Implementing new RL environments in Rust following the `Env` / `EnvProperties` traits
- Adding classical control environments (translate from Python OpenAI Gym to Rust)
- Debugging rendering issues with the SDL2 backend
- Working with action/observation spaces (`BoxR`, `Discrete`)

## Context

- **Language**: Rust (edition 2021)
- **Build**: `cargo` via `justfile`
- **Key traits**: `Env` (step, reset, render, close) and `EnvProperties` (action_space, observation_space) in `src/core.rs`
- **Environments**: `src/envs/classical_control/` -- `cartpole.rs`, `mountain_car.rs`
- **Spaces**: `src/spaces/` -- `BoxR` (continuous), `Discrete`
- **Rendering**: Optional SDL2 via `sdl2` feature flag; headless by default in tests
- **Lints**: `clippy` all warnings, `dead_code` denied, `unused_imports` warned

## Key Commands

```bash
just init           # install deps + hooks
just build          # build with SDL2
just build-headless # build without SDL2
just test           # test headless
just lint           # clippy headless
just fmt            # format code
just check          # full CI check
```

## Conventions

- All environments implement `Env` + `EnvProperties`
- Observations must implement `Sample + Into<Vec<f64>>`
- Use `ordered_float::OrderedFloat<f64>` for reward values
- Feature flag `sdl2` gates rendering and `nalgebra` dependency
- Conventional commits required (see `sr.yaml`)
