use gymnasia::{
    envs::classical_control::mountain_car::MountainCarEnv, render::RenderEnv,
    utils::renderer::RenderMode,
};
use macroquad::prelude::*;

#[macroquad::main("MountainCar")]
async fn main() {
    let env = MountainCarEnv::new();
    let mut renv = RenderEnv::new(env, RenderMode::Human);
    renv.reset(None, false, None);
    next_frame().await;

    const N: usize = 15;

    for _ in 0..N {
        let mut episode_length = 0;

        loop {
            if episode_length > 200 {
                break;
            }
            let action = ::rand::Rng::gen_range(&mut ::rand::thread_rng(), 0..3);
            let result = renv.step(action);
            episode_length += 1;
            println!("episode_length: {}", episode_length);

            next_frame().await;

            if result.terminated {
                break;
            }
        }

        renv.reset(None, false, None);
    }
}
