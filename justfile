# Fetch project dependencies
fetch:
    cargo fetch

# Initialize the project
init: fetch

# Build the project
build:
    cargo build

# Format all code
fmt:
    cargo fmt --all

# Run clippy
lint:
    cargo clippy --no-default-features -- -D warnings

# Run all CI checks locally (fmt + lint + test)
check:
    cargo fmt --all -- --check
    cargo clippy --no-default-features -- -D warnings
    cargo test --no-default-features
