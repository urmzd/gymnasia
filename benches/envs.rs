use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gymnasia::{
    core::Env,
    envs::classical_control::{cartpole::CartPoleEnv, mountain_car::MountainCarEnv},
};

fn bench_cartpole_step(c: &mut Criterion) {
    let mut env = CartPoleEnv::new();
    env.reset(None, None);

    c.bench_function("cartpole/step", |b| {
        b.iter(|| {
            let result = env.step(black_box(1));
            if result.terminated || result.truncated {
                env.reset(None, None);
            }
        })
    });
}

fn bench_cartpole_reset(c: &mut Criterion) {
    let mut env = CartPoleEnv::new();

    c.bench_function("cartpole/reset", |b| {
        b.iter(|| {
            env.reset(black_box(None), None);
        })
    });
}

fn bench_cartpole_episode(c: &mut Criterion) {
    let mut env = CartPoleEnv::new();

    c.bench_function("cartpole/episode", |b| {
        b.iter(|| {
            env.reset(None, None);
            loop {
                let result = env.step(black_box(1));
                if result.terminated || result.truncated {
                    break;
                }
            }
        })
    });
}

fn bench_mountain_car_step(c: &mut Criterion) {
    let mut env = MountainCarEnv::new();
    env.reset(None, None);

    c.bench_function("mountain_car/step", |b| {
        b.iter(|| {
            let result = env.step(black_box(2));
            if result.terminated || result.truncated {
                env.reset(None, None);
            }
        })
    });
}

fn bench_mountain_car_reset(c: &mut Criterion) {
    let mut env = MountainCarEnv::new();

    c.bench_function("mountain_car/reset", |b| {
        b.iter(|| {
            env.reset(black_box(None), None);
        })
    });
}

fn bench_mountain_car_episode(c: &mut Criterion) {
    let mut env = MountainCarEnv::new();

    c.bench_function("mountain_car/episode", |b| {
        b.iter(|| {
            env.reset(None, None);
            for _ in 0..200 {
                let result = env.step(black_box(2));
                if result.terminated || result.truncated {
                    break;
                }
            }
        })
    });
}

criterion_group!(
    cartpole,
    bench_cartpole_step,
    bench_cartpole_reset,
    bench_cartpole_episode,
);

criterion_group!(
    mountain_car,
    bench_mountain_car_step,
    bench_mountain_car_reset,
    bench_mountain_car_episode,
);

criterion_main!(cartpole, mountain_car);
