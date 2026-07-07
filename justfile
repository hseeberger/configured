set shell := ["bash", "-uc"]

nightly := `rustc --version | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sed 's/^/nightly-/'`

check:
    cargo check --tests
    cargo check --tests --no-default-features --features toml
    cargo check --tests --all-features

fix:
    cargo fix --tests --all-features --allow-dirty --allow-staged

fmt:
    cargo +{{ nightly }} fmt
    RUST_LOG=error taplo fmt

fmt-check:
    cargo +{{ nightly }} fmt --check

lint:
    cargo clippy --tests --no-deps                                      -- -D warnings
    cargo clippy --tests --no-deps --no-default-features --features toml -- -D warnings
    cargo clippy --tests --no-deps --all-features                       -- -D warnings

lint-fix:
    cargo clippy --tests --no-deps --all-features --allow-dirty --allow-staged --fix

test:
    cargo test
    cargo test --all-features

doc:
    RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo +{{ nightly }} doc --no-deps --all-features

all: check fmt lint test doc
