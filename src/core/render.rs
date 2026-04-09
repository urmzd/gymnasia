use crate::render::draw::DrawList;

/// Environments that can produce a visual representation of their state.
///
/// [`DrawList`] is pure data with no rendering-backend dependency, so this
/// trait is always available regardless of feature flags.
pub trait Renderable {
    /// Produce draw commands representing the current visual state.
    /// The returned [`DrawList`] includes logical canvas width/height.
    fn draw_list(&self) -> DrawList;

    /// Target frames per second for human-mode display.
    fn render_fps(&self) -> u32 {
        30
    }
}
