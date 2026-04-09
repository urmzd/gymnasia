# Contributing

Contributions are welcome — new environments, wrappers, bug fixes, and documentation improvements.

## Getting started

```bash
git clone https://github.com/urmzd/gymnasia.git
cd gymnasia
just fetch
just check   # fmt + lint + test
```

## Workflow

1. Fork the repo and create a branch from `main`.
2. Make your changes.
3. Run `just check` to verify formatting, linting, and tests pass.
4. Open a pull request against `main`.

## Commit convention

This project uses [conventional commits](https://www.conventionalcommits.org/) enforced by [sr](https://github.com/urmzd/sr). Examples:

```
feat(envs): add Pendulum environment
fix(wrappers): correct TimeLimit off-by-one
docs: update Quick Start example
```

## Adding an environment

Translate from [Gymnasium](https://github.com/Farama-Foundation/Gymnasium) source. Each environment implements the `Env` trait and optionally `Renderable` for visualization. See existing environments in `src/envs/classical_control/` for the pattern.
