# physa-db — unified dev commands.
# Run `just` with no args to list available recipes.

set shell := ["bash", "-euo", "pipefail", "-c"]
set dotenv-load := true

# Show all recipes.
default:
    @just --list --unsorted

# ------------------------------------------------------------------
# Core quality gates — run locally before any commit.
# ------------------------------------------------------------------

# Format all Rust code.
fmt:
    cargo fmt --all

# Check formatting without rewriting.
fmt-check:
    cargo fmt --all --check

# Run clippy with workspace-wide errors on warnings.
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run the full test suite. `--no-tests=pass` lets CI stay green while
# crates remain stubs; remove once the first real test lands.
test:
    cargo nextest run --workspace --all-features --no-tests=pass

# Run the proptest regression suite (slower, more thorough).
test-prop:
    PROPTEST_CASES=4096 cargo nextest run --workspace --all-features -- --ignored proptest

# Run documentation tests.
test-doc:
    cargo test --doc --workspace

# Run a cargo-fuzz target (smoke, 60 s).
fuzz target:
    cargo +nightly fuzz run {{target}} -- -max_total_time=60

# ------------------------------------------------------------------
# Benchmarks & stress — "all along the dev" per founder's rule.
# ------------------------------------------------------------------

# Run criterion micro-benchmarks.
bench:
    cargo criterion --workspace

# Compare current HEAD against a stored baseline (e.g. `just bench-compare main`).
bench-compare baseline="main":
    cargo criterion --workspace -- --baseline {{baseline}}

# Save the current run as the new baseline.
bench-save baseline="current":
    cargo criterion --workspace -- --save-baseline {{baseline}}

# Instruction-count benches (stable across hardware, good for CI gates).
bench-iai:
    cargo bench --bench iai --workspace

# Macro benchmarks (LDBC SNB / SNAP) on SF1.
bench-macro sf="1":
    cargo run --release -p physa-cli --bin physa-bench -- run --sf {{sf}}

# Run a stress scenario (chaos, soak, partition, disk-full, …).
stress scenario="smoke":
    cargo run --release -p physa-cli --bin physa-stress -- {{scenario}}

# ------------------------------------------------------------------
# CI / release / docs
# ------------------------------------------------------------------

# The exact gate CI runs. Must pass locally before you push.
ci: fmt-check lint test test-doc
    @echo "CI gate passed."

# Regenerate the dashboard JSON snapshot from GitHub Issues + Projects v2.
snapshot-dashboard:
    cargo run --release -p xtask -- snapshot-dashboard

# Create GitHub issues from docs/seed-issues.md (dry-run by default).
seed-issues dry_run="true":
    cargo run --release -p xtask -- seed-issues --dry-run={{dry_run}}

# Build and preview the dashboard locally.
dashboard:
    cd dashboard && python3 -m http.server 8000

# Emit the agent prompt for profiling a competitor codename.
research-prompt codename:
    cargo run --release -p xtask -- research-prompt {{codename}}

# ------------------------------------------------------------------
# Dev loop
# ------------------------------------------------------------------

# Watch mode: re-run tests on change.
dev:
    cargo watch -x 'nextest run --workspace' -x 'clippy --workspace --all-targets -- -D warnings'

# Update all dependencies within semver-compatible ranges.
update:
    cargo update --workspace

# Audit for vulnerable / unmaintained dependencies.
audit:
    cargo audit

# Verify that nothing from private/ is staged.
check-private:
    @git status --porcelain | awk '{print $2}' | grep -E '^private/' && (echo "ERROR: private/ path staged" && exit 1) || exit 0
