<p align="center">
  <h1 align="center">gymnasia</h1>
  <p align="center">
    OpenAI Gymnasium environments in pure Rust.
    <br /><br />
    <a href="https://github.com/urmzd/gymnasia/releases">Install</a>
    &middot;
    <a href="https://github.com/urmzd/gymnasia/issues">Report Bug</a>
    &middot;
    <a href="https://crates.io/crates/gymnasia">Crates.io</a>
  </p>
</p>

<p align="center">
  <a href="https://github.com/urmzd/gymnasia/actions/workflows/ci.yml"><img src="https://github.com/urmzd/gymnasia/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/gymnasia"><img src="https://img.shields.io/crates/v/gymnasia" alt="crates.io"></a>
</p>

## Showcase

<table align="center">
  <tr>
    <td align="center">
      <img src="showcase/cartpole.gif" alt="CartPole" width="300" />
      <br />
      <sub><b>CartPole</b></sub>
    </td>
    <td align="center">
      <img src="showcase/mountain-car.gif" alt="MountainCar" width="300" />
      <br />
      <sub><b>MountainCar</b></sub>
    </td>
  </tr>
  <tr>
    <td align="center" colspan="2">
      <img src="showcase/cartpole-headless.gif" alt="CartPole (headless)" width="300" />
      <br />
      <sub><b>CartPole (headless)</b></sub>
    </td>
  </tr>
</table>

## Architecture

Unlike Python Gymnasium, gymnasia **separates simulation from rendering** and
uses **pure-Rust dependencies only** — no C bindings, no SDL2, no system library
installation. This eliminates cross-compilation headaches and keeps binary
bundling simple.

Rendering is abstracted behind a backend-agnostic `DrawList` of simple draw
commands (`FilledPolygon`, `FilledCircle`, `Line`, etc.). The current backend is
[macroquad](https://github.com/not-fl3/macroquad), but swapping it out requires
changing only the feature-gated `Screen` implementation — environment code never
touches a graphics API directly.

| Layer | What it does | Feature gate |
|-------|-------------|--------------|
| `Env` trait | Pure physics — `step()`, `reset()` | Always compiled |
| `Renderable` trait | Produces a `DrawList` (backend-agnostic draw commands) | Always compiled |
| `Screen` | Translates `DrawList` into graphics calls (currently macroquad) | `render` feature |
| `RenderEnv<E>` wrapper | Composes `Env + Renderable` with a `Screen` | `render` feature |

Gymnasium mixes rendering into every `step()` / `reset()` call and requires
`render_mode` at construction. This couples every environment to a graphics
backend and complicates headless usage. Gymnasia keeps the simulation pure and
makes rendering an opt-in wrapper with zero impact on the core library.

## Quick Start

```toml
[dependencies]
gymnasia = "2"
```

Headless by default — no graphics dependencies. To enable rendering:

```toml
[dependencies]
gymnasia = { version = "2", features = ["render"] }
```

### Headless

```rust
use gymnasia::{core::Env, envs::classical_control::cartpole::CartPoleEnv};

let mut env = CartPoleEnv::new();
env.reset(None, false, None);
let result = env.step(1);
```

```bash
cargo run --example=cartpole_headless
```

### With rendering

```rust
use gymnasia::{core::Env, render::RenderEnv, utils::renderer::RenderMode};
use gymnasia::envs::classical_control::cartpole::CartPoleEnv;

#[macroquad::main("CartPole")]
async fn main() {
    let env = CartPoleEnv::new();
    let mut renv = RenderEnv::new(env, RenderMode::Human);
    renv.reset(None, false, None);
    loop {
        let result = renv.step(1);
        renv.next_frame().await;
        if result.terminated { break; }
    }
}
```

```bash
cargo run --example=cartpole --features render
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `render` | No | macroquad-based window rendering and pixel capture |

## Benchmarks

<!-- embed-src src="benches/RESULTS.md" -->
| Benchmark | Time (median) |
|-----------|---------------|
| `cartpole/step` | ~26 ns |
| `cartpole/reset` | ~21 ns |
| `cartpole/episode` | ~248 ns |
| `mountain_car/step` | ~25 ns |
| `mountain_car/reset` | ~15 ns |
| `mountain_car/episode` | ~4.6 µs |

> Apple M3 Pro — `cargo bench` via [Criterion](https://github.com/bheisler/criterion.rs). Run `cargo bench` to reproduce.
<!-- /embed-src -->

## History

Gymnasia is a fork of
[MathisWellmann/gym-rs](https://github.com/MathisWellmann/gym-rs), which is no
longer actively maintained. OpenAI Gym itself has since evolved into
[Gymnasium](https://github.com/Farama-Foundation/Gymnasium) — gymnasia tracks
that direction for Rust.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## Agent Skill

This repo's conventions are available as portable agent skills in [`skills/`](skills/).

## License

Licensed under [Apache 2.0](./LICENSE).
