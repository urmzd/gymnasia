use gymnasia::{
    core::{Env, Flatten},
    envs::classical_control::{
        cartpole::{CartPoleEnv, CartPoleObservation},
        mountain_car::{MountainCarEnv, MountainCarObservation},
    },
};

#[test]
fn cartpole_headless_step_and_reset() {
    let mut env = CartPoleEnv::new();
    let obs = env.reset(None, None);
    let flat = obs.flatten();
    assert_eq!(flat.len(), CartPoleObservation::flat_dim());

    let result = env.step(0_i64);
    assert!(result.reward >= 0.0);
    assert_eq!(result.observation.flatten().len(), 4);
}

#[test]
fn mountain_car_headless_step_and_reset() {
    let mut env = MountainCarEnv::new();
    let obs = env.reset(None, None);
    let flat = obs.flatten();
    assert_eq!(flat.len(), MountainCarObservation::flat_dim());

    let result = env.step(1_i64);
    assert_eq!(result.observation.flatten().len(), 2);
}
