#![warn(missing_docs, unused_crate_dependencies)]
//! A Rust implementation of OpenAI's Gymnasium environments.
//!
//! # Architecture
//!
//! Unlike Python Gymnasium, gymnasia **separates simulation from rendering**.
//!
//! In Gymnasium, the `Env` class mixes physics with display — `render_mode` is
//! passed at construction and rendering happens inside `step()` / `reset()`.
//! This couples every environment to a graphics backend and complicates headless
//! usage.
//!
//! Gymnasia splits this into three layers:
//!
//! 1. **[`core::Env`]** — pure simulation. `step()` and `reset()` perform
//!    physics only. No feature flags, no rendering code, no graphics imports.
//!
//! 2. **[`core::Renderable`]** — a trait that produces a [`utils::custom::draw::DrawList`]
//!    (a backend-agnostic list of draw commands) from the current state. Always
//!    compiled — `DrawList` has zero graphics dependencies.
//!
//! 3. **[`render::RenderEnv`]** *(requires `render` feature)* — a wrapper that
//!    composes an `Env + Renderable` with a macroquad-backed screen. Call
//!    `renv.step(action)` and it delegates to the inner env then renders.
//!
//! This means the simulation compiles and runs with **zero dependencies** on any
//! graphics library. Rendering is opt-in via `--features render`.
//!
//! # Quick start
//!
//! ```no_run
//! use gymnasia::{core::Env, envs::classical_control::cartpole::CartPoleEnv};
//!
//! let mut env = CartPoleEnv::new();
//! env.reset(None, false, None);
//! let result = env.step(1);
//! ```

/// Contains user-facing interfaces.
pub mod core;
/// Holds implementations of various environments.
pub mod envs;
/// Rendering wrapper for environments (requires `render` feature).
#[cfg(feature = "render")]
pub mod render;
/// Holds structures describing collections of values.
pub mod spaces;
/// Functions, structures and traits designed to reduce complex interactions.
pub mod utils;
