# Gymnasia

OpenAI Gymnasium environments in pure Rust.

## Quick Start

```toml
[dependencies]
gymnasia = "1.0.0"
```

For headless usage (no SDL2 required):

```toml
[dependencies]
gymnasia = { version = "1.0.0", default-features = false }
```

See [USAGE.md](./USAGE.md) for detailed setup instructions across different operating systems.

## Examples

### With rendering (requires SDL2)

```bash
cargo run --example=cartpole
```
![cartpole](assets/cartpole.png)

```bash
cargo run --example=mountain_car
```
![mountain_car](assets/mountain_car.png)

### Headless

```bash
cargo run --example=cartpole_headless --no-default-features
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `sdl2` | Yes | SDL2 rendering and nalgebra support |
| `bundled` | Yes | Compiles SDL2 from source (no system install needed) |

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Licensed under [Apache 2.0](./LICENSE).
