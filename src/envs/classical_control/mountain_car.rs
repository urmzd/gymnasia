use derivative::Derivative;
use num_traits::Float;
use ordered_float::OrderedFloat;
use rand::Rng;
use rand_pcg::Pcg64;
use serde::Serialize;

use crate::{
    core::{Env, Flatten, Renderable, StepResult},
    render::draw::{rotate_point, Color, DrawCommand, DrawList},
    spaces::{Bounded, BoxSpace, Discrete, Space},
    utils::{clip::clip, types::O64},
};

/// An implementation of the classical reinforcement learning environment, mountain car.
///
/// The problem involves moving a stochastically placed car on the bottom of a sinusoidal valley
/// to the top of the hill by applying left and right forces to the car. The car is given a reward
/// of `-1` for each step taken by the agent, until a terminal step is reached.
///
/// An episode ends when one of the following conditions occur:
///     1. Termination: The car reaches the goal position.
///     2. Truncation: The episode exceeds 200 steps.
#[derive(Serialize, Derivative, Clone)]
#[derivative(Debug)]
pub struct MountainCarEnv {
    /// The minimum position the car can be spawned at.
    pub min_position: O64,
    /// The maximum position the cart can be spawned at.
    pub max_position: O64,
    /// The max speed the car can reach.
    pub max_speed: O64,
    /// The position on the map, where when passed, an episode can be considered terminated.
    pub goal_position: O64,
    /// The velocity at which an episode can be considered terminated.
    pub goal_velocity: O64,

    /// The force of the cart.
    pub force: O64,
    /// The gravity constant applied to the environment.
    pub gravity: O64,

    /// The set of actions which can be taken.
    pub action_space: Discrete,
    /// The range of values that can be observed.
    pub observation_space: BoxSpace<MountainCarObservation>,

    /// The state of the environment.
    pub state: MountainCarObservation,

    #[serde(skip_serializing)]
    #[derivative(Debug = "ignore")]
    rand_random: Pcg64,
}

/// Screen dimensions for MountainCar rendering.
const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 400;

fn height(x: f64) -> f64 {
    (3.0 * x).sin() * 0.45 + 0.55
}

impl MountainCarEnv {
    /// Generates an instance of the mountain car environment using the defaults provided in the
    /// paper.
    pub fn new() -> Self {
        let (mut rng, _) = crate::utils::seeding::rand_random(None);

        let min_position = OrderedFloat(-1.2);
        let max_position = OrderedFloat(0.6);
        let max_speed = OrderedFloat(0.07);
        let goal_position = OrderedFloat(0.5);
        let goal_velocity = OrderedFloat(0.);

        let force = OrderedFloat(0.001);
        let gravity = OrderedFloat(0.0025);

        let low = MountainCarObservation {
            position: min_position,
            velocity: -max_speed,
        };
        let high = MountainCarObservation {
            position: max_position,
            velocity: max_speed,
        };

        let state = MountainCarObservation::sample_default(&mut rng);

        let action_space = Discrete::new(3);
        let observation_space = BoxSpace::new(low, high);

        Self {
            min_position,
            max_position,
            max_speed,
            goal_position,
            goal_velocity,
            force,
            gravity,
            action_space,
            observation_space,
            state,
            rand_random: rng,
        }
    }
}

impl Default for MountainCarEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for MountainCarEnv {
    fn draw_list(&self) -> DrawList {
        let mut dl = DrawList::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        let world_width = self.max_position - self.min_position;
        let scale = OrderedFloat(SCREEN_WIDTH as f64) / world_width;
        let carwidth = 40;
        let carheight = 20;
        let pos = self.state.position;

        dl.push(DrawCommand::Clear(Color::WHITE));

        // Terrain line
        let terrain_points: Vec<(f32, f32)> = (0..100)
            .map(|i| {
                let x_val = (((self.max_position - self.min_position) / 100.) * i as f64
                    + self.min_position)
                    .into_inner();
                let screen_x = (OrderedFloat(x_val) - self.min_position) * scale;
                let screen_y = OrderedFloat(height(x_val)) * scale;
                (screen_x.into_inner() as f32, screen_y.into_inner() as f32)
            })
            .collect();

        dl.push(DrawCommand::Polyline {
            points: terrain_points,
            color: Color::BLACK,
        });

        // Car body
        let clearance = 10f64;
        let (l, r, t, b) = (-carwidth / 2, carwidth / 2, carheight, 0);
        let desired_angle = (OrderedFloat(3.) * pos).cos().into_inner();

        let car_verts: Vec<(f32, f32)> = [(l, b), (l, t), (r, t), (r, b)]
            .iter()
            .map(|(x, y)| {
                let (rx, ry) = rotate_point(*x as f64, *y as f64, desired_angle);
                let new_x = OrderedFloat(rx) + (pos - self.min_position) * scale;
                let new_y =
                    OrderedFloat(ry) + clearance + OrderedFloat(height(pos.into_inner())) * scale;
                (new_x.into_inner() as f32, new_y.into_inner() as f32)
            })
            .collect();

        dl.push(DrawCommand::FilledPolygon {
            vertices: car_verts,
            color: Color::BLACK,
        });

        // Wheels
        for wheel_offset in [carwidth as f64 / 4., -(carwidth as f64 / 4.)] {
            let (rx, ry) = rotate_point(wheel_offset, 0., desired_angle);
            let wheel_x =
                (OrderedFloat(rx) + (pos - self.min_position) * scale).into_inner() as f32;
            let wheel_y =
                (OrderedFloat(ry) + clearance + OrderedFloat(height(pos.into_inner())) * scale)
                    .into_inner() as f32;
            let rad = (carheight as f64 / 2.5).floor() as f32;

            dl.push(DrawCommand::FilledCircle {
                x: wheel_x,
                y: wheel_y,
                radius: rad,
                color: Color::rgb(128, 128, 128),
            });
        }

        // Flag
        let flagx = ((self.goal_position - self.min_position) * scale).into_inner() as f32;
        let flagy1 =
            (OrderedFloat(height(self.goal_position.into_inner())) * scale).into_inner() as f32;
        let flagy2 = flagy1 + 50.0;

        dl.push(DrawCommand::Line {
            x1: flagx,
            y1: flagy1,
            x2: flagx,
            y2: flagy2,
            color: Color::BLACK,
        });

        dl.push(DrawCommand::FilledPolygon {
            vertices: vec![
                (flagx, flagy2),
                (flagx, flagy2 - 10.0),
                (flagx + 25.0, flagy2 - 5.0),
            ],
            color: Color::rgb(204, 204, 0),
        });

        dl
    }

