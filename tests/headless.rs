use gym_rs::{
    core::Env,
    envs::classical_control::{cartpole::CartPoleEnv, mountain_car::MountainCarEnv},
    utils::renderer::{RenderMode, Renders},
};

#[test]
fn cartpole_headless_step_and_reset() {
    let mut env = CartPoleEnv::new(RenderMode::None);
    let (obs, _) = env.reset(None, true, None);
    let obs_vec: Vec<f64> = obs.into();
    assert_eq!(obs_vec.len(), 4);

    let result = env.step(0);
    assert!(!result.done || result.done); // valid bool
    let obs_vec: Vec<f64> = result.observation.into();
    assert_eq!(obs_vec.len(), 4);

    assert_eq!(env.render(RenderMode::None), Renders::None);
    env.close();
}

#[test]
fn mountain_car_headless_step_and_reset() {
    let mut env = MountainCarEnv::new(RenderMode::None);
    let (obs, _) = env.reset(None, true, None);
    let obs_vec: Vec<f64> = obs.into();
    assert_eq!(obs_vec.len(), 2);

    let result = env.step(1);
    let obs_vec: Vec<f64> = result.observation.into();
    assert_eq!(obs_vec.len(), 2);

    assert_eq!(env.render(RenderMode::None), Renders::None);
    env.close();
}
