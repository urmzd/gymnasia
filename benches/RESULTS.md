| Benchmark | Time (median) |
|-----------|---------------|
| `cartpole/step` | ~26 ns |
| `cartpole/reset` | ~21 ns |
| `cartpole/episode` | ~248 ns |
| `mountain_car/step` | ~25 ns |
| `mountain_car/reset` | ~15 ns |
| `mountain_car/episode` | ~4.6 µs |

> Apple M3 Pro — `cargo bench` via [Criterion](https://github.com/bheisler/criterion.rs). Run `cargo bench` to reproduce.
