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
| Macro benches (LDBC SNB SF1) | `just bench-macro 1` — placeholder scaffold until M3 + #47 |
| Stress (chaos) | `just stress chaos` — placeholder scaffold until M3 + #48 |
| Preview the dashboard | `just dashboard` then open http://localhost:8000 |
| Check release automation | `just release-check` |
| Check commit subjects | `just check-commits origin/main` |

List everything with `just` alone.

## Sensitive data

The `private/` directory is gitignored and holds competitive research. Before any commit, run:

```bash
just check-private
```

It will fail loudly if anything under `private/` is staged.

## Release automation

`release-plz` is installed by `mise install` at the version pinned in
`.mise.toml`. The GitHub workflow opens or updates a release PR after pushes to
`main`; when that release PR is merged, it creates git tags and GitHub releases.
Crates.io publishing stays disabled until M3 via `git_only = true` and
`publish = false` in `release-plz.toml`.

Run the local release gate before changing release configuration:

```bash
just release-check
```

This validates `release-plz.toml` against the schema emitted by the pinned
`release-plz` binary, then runs `release-plz update` inside a temporary git
worktree. The pinned `release-plz 0.3.157` CLI does not provide
`update --dry-run`, so the temp-worktree recipe is the safe dry run: generated
`CHANGELOG.md`, `Cargo.toml`, and `Cargo.lock` changes are printed and then
discarded.

Pull requests are also checked by the `Conventional Commits` workflow. Locally,
use this before pushing:

```bash
just check-commits origin/main
```

## First-time Rust setup

`mise install` handles the Rust toolchain via `rust-toolchain.toml`. No manual `rustup` calls required. If you prefer `rustup` directly, the pinned version in `rust-toolchain.toml` is authoritative.

## Troubleshooting

