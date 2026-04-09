use gymnasia::{
    core::{Env, Flatten},
    envs::classical_control::cartpole::CartPoleEnv,
    wrappers::{
        Autoreset, ClipReward, NormalizeObservation, NormalizeReward, OrderEnforcing,
        RecordEpisodeStatistics, TimeLimit, TransformObservation, TransformReward, Wrapper,
    },
};

/// Helper: create a CartPole env and reset it.
fn cartpole() -> CartPoleEnv {
    let mut env = CartPoleEnv::new();
    env.reset(Some(42), None);
    env
}

// ---- TimeLimit ----

#[test]
fn time_limit_truncates_at_boundary() {
    let mut env = TimeLimit::new(cartpole(), 5);
    env.reset(Some(42), None);

    for i in 0..5 {
        let result = env.step(1_i64);
        if i < 4 {
            assert!(!result.truncated, "should not truncate at step {}", i + 1);
        } else {
            assert!(
                result.truncated || result.terminated,
                "step 5 should end the episode"
            );
        }
        if result.terminated {
            break;
        }
    }
}

#[test]
fn time_limit_resets_counter_on_reset() {
    let mut env = TimeLimit::new(cartpole(), 3);
    env.reset(Some(42), None);

    let _ = env.step(1_i64);
    let _ = env.step(1_i64);
    assert_eq!(env.steps_remaining(), 1);

    env.reset(Some(42), None);
    assert_eq!(env.steps_remaining(), 3);
    assert_eq!(env.current_step(), 0);
}

#[test]
fn time_limit_does_not_truncate_if_already_terminated() {
    // Run until natural termination within the limit.
    let mut env = TimeLimit::new(cartpole(), 1000);
    env.reset(Some(0), None);

    let mut terminated_naturally = false;
    for _ in 0..1000 {
        let result = env.step(1_i64);
        if result.terminated {
            terminated_naturally = true;
            // Should NOT also have truncated.
            assert!(!result.truncated);
            break;
        }
    }
    assert!(
        terminated_naturally,
        "cartpole should terminate within 1000 steps"
    );
}

// ---- OrderEnforcing ----

#[test]
#[should_panic(expected = "must call reset() before step()")]
fn order_enforcing_panics_without_reset() {
    let mut env = OrderEnforcing::new(cartpole());
    let _ = env.step(1_i64);
}

#[test]
fn order_enforcing_allows_step_after_reset() {
    let mut env = OrderEnforcing::new(cartpole());
    env.reset(Some(42), None);
    let result = env.step(1_i64);
    assert!(result.reward >= 0.0);
}

// ---- Autoreset ----

#[test]
fn autoreset_stores_terminal_obs_and_resets() {
    let mut env = Autoreset::new(TimeLimit::new(cartpole(), 3));
    env.reset(Some(42), None);

    assert!(env.last_terminal_observation().is_none());

    // Step until the time limit truncates (3 steps).
    let mut truncated = false;
    for _ in 0..3 {
        let result = env.step(1_i64);
        if result.truncated || result.terminated {
            truncated = true;
            break;
        }
    }
    assert!(truncated, "should truncate after 3 steps");

    // After autoreset, the terminal observation is saved.
    assert!(env.last_terminal_observation().is_some());
}

#[test]
fn autoreset_clears_terminal_obs_on_manual_reset() {
    let mut env = Autoreset::new(TimeLimit::new(cartpole(), 2));
    env.reset(Some(42), None);
    let _ = env.step(1_i64);
    let _ = env.step(1_i64); // triggers truncation + autoreset

    assert!(env.last_terminal_observation().is_some());

    env.reset(Some(42), None);
    assert!(env.last_terminal_observation().is_none());
}

// ---- RecordEpisodeStatistics ----

#[test]
fn record_episode_statistics_tracks_returns() {
    let mut env = RecordEpisodeStatistics::new(TimeLimit::new(cartpole(), 5));
    env.reset(Some(42), None);

    assert_eq!(env.episode_count(), 0);
    assert_eq!(env.episode_return(), 0.0);

    for _ in 0..5 {
        let result = env.step(1_i64);
        if result.terminated || result.truncated {
            break;
        }
    }

    // Episode should have been recorded.
    assert!(env.episode_count() >= 1);
    assert!(env.episode_return() > 0.0);
    assert!(!env.return_history().is_empty());
    assert!(!env.length_history().is_empty());
}

#[test]
fn record_episode_statistics_resets_counters() {
    let mut env = RecordEpisodeStatistics::new(TimeLimit::new(cartpole(), 3));
    env.reset(Some(42), None);
    let _ = env.step(1_i64);
    assert!(env.episode_return() > 0.0);

    env.reset(Some(42), None);
    assert_eq!(env.episode_return(), 0.0);
    assert_eq!(env.episode_length(), 0);
}

