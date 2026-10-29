# Gymnasium Compliance

How gymnasia maps to Python [Gymnasium](https://gymnasium.faraday.ai/) (v1.0).

## Core API

| Gymnasium | Gymnasia | Status |
|-----------|----------|--------|
| `Env` class | `Env` trait | Equivalent — `step`, `reset`, `close` |
| `(obs, reward, terminated, truncated, info)` | `StepResult<O>` | No `info` dict — wrappers expose typed methods instead |
| `env.np_random` | `rand_random(seed)` → `Pcg64` | Equivalent |
| `gym.make("CartPole-v1")` | `CartPoleEnv::new()` | No string registry — direct construction |

## Spaces

| Gymnasium | Gymnasia | Status |
|-----------|----------|--------|
| `Box` | `BoxSpace<B: Bounded>` | Generic over bound type |
| `Discrete` | `Discrete` | Equivalent (with masking) |
| `MultiDiscrete` | `MultiDiscrete` | Equivalent |
| `MultiBinary` | `MultiBinary` | Equivalent |
| `Tuple` | — | Missing |
| `Dict` | — | Missing |
| `Graph` | — | Missing |
| `Text` | — | Missing |
| `Sequence` | — | Missing |

## Wrappers

| Gymnasium | Gymnasia | Status |
|-----------|----------|--------|
| `TimeLimit` | `TimeLimit` | Equivalent |
| `OrderEnforcing` | `OrderEnforcing` | Equivalent |
| `AutoResetWrapper` | `Autoreset` | Equivalent |
| `RecordEpisodeStatistics` | `RecordEpisodeStatistics` | Equivalent (typed methods, no info dict) |
| `ClipAction` | `ClipAction` | Equivalent |
| `RescaleAction` | `RescaleAction` | Equivalent |
| `TransformAction` | `TransformAction` | Equivalent |
| `ClipReward` | `ClipReward` | Equivalent |
| `NormalizeReward` | `NormalizeReward` | Equivalent |
| `TransformReward` | `TransformReward` | Equivalent |
| `FlattenObservation` | `FlattenObservation` | Equivalent |
| `NormalizeObservation` | `NormalizeObservation` | Equivalent |
| `TransformObservation` | `TransformObservation` | Equivalent |
| `FrameStack` | — | Missing |
| `FrameSkip` / `AtariPreprocessing` | — | Missing |
| `GrayScaleObservation` | — | Missing |
| `ResizeObservation` | — | Missing |
| `HumanRendering` | — | Missing |
| `RenderCollection` | — | Missing |
| `PassiveEnvChecker` | — | Missing |

## Environments

### Classical Control

| Gymnasium | Gymnasia | Status |
|-----------|----------|--------|
| `CartPole-v1` | `CartPoleEnv` | Equivalent |
| `MountainCar-v0` | `MountainCarEnv` | Equivalent |
| `Pendulum-v1` | — | Missing |
| `Acrobot-v1` | — | Missing |
| `MountainCarContinuous-v0` | — | Missing |

### Other Categories

| Category | Gymnasium | Gymnasia |
|----------|-----------|----------|
| Toy text | FrozenLake, Taxi, CliffWalking, Blackjack | — |
| Box2D | LunarLander, BipedalWalker, CarRacing | — |
| MuJoCo | 17 environments | — |
| Atari | 50+ via ALE | — |

## Vector Environments

| Gymnasium | Gymnasia | Status |
|-----------|----------|--------|
| `SyncVectorEnv` | — | Planned (via rayon) |
| `AsyncVectorEnv` | — | Planned (via rayon) |

## Rendering

| Gymnasium | Gymnasia | Status |
|-----------|----------|--------|
| Pygame backend | Macroquad backend | Feature-gated (`render`) |
| `render_mode` param | `RenderMode` enum | Equivalent |
| `rgb_array` return | `RenderMode::RGB` | Equivalent |

## Intentional Divergences

These are deliberate design decisions, not missing features:

- **No `info` dict.** Wrappers expose metadata via typed methods. This is safer and avoids stringly-typed bugs.
- **No string registry.** Environments are constructed directly. Type-checked at compile time.
- **Generic `BoxSpace<B>`.** Works with custom observation structs, not just numpy arrays.
- **`Flatten` is opt-in.** Not a supertrait of `Env` — only needed by wrappers that flatten.
- **`Renderable` is separate from `Env`.** Simulation has zero graphics dependencies.
