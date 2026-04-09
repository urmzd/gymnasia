#![warn(missing_docs)]
//! A Rust implementation of OpenAI's Gymnasium environments.
//!
//! # Architecture
//!
//! Unlike Python Gymnasium, gymnasia **separates simulation from rendering**.
//!
//! The crate is organized into layers:
//!
//! 1. **[`core::Env`]** — pure simulation. `step()` and `reset()` perform
//!    physics only. No feature flags, no rendering code, no graphics imports.
//!
//! 2. **[`core::Renderable`]** — a trait that produces a [`render::draw::DrawList`]
//!    (a backend-agnostic list of draw commands) from the current state. Always
//!    compiled — `DrawList` has zero graphics dependencies.
//!
//! 3. **[`render::RenderEnv`]** *(requires `render` feature)* — a wrapper that
//!    composes an `Env + Renderable` with a macroquad-backed screen. Implements
//!    `Env` so it participates in wrapper chains.
//!
//! 4. **[`wrappers`]** — composable behavior wrappers (`TimeLimit`,
//!    `RecordEpisodeStatistics`, `NormalizeObservation`, etc.) that implement
//!    `Env` via generic delegation.
//!
//! This means the simulation compiles and runs with **zero dependencies** on any
//! graphics library. Rendering is opt-in via `--features render`.
//!
//! # Quick start
//!
//! ```no_run
//! use gymnasia::core::Env;
//! use gymnasia::envs::classical_control::cartpole::CartPoleEnv;
//!
//! let mut env = CartPoleEnv::new();
//! env.reset(None, Default::default());
//! let result = env.step(1);
//! ```

/// Core traits and types: [`Env`](core::Env), [`StepResult`](core::StepResult),
/// [`Flatten`](core::Flatten), [`Renderable`](core::Renderable).
pub mod core;
/// Concrete environment implementations.
pub mod envs;
/// Drawing, rendering backend, and the [`RenderEnv`](render::RenderEnv) wrapper.
pub mod render;
/// Space descriptors: [`Discrete`](spaces::Discrete), [`BoxSpace`](spaces::BoxSpace), etc.
pub mod spaces;
/// Shared utilities: seeding, clip, etc.
pub mod utils;
/// Composable environment wrappers.
pub mod wrappers;