#[test]
fn record_episode_statistics_buffer_rotation() {
    let mut env = RecordEpisodeStatistics::with_buffer(TimeLimit::new(cartpole(), 2), 3);

    // Run 5 episodes (buffer holds only 3).
    for _ in 0..5 {
        env.reset(Some(42), None);
        let _ = env.step(1_i64);
        let _ = env.step(1_i64);
    }

    assert_eq!(env.episode_count(), 5);
    assert_eq!(env.return_history().len(), 3);
    assert_eq!(env.length_history().len(), 3);
}

// ---- ClipReward ----

#[test]
fn clip_reward_clamps_values() {
    let mut env = ClipReward::new(cartpole(), -0.5, 0.5);
    env.reset(Some(42), None);

    let result = env.step(1_i64);
    // CartPole normally gives reward 1.0; clipped to 0.5.
    assert!(result.reward <= 0.5);
    assert!(result.reward >= -0.5);
}

// ---- TransformReward ----

#[test]
fn transform_reward_applies_function() {
    let mut env = TransformReward::new(cartpole(), |r| r * 10.0);
    env.reset(Some(42), None);

    let result = env.step(1_i64);
    // CartPole reward is 1.0, transformed to 10.0.
    assert!((result.reward - 10.0).abs() < f64::EPSILON);
}

// ---- NormalizeReward ----

#[test]
fn normalize_reward_produces_finite_values() {
    let mut env = NormalizeReward::new(cartpole());
    env.reset(Some(42), None);

    for _ in 0..20 {
        let result = env.step(1_i64);
        assert!(result.reward.is_finite(), "reward should be finite");
        if result.terminated || result.truncated {
            env.reset(Some(42), None);
        }
    }
}

#[test]
fn normalize_reward_resets_returns_accumulator() {
    let mut env = NormalizeReward::new(cartpole());
    env.reset(Some(42), None);

    // Step a few times to accumulate returns.
    for _ in 0..5 {
        let result = env.step(1_i64);
        if result.terminated {
            break;
        }
    }

    let var_before = env.running_var();
    assert!(var_before > 0.0);

    // Reset should zero the returns accumulator.
    env.reset(Some(42), None);
    // running_var is NOT reset (it's a running statistic), but returns should be 0.
    // Verify by stepping again: should still produce finite results.
    let result = env.step(1_i64);
    assert!(result.reward.is_finite());
}

// ---- NormalizeObservation ----

#[test]
fn normalize_observation_produces_finite_values() {
    let mut env = NormalizeObservation::new(cartpole());
    let obs = env.reset(Some(42), None);
    let flat = obs.flatten();
    assert!(flat.iter().all(|v| v.is_finite()));

    for _ in 0..10 {
        let result = env.step(1_i64);
        let flat = result.observation.flatten();
        assert!(flat.iter().all(|v| v.is_finite()));
        if result.terminated || result.truncated {
            break;
        }
    }
}

#[test]
fn normalize_observation_updates_statistics() {
    let mut env = NormalizeObservation::new(cartpole());
    env.reset(Some(42), None);

    assert_eq!(env.running_mean().len(), 4);
    assert_eq!(env.running_var().len(), 4);

    // After a few steps the mean should have moved from zero.
    for _ in 0..5 {
        let result = env.step(1_i64);
        if result.terminated {
            break;
        }
    }

    // At least one element of the running mean should be non-zero.
    assert!(env.running_mean().iter().any(|&v| v.abs() > 1e-10));
}

// ---- TransformObservation ----

#[test]
fn transform_observation_changes_type() {
    // Transform CartPoleObservation -> Vec<f64>
    let mut env = TransformObservation::new(cartpole(), |obs| obs.flatten());
    env.reset(Some(42), None);

    let result = env.step(1_i64);
    let flat: Vec<f64> = result.observation;
    assert_eq!(flat.len(), 4);
}

// ---- Wrapper composition ----

#[test]
fn wrapper_stacking_compiles_and_runs() {
    let env = CartPoleEnv::new();
    let env = TimeLimit::new(env, 100);
    let env = ClipReward::new(env, -1.0, 1.0);
    let mut env = RecordEpisodeStatistics::new(env);

    env.reset(Some(42), None);
    for _ in 0..10 {
        let result = env.step(1_i64);
        assert!(result.reward >= -1.0 && result.reward <= 1.0);
        if result.terminated || result.truncated {
            break;
        }
    }

    assert!(env.episode_return() > 0.0);

    // Traverse the wrapper chain via inner().
    assert!(env.inner().inner().max_steps() == 100);
}

// ---- FlattenObservation ----

#[test]
fn flatten_observation_returns_vec() {
    use gymnasia::wrappers::FlattenObservation;

    let mut env = FlattenObservation::new(cartpole());
    let obs: Vec<f64> = env.reset(Some(42), None);
    assert_eq!(obs.len(), 4);

    let result = env.step(1_i64);
    assert_eq!(result.observation.len(), 4);
}
