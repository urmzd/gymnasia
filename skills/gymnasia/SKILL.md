---
name: gymnasia
description: Work with the gymnasia crate -- a pure-Rust implementation of OpenAI Gymnasium environments for reinforcement learning. Use when implementing RL environments in Rust, adding classical control environments, working with wrappers, or using action/observation spaces.
---

# Skill: gymnasia

## When to Use

- Implementing new RL environments in Rust following the `Env` trait
- Adding classical control environments (translate from Python Gymnasium to Rust)
- Composing behavior with wrappers (`TimeLimit`, `NormalizeObservation`, etc.)
- Working with action/observation spaces (`BoxSpace<B: Bounded>`, `Discrete`, `MultiDiscrete`, `MultiBinary`)
- Enabling macroquad-based rendering via the `render` feature

## Context

- **Language**: Rust (edition 2021)
- **Build**: `cargo` via `justfile`
- **Key traits**: `Env` (step, reset), `Renderable`, `Flatten`, `Wrapper` in `src/core/`
- **Environments**: `src/envs/classical_control/` -- `cartpole.rs`, `mountain_car.rs`
- **Spaces**: `src/spaces/` -- `BoxSpace<B: Bounded>`, `Discrete`, `MultiDiscrete`, `MultiBinary`
- **Wrappers**: `src/wrappers/` -- 13 composable wrappers, `delegate_env!` macro
- **Rendering**: Optional macroquad via `render` feature flag; headless by default
- **Lints**: `clippy` all warnings, `dead_code` denied, `unused_imports` warned

## Key Commands

```bash
just fetch   # fetch dependencies
just build   # build the project
just fmt     # format code
just lint    # run clippy
just check   # full CI check (fmt + lint + test)
```

## Conventions

- All environments implement `Env` (zero supertraits)
- Wrappers own their data -- no info dict
- `BoxSpace<B: Bounded>` is generic over the bounds representation
- `Flatten` is opt-in for ML pipelines
- `StepResult` uses `f64` reward; `O64` stays internal
- Feature flag `render` gates macroquad visualization
- Conventional commits required (see `sr.yaml`)
