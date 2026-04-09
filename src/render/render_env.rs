use crate::core::{Env, Renderable, StepResult};

use super::{
    renderer::{RenderMode, Renderer, Renders},
    screen::Screen,
};

/// Wraps an [`Env`] with rendering capabilities.
///
/// The inner environment runs pure simulation; this wrapper adds visualization
/// by calling [`Renderable::draw_list`] after each step/reset and feeding the
/// result to a macroquad-backed [`Screen`].
///
/// `RenderEnv` implements [`Env`], so it can participate in wrapper chains.
pub struct RenderEnv<E: Env + Renderable> {
    env: E,
    screen: Screen,
    collector: Renderer,
    mode: RenderMode,
}

impl<E: Env + Renderable> RenderEnv<E> {
    /// Wrap an environment with rendering.
    pub fn new(env: E, mode: RenderMode) -> Self {
        let screen = Screen;
        let collector = Renderer::new(mode, None, None);
        Self {
            env,
            screen,
            collector,
            mode,
        }
    }

    /// Access the inner environment.
    pub fn inner(&self) -> &E {
        &self.env
    }

    /// Access the inner environment mutably.
    pub fn inner_mut(&mut self) -> &mut E {
        &mut self.env
    }

    /// Unwrap and return the inner environment.
    pub fn into_inner(self) -> E {
        self.env
    }

    /// The current render mode.
    pub fn mode(&self) -> RenderMode {
        self.mode
    }

    /// Retrieve collected frames (for [`RenderMode::RgbArray`]).
    pub fn get_renders(&mut self) -> Renders {
        let screen = &mut self.screen;
        let env = &self.env;
        self.collector
            .get_renders(&mut |mode| screen.execute(&env.draw_list(), mode))
    }

    /// Advance to the next frame (delegates to the rendering backend).
    pub async fn next_frame(&self) {
        self.screen.next_frame().await;
    }

    fn render_current_state(&mut self) {
        let screen = &mut self.screen;
        let env = &self.env;
        self.collector
            .render_step(&mut |mode| screen.execute(&env.draw_list(), mode));
    }
}

impl<E: Env + Renderable> Env for RenderEnv<E> {
    type Action = E::Action;
    type Observation = E::Observation;
    type ActionSpace = E::ActionSpace;
    type ObservationSpace = E::ObservationSpace;
    type ResetOptions = E::ResetOptions;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        let result = self.env.step(action);
        self.render_current_state();
        result
    }

    fn reset(&mut self, seed: Option<u64>, options: Self::ResetOptions) -> Self::Observation {
        let obs = self.env.reset(seed, options);
        self.collector.reset();
        self.render_current_state();
        obs
    }

    fn action_space(&self) -> &Self::ActionSpace {
        self.env.action_space()
    }

    fn observation_space(&self) -> &Self::ObservationSpace {
        self.env.observation_space()
    }
}
