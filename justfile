default: fmt lint test build

# Ensure cargo-sort is installed (no-op if already present).
_ensure-cargo-sort:
    @cargo sort --version > /dev/null 2>&1 || cargo install cargo-sort --quiet

fmt: _ensure-cargo-sort
    cargo sort --workspace
    cargo fmt --all

fmt-check: _ensure-cargo-sort
    cargo sort --workspace --check
    cargo fmt --all -- --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all --locked

build:
    cargo build --all --locked

build-release:
    cargo build --all --release --locked

coverage:
    cargo llvm-cov --workspace --all-targets --summary-only --fail-under-lines 90

check:
    cargo check --all --locked

proto-lint:
    buf lint

ci: fmt-check lint test build proto-lint
