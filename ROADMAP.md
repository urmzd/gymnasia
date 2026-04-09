# Roadmap

Feature parity mapping between **[gymnasia](https://github.com/urmzd/gymnasia)** (Rust) and **[Gymnasium](https://github.com/Farama-Foundation/Gymnasium)** (Python).

Legend: done / partial / planned / not planned

---

## Core API

| Feature | Gymnasium | gymnasia | Status |
|---------|-----------|----------|--------|
| `Env` trait (step, reset) | `gymnasium.Env` | `core::Env` | done |
| `Renderable` trait | Built into `Env` | `core::Renderable` | done |
| `ActionReward` (obs, reward, terminated, truncated, info) | tuple return | `core::ActionReward` | done |
| `RenderEnv` wrapper | `render_mode` param | `render::RenderEnv` | done |
| Environment registry (`gymnasium.make()`) | `gymnasium.make` | -- | planned |
| Spec / EnvSpec metadata | `gymnasium.envs.registration` | -- | planned |
| `close()` method | `Env.close()` | -- | planned |

## Spaces

| Space | Gymnasium | gymnasia | Status |
|-------|-----------|----------|--------|
| `Box` (continuous bounded) | `gymnasium.spaces.Box` | `spaces::BoxR` | done |
| `Discrete` | `gymnasium.spaces.Discrete` | `spaces::Discrete` | done |
| `Discrete` with start offset | `Discrete(n, start=k)` | `spaces::DiscreteRange` | done |
| Action masking (sample w/ mask) | `spaces.sample(mask=)` | `SampleSpace` trait | done |
| `MultiDiscrete` | `gymnasium.spaces.MultiDiscrete` | -- | planned |
| `MultiBinary` | `gymnasium.spaces.MultiBinary` | -- | planned |
| `Tuple` | `gymnasium.spaces.Tuple` | -- | planned |
| `Dict` | `gymnasium.spaces.Dict` | -- | planned |
| `Text` | `gymnasium.spaces.Text` | -- | planned |
| `Graph` | `gymnasium.spaces.Graph` | -- | planned |
| `Sequence` | `gymnasium.spaces.Sequence` | -- | planned |
| `OneOf` | `gymnasium.spaces.OneOf` | -- | planned |
| `spaces.flatdim` / `flatten` | `gymnasium.spaces.utils` | -- | planned |

## Environments -- Classical Control

| Environment | Gymnasium | gymnasia | Status |
|-------------|-----------|----------|--------|
| CartPole-v1 | `CartPole-v1` | `envs::classical_control::cartpole` | done |
| MountainCar-v0 | `MountainCar-v0` | `envs::classical_control::mountain_car` | done |
| MountainCarContinuous-v0 | `MountainCarContinuous-v0` | -- | planned |
| Pendulum-v1 | `Pendulum-v1` | -- | planned |
| Acrobot-v1 | `Acrobot-v1` | -- | planned |

## Environments -- Toy Text

| Environment | Gymnasium | gymnasia | Status |
|-------------|-----------|----------|--------|
| FrozenLake | `FrozenLake-v1` | -- | planned |
| Taxi | `Taxi-v3` | -- | planned |
| CliffWalking | `CliffWalking-v0` | -- | planned |
| Blackjack | `Blackjack-v1` | -- | planned |

## Environments -- Box2D

| Environment | Gymnasium | gymnasia | Status |
|-------------|-----------|----------|--------|
| LunarLander | `LunarLander-v3` | -- | not planned |
| BipedalWalker | `BipedalWalker-v3` | -- | not planned |
| CarRacing | `CarRacing-v3` | -- | not planned |

## Environments -- MuJoCo

Not planned. Requires proprietary physics engine.

## Wrappers

| Wrapper | Gymnasium | gymnasia | Status |
|---------|-----------|----------|--------|
| `TimeLimit` | `gymnasium.wrappers.TimeLimit` | -- | planned |
| `AutoResetWrapper` | `gymnasium.wrappers.AutoResetWrapper` | -- | planned |
| `PassiveEnvChecker` | `gymnasium.wrappers.PassiveEnvChecker` | -- | planned |
| `OrderEnforcing` | `gymnasium.wrappers.OrderEnforcing` | -- | planned |
| `RecordEpisodeStatistics` | `gymnasium.wrappers.RecordEpisodeStatistics` | -- | planned |
| `ClipAction` | `gymnasium.wrappers.ClipAction` | -- | planned |
| `RescaleAction` | `gymnasium.wrappers.RescaleAction` | -- | planned |
| `NormalizeObservation` | `gymnasium.wrappers.NormalizeObservation` | -- | planned |
| `NormalizeReward` | `gymnasium.wrappers.NormalizeReward` | -- | planned |
| `FlattenObservation` | `gymnasium.wrappers.FlattenObservation` | -- | planned |
| `FrameStack` | `gymnasium.wrappers.FrameStack` | -- | planned |
| `HumanRendering` | `gymnasium.wrappers.HumanRendering` | -- | planned |
| `RecordVideo` | `gymnasium.wrappers.RecordVideo` | -- | not planned |

## Vector Environments

| Feature | Gymnasium | gymnasia | Status |
|---------|-----------|----------|--------|
| `SyncVectorEnv` | `gymnasium.vector.SyncVectorEnv` | -- | planned |
| `AsyncVectorEnv` | `gymnasium.vector.AsyncVectorEnv` | -- | planned |

## Utilities

| Utility | Gymnasium | gymnasia | Status |
|---------|-----------|----------|--------|
| Seeding / PRNG | `gymnasium.utils.seeding` | `utils::seeding` (PCG64) | done |
| Rendering backend | pygame / SDL2 | macroquad (pure Rust) | done |
| Drawing abstraction | -- | `DrawList` / `DrawCommand` | done |
| `env_checker` | `gymnasium.utils.env_checker` | -- | planned |
| `play` (interactive) | `gymnasium.utils.play` | -- | planned |
