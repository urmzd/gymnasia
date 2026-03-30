use gymnasia::{
    core::Env,
    envs::classical_control::{cartpole::CartPoleEnv, mountain_car::MountainCarEnv},
};

#[test]
fn cartpole_headless_step_and_reset() {
    let mut env = CartPoleEnv::new();
    let (obs, _) = env.reset(None, true, None);
    let obs_vec: Vec<f64> = obs.into();
    assert_eq!(obs_vec.len(), 4);

    let result = env.step(0);
    assert!(result.reward >= ordered_float::OrderedFloat(0.));
    let obs_vec: Vec<f64> = result.observation.into();
    assert_eq!(obs_vec.len(), 4);
}

#[test]
fn mountain_car_headless_step_and_reset() {
    let mut env = MountainCarEnv::new();
    let (obs, _) = env.reset(None, true, None);
    let obs_vec: Vec<f64> = obs.into();
    assert_eq!(obs_vec.len(), 2);

    let result = env.step(1);
    let obs_vec: Vec<f64> = result.observation.into();
    assert_eq!(obs_vec.len(), 2);
}
