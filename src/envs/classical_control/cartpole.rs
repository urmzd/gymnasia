use std::f64::consts::PI;

use log::warn;
use num_traits::Float;
use ordered_float::OrderedFloat;
use rand::Rng;
use rand_pcg::Pcg64;
use serde::Serialize;

use crate::{
    core::{Env, Flatten, Renderable, StepResult},
    render::draw::{rotate_point, Color, DrawCommand, DrawList},
    spaces::{Bounded, BoxSpace, Discrete, Space},
    utils::types::O64,
};

/// An environment which implements the cart pole problem described in
/// [Neuronlike adaptive elements that can solve difficult learning control
/// problems](https://ieeexplore.ieee.org/document/6313077).
///
/// The problem involves applying the correct forces onto a cart with a pole hinged onto it,
/// in order to ensure the pole remains within the preconfigured regions.
/// The agent starts by being assigned random values between (-0.05, 0.05) for all
/// fields available in the state structure. The agent is rewarded '+1' for every step taken until the episode ends.
///
/// The episode ends when any of the following conditions occur:
///
/// 1. Termination: [`CartPoleObservation::theta`] is greater than +/-12.0 (pole has fallen).
/// 2. Termination: [`CartPoleObservation::x`] is greater than +/-2.4 (cart is outside bounds).
/// 3. Truncation: Episode length is greater than 500.
#[derive(Debug, Clone, Serialize)]
pub struct CartPoleEnv {
    /// The available actions that can be taken.
    pub action_space: Discrete,
    /// The range of values that can be observed.
    pub observation_space: BoxSpace<CartPoleObservation>,
    /// The current state of the environment.
    pub state: CartPoleObservation,
    /// The gravity constant applied to the environment.
    pub gravity: O64,
    /// The mass of the cart.
    pub masscart: O64,
    /// The mass of the pole.
    pub masspole: O64,
    /// Half the length of the pole.
    pub length: O64,
    /// The default force applied to the pole.
    pub force_mag: O64,
    /// The number of seconds between state updates.
    pub tau: O64,
    /// The type of integration done on the differential equations found in the paper.
    pub kinematics_integrator: KinematicsIntegrator,
    /// The angle that the pole can lean to before an episode is considered terminated.
    pub theta_threshold_radians: O64,
    /// The x value that the cart can be at before an episode is considered terminated.
    pub x_threshold: O64,
    /// The number of steps taken after the episode was terminated.
    pub steps_beyond_terminated: Option<usize>,
    #[serde(skip_serializing)]
    rand_random: Pcg64,
}

/// Screen dimensions for CartPole rendering.
const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 400;

impl CartPoleEnv {
    /// Creates a cart pole environment using defaults from the paper.
    pub fn new() -> Self {
        let (mut rand_random, _) = crate::utils::seeding::rand_random(None);

        let gravity = OrderedFloat(9.8);
        let masscart = OrderedFloat(1.0);
        let masspole = OrderedFloat(0.1);
        let length = OrderedFloat(0.5);
        let force_mag = OrderedFloat(10.0);
        let tau = OrderedFloat(0.02);
        let kinematics_integrator = KinematicsIntegrator::Euler;

        let theta_threshold_radians = OrderedFloat(12. * 2. * PI / 360.);
        let x_threshold = OrderedFloat(2.4);

        let high = CartPoleObservation {
            x: x_threshold * 2.,
            x_dot: OrderedFloat(f64::INFINITY),
            theta: theta_threshold_radians * 2.,
            theta_dot: OrderedFloat(f64::INFINITY),
        };
        let low = CartPoleObservation {
            x: -high.x,
            x_dot: -high.x_dot,
            theta: -high.theta,
            theta_dot: -high.theta_dot,
        };

        let action_space = Discrete::new(2);
        let observation_space = BoxSpace::new(low, high);
        let state = CartPoleObservation::sample_default(&mut rand_random);

        Self {
            gravity,
            masscart,
            masspole,
            length,
            force_mag,
            tau,
            kinematics_integrator,
            theta_threshold_radians,
            x_threshold,
            action_space,
            observation_space,
            state,
            rand_random,
            steps_beyond_terminated: None,
        }
    }

    fn total_mass(&self) -> O64 {
        self.masspole + self.masscart
    }

    fn polemass_length(&self) -> O64 {
        self.masspole * self.length
    }
}

