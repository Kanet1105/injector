default: fmt lint test build

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all

build:
    cargo build --all

build-release:
    cargo build --all --release

check:
    cargo check --all

ci: fmt-check lint test build
