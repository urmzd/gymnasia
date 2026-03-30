use gymnasia::{
    envs::classical_control::cartpole::CartPoleEnv, render::RenderEnv, utils::renderer::RenderMode,
};
use macroquad::prelude::*;
use ordered_float::OrderedFloat;

fn window_conf() -> Conf {
    Conf {
        window_title: "CartPole".to_string(),
        window_width: 600,
        window_height: 400,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let env = CartPoleEnv::new();
    let mut renv = RenderEnv::new(env, RenderMode::Human);
    renv.reset(None, false, None);
    next_frame().await;

    const N: usize = 100;
    let mut rewards = Vec::with_capacity(N);

    for _ in 0..N {
        let mut current_reward = OrderedFloat(0.);

        for _ in 0..475 {
            let action = ::rand::Rng::gen_range(&mut ::rand::thread_rng(), 0..=1);
            let state_reward = renv.step(action);
            current_reward += state_reward.reward;

            next_frame().await;

            if state_reward.terminated {
                break;
            }
        }

        renv.reset(None, false, None);
        rewards.push(current_reward);
    }

    println!("{:?}", rewards);
}