impl Default for CartPoleEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for CartPoleEnv {
    fn draw_list(&self) -> DrawList {
        let mut dl = DrawList::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        let world_width = self.x_threshold * 2.;
        let scale = OrderedFloat(SCREEN_WIDTH as f64) / world_width;
        let polewidth: O64 = OrderedFloat(10.);
        let polelen = scale * 2. * self.length;
        let cartwidth = OrderedFloat(50.);
        let cartheight = OrderedFloat(30.);

        dl.push(DrawCommand::Clear(Color::WHITE));

        // Cart
        let (l, r, t, b) = (
            -cartwidth / OrderedFloat(2f64),
            cartwidth / OrderedFloat(2f64),
            cartheight / OrderedFloat(2f64),
            -cartheight / OrderedFloat(2f64),
        );

        let axleoffset = cartheight / OrderedFloat(4.0);
        let cartx = self.state.x * scale + OrderedFloat(SCREEN_WIDTH as f64) / OrderedFloat(2.0);
        let carty = OrderedFloat(100.);

        let cart_verts: Vec<(f32, f32)> = [(l, b), (l, t), (r, t), (r, b)]
            .iter()
            .map(|(x, y)| {
                (
                    (*x + cartx).into_inner() as f32,
                    (*y + carty).into_inner() as f32,
                )
            })
            .collect();

        dl.push(DrawCommand::FilledPolygon {
            vertices: cart_verts,
            color: Color::BLACK,
        });

        // Pole
        let (pl, pr, pt, pb) = (
            -polewidth / OrderedFloat(2f64),
            polewidth / OrderedFloat(2f64),
            polelen - polewidth / OrderedFloat(2f64),
            -polewidth / OrderedFloat(2f64),
        );

        let pole_verts: Vec<(f32, f32)> = [(pl, pb), (pl, pt), (pr, pt), (pr, pb)]
            .iter()
            .map(|(x, y)| {
                let (rx, ry) = rotate_point(
                    x.into_inner(),
                    y.into_inner(),
                    -self.state.theta.into_inner(),
                );
                (
                    (rx + cartx.into_inner()) as f32,
                    (ry + (carty + axleoffset).into_inner()) as f32,
                )
            })
            .collect();

        dl.push(DrawCommand::FilledPolygon {
            vertices: pole_verts,
            color: Color::rgb(202, 152, 101),
        });

        // Axle
        dl.push(DrawCommand::FilledCircle {
            x: cartx.into_inner() as f32,
            y: (carty + axleoffset).into_inner() as f32,
            radius: (polewidth / OrderedFloat(2f64)).into_inner() as f32,
            color: Color::rgb(129, 132, 203),
        });

        // Ground line
        dl.push(DrawCommand::Line {
            x1: 0.0,
            y1: carty.into_inner() as f32,
            x2: SCREEN_WIDTH as f32,
            y2: carty.into_inner() as f32,
            color: Color::BLACK,
        });

        dl
    }

    fn render_fps(&self) -> u32 {
        50
    }
}

/// Defines the state found in the cart pole environment.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct CartPoleObservation {
    /// Cart position.
    pub x: O64,
    /// Cart velocity.
    pub x_dot: O64,
    /// Pole angle (radians).
    pub theta: O64,
    /// Pole angular velocity.
    pub theta_dot: O64,
}

impl CartPoleObservation {
    /// Sample from default initial bounds `(-0.05, 0.05)`.
    fn sample_default<R: Rng>(rng: &mut R) -> Self {
        let bound = 0.05;
        let low = CartPoleObservation {
            x: OrderedFloat(-bound),
            x_dot: OrderedFloat(-bound),
            theta: OrderedFloat(-bound),
            theta_dot: OrderedFloat(-bound),
        };
        let high = CartPoleObservation {
            x: OrderedFloat(bound),
            x_dot: OrderedFloat(bound),
            theta: OrderedFloat(bound),
            theta_dot: OrderedFloat(bound),
        };
        CartPoleObservation::sample_uniform(rng, &low, &high)
    }
}

impl Bounded for CartPoleObservation {
    fn in_bounds(value: &Self, low: &Self, high: &Self) -> bool {
        value.x >= low.x
            && value.x <= high.x
            && value.x_dot >= low.x_dot
            && value.x_dot <= high.x_dot
            && value.theta >= low.theta
            && value.theta <= high.theta
            && value.theta_dot >= low.theta_dot
            && value.theta_dot <= high.theta_dot
    }