- **`just` not found** → `mise install` didn't run. Re-run it.
- **CI fails but `just ci` passes locally** → ensure you're on the same `mise` tool versions; check `mise ls` vs `.mise.toml`.
- **Bench numbers unstable** → use `just bench-iai` for instruction-count benches; they're stable across hardware.
- **Fuzz targets missing** → they will live under `fuzz/` once the first parser/codec lands in M3+; list with `cargo fuzz list`.
- **`just bench-macro` / `just stress` look too successful** → until M3 + the harness issues (#47, #48) land, these commands are intentionally wired as truthful placeholders. They prove command shape and documentation wiring, not benchmark or correctness evidence.

## Self-hosted bench runner

The nightly bench gate (`.github/workflows/bench-nightly.yml`) runs on a
dedicated self-hosted runner. Wall-clock numbers from GitHub's shared
runners have 5-10 % variance, which is larger than our +2 % regression
threshold (AGENTS.md §5), so wall-clock work needs pinned hardware.

The PR gate (`.github/workflows/bench-regression.yml`) runs on
`ubuntu-latest` because it uses iai-callgrind (instruction counts,
deterministic across hosts). Self-hosted + public repo is a known
security risk and is intentionally avoided for PR-triggered runs.

### Runner spec

| Field | Baseline |
|-------|----------|
| OS | Ubuntu 24.04 LTS |
| CPU | ≥ 4 physical cores, x86_64 |
| RAM | ≥ 7 GB |
| Disk | ≥ 20 GB free for Cargo cache + target/ |
| Labels | `self-hosted`, `bench`, `physa-db` (plus auto-added `Linux`, `X64`) |
| Runner mode | `--ephemeral` (auto-deregister after each job) |
| Runner user | dedicated non-root system user (e.g. `github-runner`) |

### Provisioning (one-time)

1. Pick a host that is not shared with user workloads. If it also hosts
   another CI (GitLab runner, Buildkite, …), give each CI its own user
   and let them cohabit on separate home directories.
2. Create a system user with a shell and home:
   ```bash
   sudo adduser --system --group --shell /bin/bash --home /home/github-runner github-runner
   ```
3. Install build dependencies (`valgrind` is required for iai-callgrind):
   ```bash
   sudo apt-get install -y --no-install-recommends \
     valgrind build-essential pkg-config libssl-dev ca-certificates curl git
   ```
4. Install Rust for the runner user:
   ```bash
   sudo -u github-runner -H bash -c '
     curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | \
       sh -s -- -y --default-toolchain stable --profile minimal \
       --component rustfmt --component clippy --no-modify-path
   '
   ```
5. Install the CLI helpers used by the benchmark workflows:
   ```bash
   sudo -u github-runner -H bash -lc '
     export PATH=/home/github-runner/.cargo/bin:$PATH
     cargo install just --version 1.50.0 --locked
     cargo install cargo-criterion --version ^1.1 --locked
     cargo install iai-callgrind-runner --version ^0.14 --locked
   '
   ```
6. Download and register the GitHub Actions runner binary:
   ```bash
   RUNNER_VERSION=$(curl -s https://api.github.com/repos/actions/runner/releases/latest \
     | grep -oP '"tag_name": "v\K[^"]+')
   sudo -u github-runner -H bash -c "
     mkdir -p /home/github-runner/actions-runner
     cd /home/github-runner/actions-runner
     curl -sSL -o runner.tar.gz \
       https://github.com/actions/runner/releases/download/v\${RUNNER_VERSION}/actions-runner-linux-x64-\${RUNNER_VERSION}.tar.gz
     tar xzf runner.tar.gz && rm runner.tar.gz
   "
   sudo /home/github-runner/actions-runner/bin/installdependencies.sh
   ```
7. Generate a registration token from a host that can auth to GitHub as
   a repo admin:
   ```bash
   gh api -X POST repos/mroche14/physa-db/actions/runners/registration-token --jq .token
   ```
   The token is valid for ~1 h and one-shot.
8. Configure the runner (on the runner host), replacing `<TOKEN>`:
   ```bash
   sudo -u github-runner -H bash -c '
     cd /home/github-runner/actions-runner
     ./config.sh --unattended --ephemeral \
       --url https://github.com/mroche14/physa-db \
       --token <TOKEN> \
       --labels self-hosted,bench,physa-db \
       --name physa-db-bench-01 \
       --replace
   '
   ```
9. Install and start the systemd service:
   ```bash
   cd /home/github-runner/actions-runner
   sudo ./svc.sh install github-runner
   sudo ./svc.sh start
   ```
10. Verify from GitHub:
    ```bash
    gh api repos/mroche14/physa-db/actions/runners \
      --jq '.runners[] | {name, status, labels: [.labels[].name]}'
    ```
    Expect `"status": "online"` and the three custom labels.

### Troubleshooting

- **Runner stuck in `offline`** → `sudo systemctl status actions.runner.mroche14-physa-db.*` and `journalctl -u actions.runner.mroche14-physa-db.* -n 200`.
- **`config.sh` says "Failed to create self-hosted runner"** → token expired (>1 h since issue) or no admin rights. Regenerate via `gh api`.
- **`bench-nightly` fails with `no bench target named iai`** → the crate was removed from the `just bench-iai` recipe list; re-add it.
- **iai-callgrind reports `No version information found`** → missing `iai-callgrind` dev-dep in the bench's `Cargo.toml`.
- **Disk fills up** → `target/` caches accumulate. `sudo -u github-runner cargo clean` or prune old `_work/` subtrees under `/home/github-runner/actions-runner/_work/`.

### Decommissioning

```bash
cd /home/github-runner/actions-runner
sudo ./svc.sh stop
sudo ./svc.sh uninstall
sudo -u github-runner ./config.sh remove --token <REMOVAL_TOKEN>
```

The removal token is fetched the same way as the registration token but
via `remove-token` instead of `registration-token`.

## Onboarding for AI agents

Read `AGENTS.md` in full, then pick an issue labelled `status:ready` + `agent:good-first-task`. The rest of the workflow is in `AGENTS.md` §§3, 8, 9.
