_default:
    cargo just --list -u

init:
    cargo tool --install

lint: lint-check

lint-check:
    cargo clippy --no-deps --all-targets --all-features -- -D warnings

lint-fix:
    cargo clippy --no-deps --all-targets --all-features --fix

format: format-fix

format-check:
    cargo fmt --all -- --check

format-fix:
    cargo fmt --all

fix:
    cargo just format-fix
    cargo just lint-fix

check:
    cargo just format-check
    cargo just lint
    cargo just doc-check

doc:
    cargo doc --all-features --no-deps --open --lib

doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

doc-gen:
    cargo clean --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
    echo '<meta http-equiv="refresh" content="0;url=alpha_blend/index.html">' > target/doc/index.html
    rm target/doc/.lock

semver-checks:
    cargo tool cargo-semver-checks --baseline-version 0.1.2

msrv:
    cargo tool cargo-hack check --rust-version --workspace --all-targets --all-features --ignore-private

test *ARGS:
    cargo tool cargo-nextest run {{ARGS}}

test-doc *ARGS:
    cargo test {{ARGS}} --doc --all-features

test-all:
    cargo just test --all-features
    cargo just test-doc

coverage *ARGS:
    cargo tool cargo-llvm-cov --lib --all-features --open

coverage-gen:
    cargo tool cargo-llvm-cov --lib --all-features --lcov --output-path lcov.info
