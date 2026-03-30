use gymnasia::{core::Env, envs::classical_control::cartpole::CartPoleEnv};
use ordered_float::OrderedFloat;
use rand::{thread_rng, Rng};

/// Runs cartpole training in headless mode (no rendering).
///
/// This example works without the `render` feature, demonstrating
/// that the core simulation has zero graphics dependencies.
fn main() {
    let mut env = CartPoleEnv::new();
    env.reset(None, false, None);

    const N: usize = 15;
    let mut rewards = Vec::with_capacity(N);

    let mut rng = thread_rng();
    for _ in 0..N {
        let mut current_reward = OrderedFloat(0.);

        for _ in 0..475 {
            let action = rng.gen_range(0..=1);
            let state_reward = env.step(action);
            current_reward += state_reward.reward;

            if state_reward.terminated {
                break;
            }
        }

        env.reset(None, false, None);
        rewards.push(current_reward);
    }

    println!("{:?}", rewards);
}
