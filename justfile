default: fmt lint test build

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all --locked

build:
    cargo build --all --locked

build-release:
    cargo build --all --release --locked

check:
    cargo check --all --locked

proto-lint:
    buf lint

ci: fmt-check lint test build proto-lint
