# === SETUP ===

# Initialize the project: install hooks and dependencies
init: install _install-hooks

# Install project dependencies (build to verify)
install:
    cargo fetch

# === BUILD ===

# Build with all features (default)
build:
    cargo build

# Build headless (no SDL2)
build-headless:
    cargo build --no-default-features

# Remove build artifacts
clean:
    cargo clean

# === TEST ===

# Run tests (headless)
test:
    cargo test --no-default-features

# Run tests with all features
test-all:
    cargo test

# === LINT ===

# Run clippy (headless)
lint:
    cargo clippy --no-default-features -- -D warnings

# Run clippy with all features
lint-all:
    cargo clippy -- -D warnings

# === FORMAT ===

# Format all code
fmt:
    cargo fmt --all
    taplo fmt || true
    alejandra . 2>/dev/null || true

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# === DOCS ===

# Build documentation
doc:
    cargo doc --no-deps --no-default-features

# Build and open documentation
doc-open:
    cargo doc --no-deps --no-default-features --open

# === EXAMPLES ===

# Run cartpole with rendering
cartpole:
    cargo run --example=cartpole

# Run cartpole headless
cartpole-headless:
    cargo run --example=cartpole_headless --no-default-features

# Run mountain car with rendering
mountain-car:
    cargo run --example=mountain_car

# === CHECK ===

# Run all CI checks (fmt + lint + test + doc)
check: fmt-check lint test doc

# === INTERNAL ===

[private]
_install-hooks:
    #!/usr/bin/env bash
    set -euo pipefail
    hook=".git/hooks/pre-commit"
    cat > "$hook" << 'HOOK'
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Running pre-commit checks..."
    just fmt-check
    just lint
    just test
    HOOK
    chmod +x "$hook"
    echo "Pre-commit hook installed."
