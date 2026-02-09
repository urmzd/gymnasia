# OpenAI Gym (Unofficial Rust Implementation)

This library aims be be close to the original OpenAI Gym library written in Python.

If you don't mind Python and would like to use the original implementation from Rust,
check out a [OpenAI Gym wrapper](https://github.com/MrRobb/gym-rs).


## Usage

To use this crate in your project, put this in your Cargo.toml:

```toml
[dependencies]
gym-rs = "0.4.0"
```

By default, this includes SDL2 rendering support. If you don't need rendering
(e.g. CI, cloud VMs, embedded, or headless training), you can disable it:

```toml
[dependencies]
gym-rs = { version = "0.4.0", default-features = false }
```

### SDL2 prerequisites (only when rendering is enabled)

SDL2 is an **optional** dependency gated behind the `sdl2` feature (on by default).
You only need to install the SDL2 libraries if you use the default features or
explicitly enable `sdl2`.

- [SDL2](https://wiki.libsdl.org/Installation)
- [SDL2_gfx](https://www.ferzkopp.net/Software/SDL2_gfx/Docs/html/index.html)

**Ubuntu:**
```shell
sudo apt install libsdl2-dev libsdl2-gfx-dev
```

**Arch:**
```shell
sudo pacman -S sdl2 sdl2_gfx
```

**Nix:**
```shell
nix develop
```

### Usage on Windows (with rendering)

As per [#6](https://github.com/MathisWellmann/gym-rs/issues/6), here are some instructions for the Windows folks:

0. clone the repo & cd to the root dir of the repo
1. modify Cargo.toml, remove dependency sdl2 line and add following code:

```
[dependencies.sdl2]
version = "0.35.2"
default-features = false
features = ["static-link", "use-vcpkg", "gfx"]

[package.metadata.vcpkg]
dependencies = ["sdl2", "sdl2-gfx"]
git = "https://github.com/microsoft/vcpkg"
rev = "16ee2ec"

[package.metadata.vcpkg.target]
x86_64-pc-windows-msvc = { triplet = "x64-windows-static-md" }

```
2. install cargo-vcpkg using cargo install cargo-vcpkg
3. under the root dir of the repo, cargo vcpkg build
4. now build and run, such as cargo run --example=mountain_car

Alternatively, to skip SDL2 entirely on Windows, use `default-features = false`.

## Examples

### With rendering (requires SDL2)

```bash
cargo run --example=cartpole
```
![cart_pole](assets/cartpole.png)

```bash
cargo run --example=mountain_car
```
![mountain_car](assets/mountain_car.png)

### Headless (no SDL2 required)

```bash
cargo run --example=cartpole_headless --no-default-features
```


## Contributions

Contributions are welcome. For the contribution guidelines, please take a look at [CONTRIBUTING.md](./CONTRIBUTING.md).

## Donations

If you would like to support the development of this crate, feel free to send over a donation:

Monero:

```plain
47xMvxNKsCKMt2owkDuN1Bci2KMiqGrAFCQFSLijWLs49ua67222Wu3LZryyopDVPYgYmAnYkSZSz9ZW2buaDwdyKTWGwwb
```

![monero](assets/monero_donations_qrcode.png)
