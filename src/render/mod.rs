/// Drawing primitives for environment visualization.
pub mod draw;
/// Wraps environments with rendering capabilities.
#[cfg(feature = "render")]
mod render_env;
/// Frame collection and render mode logic.
pub mod renderer;
/// Macroquad rendering backend.
#[cfg(feature = "render")]
pub mod screen;

#[cfg(feature = "render")]
pub use render_env::RenderEnv;
