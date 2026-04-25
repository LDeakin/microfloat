set shell := ["bash", "-cu"]

default:
    just --list

generate-fixtures:
    ./scripts/generate_fixtures.py

fmt:
    cargo fmt --all

clippy:
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic

clippy_nursery:
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery

test:
    cargo test --all-features

doc:
    RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo +nightly doc --all-features --no-deps

coverage:
    @command -v cargo-llvm-cov >/dev/null || { echo 'cargo-llvm-cov is required: cargo install cargo-llvm-cov'; exit 1; }
    cargo llvm-cov --all-features

coverage-html:
    @command -v cargo-llvm-cov >/dev/null || { echo 'cargo-llvm-cov is required: cargo install cargo-llvm-cov'; exit 1; }
    cargo llvm-cov --all-features --html

coverage-lcov:
    @command -v cargo-llvm-cov >/dev/null || { echo 'cargo-llvm-cov is required: cargo install cargo-llvm-cov'; exit 1; }
    cargo llvm-cov --all-features --lcov --output-path target/llvm-cov/lcov.info
