# Roadmap

Feature parity mapping between **[gymnasia](https://github.com/urmzd/gymnasia)** (Rust) and **[Gymnasium](https://github.com/Farama-Foundation/Gymnasium)** (Python).

Legend: done / partial / planned / not planned

---

## Core API

| Feature | Gymnasium | gymnasia | Status |
|---------|-----------|----------|--------|
| `Env` trait (step, reset) | `gymnasium.Env` | `core::Env` | done |
| `StepResult` (obs, reward, terminated, truncated) | tuple return | `core::StepResult` | done |
| `Renderable` trait | Built into `Env` | `core::Renderable` | done |
| `Flatten` trait (ML interop) | numpy arrays | `core::Flatten` | done |
| `RenderEnv` wrapper (implements `Env`) | `render_mode` param | `render::RenderEnv` | done |
| `close()` method | `Env.close()` | `Env::close()` | done |
| `Wrapper` trait | `gymnasium.Wrapper` | `wrappers::Wrapper` | done |
| `delegate_env!` macro | class inheritance | `delegate_env!` | done |
| Environment registry (`gymnasium.make()`) | `gymnasium.make` | -- | planned |
| Spec / EnvSpec metadata | `gymnasium.envs.registration` | -- | planned |

## Spaces

| Space | Gymnasium | gymnasia | Status |
|-------|-----------|----------|--------|
| `Box` (continuous bounded) | `gymnasium.spaces.Box` | `spaces::BoxSpace<B: Bounded>` | done |
| `Bounded` trait (user-extensible bounds) | numpy-only | `spaces::Bounded` | done |
| `Tensor` (flat array + shape) | numpy arrays | `spaces::Tensor` | done |
| `Discrete` (with start offset) | `gymnasium.spaces.Discrete` | `spaces::Discrete` | done |
| Action masking (sample w/ mask) | `spaces.sample(mask=)` | `SampleSpace` trait | done |
| `MultiDiscrete` | `gymnasium.spaces.MultiDiscrete` | `spaces::MultiDiscrete` | done |
| `MultiBinary` | `gymnasium.spaces.MultiBinary` | `spaces::MultiBinary` | done |
| `Tuple` | `gymnasium.spaces.Tuple` | -- | planned |
| `Dict` | `gymnasium.spaces.Dict` | -- | planned |
| `Text` | `gymnasium.spaces.Text` | -- | planned |
| `Graph` | `gymnasium.spaces.Graph` | -- | planned |
| `Sequence` | `gymnasium.spaces.Sequence` | -- | planned |
| `OneOf` | `gymnasium.spaces.OneOf` | -- | planned |
| `spaces.flatdim` / `flatten` | `gymnasium.spaces.utils` | `Flatten` trait | done |

## Wrappers

| Wrapper | Gymnasium | gymnasia | Status |
|---------|-----------|----------|--------|
| `TimeLimit` | `gymnasium.wrappers.TimeLimit` | `wrappers::TimeLimit` | done |
| `Autoreset` | `gymnasium.wrappers.Autoreset` | `wrappers::Autoreset` | done |
| `OrderEnforcing` | `gymnasium.wrappers.OrderEnforcing` | `wrappers::OrderEnforcing` | done |
| `RecordEpisodeStatistics` | `gymnasium.wrappers.RecordEpisodeStatistics` | `wrappers::RecordEpisodeStatistics` | done |
| `ClipAction` | `gymnasium.wrappers.ClipAction` | `wrappers::ClipAction` | done |
| `RescaleAction` | `gymnasium.wrappers.RescaleAction` | `wrappers::RescaleAction` | done |
| `TransformAction` | `gymnasium.wrappers.TransformAction` | `wrappers::TransformAction` | done |
| `FlattenObservation` | `gymnasium.wrappers.FlattenObservation` | `wrappers::FlattenObservation` | done |
| `NormalizeObservation` | `gymnasium.wrappers.NormalizeObservation` | `wrappers::NormalizeObservation` | done |
| `TransformObservation` | `gymnasium.wrappers.TransformObservation` | `wrappers::TransformObservation` | done |
| `NormalizeReward` | `gymnasium.wrappers.NormalizeReward` | `wrappers::NormalizeReward` | done |
| `ClipReward` | `gymnasium.wrappers.ClipReward` | `wrappers::ClipReward` | done |
| `TransformReward` | `gymnasium.wrappers.TransformReward` | `wrappers::TransformReward` | done |
| `PassiveEnvChecker` | `gymnasium.wrappers.PassiveEnvChecker` | -- | planned |
| `RescaleObservation` | `gymnasium.wrappers.RescaleObservation` | -- | planned |
| `TimeAwareObservation` | `gymnasium.wrappers.TimeAwareObservation` | -- | planned |
| `FrameStackObservation` | `gymnasium.wrappers.FrameStackObservation` | -- | planned |
| `HumanRendering` | `gymnasium.wrappers.HumanRendering` | -- | planned |
| `RecordVideo` | `gymnasium.wrappers.RecordVideo` | -- | not planned |

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
| Drawing abstraction | -- | `render::draw::DrawList` | done |
| `env_checker` | `gymnasium.utils.env_checker` | -- | planned |
| `play` (interactive) | `gymnasium.utils.play` | -- | planned |
