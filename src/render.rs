use crate::{
    core::{ActionReward, Env, Renderable},
    spaces::BoxR,
    utils::{
        custom::{screen::Screen, traits::Sample},
        renderer::{RenderMode, Renderer, Renders},
    },
};

/// Wraps an [`Env`] with rendering capabilities.
///
/// The inner environment runs pure simulation; this wrapper adds visualization
/// by calling [`Renderable::draw_list`] after each step/reset and feeding the
/// result to a macroquad-backed [`Screen`].
pub struct RenderEnv<E: Env + Renderable>
where
    E::Observation: Sample + Into<Vec<f64>>,
{
    env: E,
    screen: Screen,
    collector: Renderer,
    mode: RenderMode,
}

impl<E: Env + Renderable> RenderEnv<E>
where
    E::Observation: Sample + Into<Vec<f64>>,
{
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
    pub fn env(&self) -> &E {
        &self.env
    }

    /// Access the inner environment mutably.
    pub fn env_mut(&mut self) -> &mut E {
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

    /// Step the simulation and render the new state.
    pub fn step(&mut self, action: E::Action) -> ActionReward<E::Observation, E::Info> {
        let result = self.env.step(action);
        self.render_current_state();
        result
    }

    /// Reset the simulation and render the initial state.
    pub fn reset(
        &mut self,
        seed: Option<u64>,
        return_info: bool,
        options: Option<BoxR<E::Observation>>,
    ) -> (E::Observation, Option<E::ResetInfo>) {
        let result = self.env.reset(seed, return_info, options);
        self.collector.reset();
        self.render_current_state();
        result
    }

    /// Retrieve collected frames (for [`RenderMode::RgbArray`]).
    pub fn get_renders(&mut self) -> Renders {
        let screen = &mut self.screen;
        let env = &self.env;
        self.collector
            .get_renders(&mut |mode| screen.execute(&env.draw_list(), mode))
    }

    fn render_current_state(&mut self) {
        let screen = &mut self.screen;
        let env = &self.env;
        self.collector
            .render_step(&mut |mode| screen.execute(&env.draw_list(), mode));
    }
}

/// Run an interactive rendering loop with `next_frame().await` handled for you.
///
/// Returns accumulated [`ActionReward`] results. Stops on termination or after
/// `max_steps`.
pub async fn run_visual<E, F>(
    render_env: &mut RenderEnv<E>,
    mut step_fn: F,
    max_steps: usize,
) -> Vec<ActionReward<E::Observation, E::Info>>
where
    E: Env + Renderable,
    E::Observation: Sample + Into<Vec<f64>>,
    F: FnMut(&E) -> E::Action,
{
    let mut results = Vec::new();
    for _ in 0..max_steps {
        let action = step_fn(render_env.env());
        let result = render_env.step(action);
        let done = result.terminated || result.truncated;
        results.push(result);
        macroquad::prelude::next_frame().await;
        if done {
            break;
        }
    }
    results
}
