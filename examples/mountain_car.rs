use gymnasia::{
    core::Env, envs::classical_control::mountain_car::MountainCarEnv, render::RenderEnv,
    utils::renderer::RenderMode,
};
use macroquad::prelude::*;
use rand::{thread_rng, Rng};

#[macroquad::main("MountainCar")]
async fn main() {
    let env = MountainCarEnv::new();
    let mut renv = RenderEnv::new(env, RenderMode::Human);
    renv.reset(None, false, None);

    let mut rng = thread_rng();
    let mut episode_length = 0;

    loop {
        if episode_length > 200 {
            break;
        }
        let action = rng.gen_range(0..3);
        let result = renv.step(action);
        episode_length += 1;
        println!("episode_length: {}", episode_length);

        next_frame().await;

        if result.terminated {
            break;
        }
    }
}