    fn sample_uniform<R: Rng>(rng: &mut R, low: &Self, high: &Self) -> Self {
        CartPoleObservation {
            x: OrderedFloat(rng.gen_range(low.x.into_inner()..=high.x.into_inner())),
            x_dot: OrderedFloat(rng.gen_range(low.x_dot.into_inner()..=high.x_dot.into_inner())),
            theta: OrderedFloat(rng.gen_range(low.theta.into_inner()..=high.theta.into_inner())),
            theta_dot: OrderedFloat(
                rng.gen_range(low.theta_dot.into_inner()..=high.theta_dot.into_inner()),
            ),
        }
    }
}

impl Flatten for CartPoleObservation {
    fn flat_dim() -> usize {
        4
    }

    fn flatten(&self) -> Vec<f64> {
        vec![
            self.x.into_inner(),
            self.x_dot.into_inner(),
            self.theta.into_inner(),
            self.theta_dot.into_inner(),
        ]
    }

    fn unflatten(flat: &[f64]) -> Self {
        assert_eq!(flat.len(), 4);
        CartPoleObservation {
            x: OrderedFloat(flat[0]),
            x_dot: OrderedFloat(flat[1]),
            theta: OrderedFloat(flat[2]),
            theta_dot: OrderedFloat(flat[3]),
        }
    }
}

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
/// Describes the available types of integration on cartpole equations.
pub enum KinematicsIntegrator {
    /// Euler integration.
    Euler,
    /// Semi-implicit Euler integration.
    Other,
}

impl Env for CartPoleEnv {
    type Action = usize;
    type Observation = CartPoleObservation;
    type ActionSpace = Discrete;
    type ObservationSpace = BoxSpace<CartPoleObservation>;
    type ResetOptions = Option<BoxSpace<CartPoleObservation>>;

    fn step(&mut self, action: Self::Action) -> StepResult<Self::Observation> {
        assert!(
            self.action_space.contains(&(action as i64)),
            "{} usize invalid",
            action
        );

        let CartPoleObservation {
            mut x,
            mut x_dot,
            mut theta,
            mut theta_dot,
        } = self.state;
        let force = if action == 1 {
            self.force_mag
        } else {
            -self.force_mag
        };

        let costheta = theta.cos();
        let sintheta = theta.sin();

        let temp = (force + self.polemass_length() * theta_dot.powf(OrderedFloat(2.)) * sintheta)
            / self.total_mass();
        let thetaacc = (self.gravity * sintheta - costheta * temp)
            / (self.length
                * (OrderedFloat(4.0 / 3.0)
                    - self.masspole * costheta.powf(OrderedFloat(2.)) / self.total_mass()));
        let xacc = temp - self.polemass_length() * thetaacc * costheta / self.total_mass();

        if self.kinematics_integrator == KinematicsIntegrator::Euler {
            x += self.tau * x_dot;
            x_dot += self.tau * xacc;
            theta += self.tau * theta_dot;
            theta_dot += self.tau * thetaacc;
        } else {
            x_dot += self.tau * xacc;
            x += self.tau * x_dot;
            theta_dot += self.tau * thetaacc;
            theta += self.tau * theta_dot;
        }

        self.state = CartPoleObservation {
            x,
            x_dot,
            theta_dot,
            theta,
        };

        let terminated = x < -self.x_threshold
            || x > self.x_threshold
            || theta < -self.theta_threshold_radians
            || theta > self.theta_threshold_radians;

        let reward = if !terminated {
            1.0
        } else if self.steps_beyond_terminated.is_none() {
            self.steps_beyond_terminated = Some(0);
            1.0
        } else {
            warn!("Calling step after termination may result in undefined behaviour. Consider reseting.");
            self.steps_beyond_terminated = self.steps_beyond_terminated.map(|step| step + 1);
            0.0
        };

        StepResult {
            observation: self.state,
            reward,
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
            CartPoleObservation::sample_uniform(&mut self.rand_random, &bounds.low, &bounds.high)
        } else {
            CartPoleObservation::sample_default(&mut self.rand_random)
        };
        self.steps_beyond_terminated = None;

        self.state
    }

    fn action_space(&self) -> &Self::ActionSpace {
        &self.action_space
    }

    fn observation_space(&self) -> &Self::ObservationSpace {
        &self.observation_space
    }
}
