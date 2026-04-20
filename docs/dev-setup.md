# Dev setup

Two commands, and you're productive.

## Prerequisites

- `mise` — [install guide](https://mise.jdx.dev/getting-started.html) (one-liner on Linux/macOS).
- `git`.

## The two commands

```bash
# 1. Install pinned toolchain + helpers (Rust stable, just, cargo-nextest, …)
mise install

# 2. Start the dev loop (watch-mode tests + clippy)
just dev
```

That's it. The `just` recipes expose every other workflow you need.

## Everyday recipes

| Goal | Command |
|------|---------|
| Format | `just fmt` |
| Lint (clippy) | `just lint` |
| Run tests | `just test` |
| Run property tests (slower) | `just test-prop` |
| Run the CI gate locally | `just ci` |
| Fuzz a target for 60 s | `just fuzz <target>` |
| Micro benches | `just bench` |
| Compare vs `main` | `just bench-compare main` |
| Macro benches (LDBC SNB SF1) | `just bench-macro 1` |
| Stress (chaos) | `just stress chaos` |
| Preview the dashboard | `just dashboard` then open http://localhost:8000 |

List everything with `just` alone.

## Sensitive data

The `private/` directory is gitignored and holds competitive research. Before any commit, run:

```bash
just check-private
```

It will fail loudly if anything under `private/` is staged.

## First-time Rust setup

`mise install` handles the Rust toolchain via `rust-toolchain.toml`. No manual `rustup` calls required. If you prefer `rustup` directly, the pinned version in `rust-toolchain.toml` is authoritative.

## Troubleshooting

- **`just` not found** → `mise install` didn't run. Re-run it.
- **CI fails but `just ci` passes locally** → ensure you're on the same `mise` tool versions; check `mise ls` vs `.mise.toml`.
- **Bench numbers unstable** → use `just bench-iai` for instruction-count benches; they're stable across hardware.
- **Fuzz targets missing** → they live under `fuzz/` (created in M2+); list with `cargo fuzz list`.

## Onboarding for AI agents

Read `AGENTS.md` in full, then pick an issue labelled `status:ready` + `agent:good-first-task`. The rest of the workflow is in `AGENTS.md` §§3, 8, 9.