    fn render_fps(&self) -> u32 {
        30
    }
}

/// Utility structure intended to reduce confusion around meaning of properties.
#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MountainCarObservation {
    /// The position the car exists on the mountain.
    pub position: O64,
    /// The velocity the car is travelling at.
    pub velocity: O64,
}

impl MountainCarObservation {
    /// Sample from default initial bounds (position in [-0.6, -0.4], velocity = 0).
    fn sample_default<R: Rng>(rng: &mut R) -> Self {
        let low = MountainCarObservation {
            position: OrderedFloat(-0.6),
            velocity: OrderedFloat(0.),
        };
        let high = MountainCarObservation {
            position: OrderedFloat(-0.4),
            velocity: OrderedFloat(0.),
        };
        MountainCarObservation::sample_uniform(rng, &low, &high)
    }
}

impl Bounded for MountainCarObservation {
    fn in_bounds(value: &Self, low: &Self, high: &Self) -> bool {
        value.position >= low.position
            && value.position <= high.position
            && value.velocity >= low.velocity
            && value.velocity <= high.velocity
    }

    fn sample_uniform<R: Rng>(rng: &mut R, low: &Self, high: &Self) -> Self {
        MountainCarObservation {
            position: OrderedFloat(
                rng.gen_range(low.position.into_inner()..=high.position.into_inner()),
            ),
            velocity: OrderedFloat(
                rng.gen_range(low.velocity.into_inner()..=high.velocity.into_inner()),
            ),
        }
    }
}

impl Flatten for MountainCarObservation {
    fn flat_dim() -> usize {
        2
    }

    fn flatten(&self) -> Vec<f64> {
        vec![self.position.into_inner(), self.velocity.into_inner()]
    }

    fn unflatten(flat: &[f64]) -> Self {
        assert_eq!(flat.len(), 2);
        MountainCarObservation {
            position: OrderedFloat(flat[0]),
            velocity: OrderedFloat(flat[1]),
        }
    }
}

impl Env for MountainCarEnv {
    type Action = usize;
    type Observation = MountainCarObservation;
    type ActionSpace = Discrete;
    type ObservationSpace = BoxSpace<MountainCarObservation>;
    type ResetOptions = Option<BoxSpace<MountainCarObservation>>;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        assert!(
            self.action_space.contains(&(action as i64)),
            "{} (usize) invalid",
            action
        );

        let mut position = self.state.position;
        let mut velocity = self.state.velocity;

        velocity += OrderedFloat((action as f64) - 1.) * self.force
            + (OrderedFloat(3.) * position).cos() * (-self.gravity);
        velocity = clip(velocity, -self.max_speed, self.max_speed);

        position += velocity;
        position = clip(position, self.min_position, self.max_position);

        if position == self.min_position && velocity < OrderedFloat(0.) {
            velocity = OrderedFloat(0.);
        }

        let terminated = position >= self.goal_position && velocity >= self.goal_velocity;

        self.state = MountainCarObservation { position, velocity };

        StepResult {
            observation: self.state,
            reward: -1.0,
            terminated,
            truncated: false,
        }
    }

    fn reset(
        &mut self,
        seed: Option<u64>,
        options: Self::ResetOptions,
    ) -> Self::Observation {
        let (rand_random, _) = crate::utils::seeding::rand_random(seed);
        self.rand_random = rand_random;

        self.state = if let Some(bounds) = options {
            MountainCarObservation::sample_uniform(
                &mut self.rand_random,
                &bounds.low,
                &bounds.high,
            )
        } else {
            MountainCarObservation::sample_default(&mut self.rand_random)
        };

        self.state
    }

    fn action_space(&self) -> &Self::ActionSpace {
        &self.action_space
    }

    fn observation_space(&self) -> &Self::ObservationSpace {
        &self.observation_space
    }
}
