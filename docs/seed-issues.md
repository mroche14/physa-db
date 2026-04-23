# Seed issues — the first tasks to open once the repo is pushed

Each block below is ready to become a GitHub issue. When the repo is pushed to GitHub, run:

```bash
gh issue create --title "<title>" --body-file <(echo "<body>") --label "<labels>"
```

Or batch via `xtask seed-issues --dry-run` (current stub; real importer tracked separately).

**Reminder:** competitor names must never appear in public issues. Use codenames (`Competitor ALPHA`, `BETA`, …). The codename↔name map is a local thread note, never committed. See ADR-0006.

---

## M0 — Foundation

### Finalise CI matrix and cache
- **Labels:** `area:infra`, `type:feature`, `status:ready`, `priority:p1`
- **Acceptance:**
  - [ ] `ci.yml` runs `just ci` on Linux + macOS (Windows is a non-goal — see `docs/requirements/non-goals.md`)
  - [ ] `Swatinem/rust-cache` configured with workspace-aware key
  - [ ] First green build on `main`

### Implement `xtask snapshot-dashboard`
- **Labels:** `area:infra`, `type:feature`, `status:ready`, `priority:p1`
- **Context:** ADR-0001 commits us to a static `state.json` snapshot. The `snapshot-dashboard.yml` workflow currently emits a placeholder. Implement the real generator as an `xtask` subcommand so it's reproducible locally.
- **Acceptance:**
  - [ ] `cargo xtask snapshot-dashboard` pulls issues, PRs, Projects v2 data via GraphQL
  - [ ] Output matches schema in `dashboard/src/app.js`
  - [ ] Handles pagination; idempotent; stable sort
  - [ ] Tests with recorded GraphQL fixtures
  - [ ] `just snapshot-dashboard` recipe wired up

### Publish dashboard to GitHub Pages
- **Labels:** `area:infra`, `type:feature`, `status:ready`, `priority:p1`
- **Acceptance:**
  - [ ] Pages enabled
  - [ ] First deployed URL added to `README.md`

### Provision labels via `.github/labels.yml` + sync action
- **Labels:** `area:infra`, `type:feature`, `status:ready`, `priority:p2`
- **Acceptance:**
  - [ ] `labels.yml` encodes the canonical set from `AGENTS.md` §6
  - [ ] Sync action runs on push to `main`

### Wire up `release-plz`
- **Labels:** `area:infra`, `type:feature`, `status:ready`, `priority:p2`
- **Acceptance:**
  - [ ] `release-plz.toml` validated
  - [ ] `.github/workflows/release-plz.yml` produces a release PR on `main`
  - [ ] Conventional commit check enforced on PRs

### Set up `bench-regression` workflow
- **Labels:** `area:infra`, `area:benchmark`, `type:feature`, `status:ready`, `priority:p1`
- **Acceptance:**
  - [ ] Nightly job runs `just bench` on a dedicated runner
  - [ ] Results stored under `gh-pages:bench-history/`
  - [ ] PR comments with delta vs baseline
  - [ ] Gate fails on >2% regression for tracked benchmarks

---

## M1 — Research corpus + foundational ADRs

> All competitor research is PRIVATE. Issues below refer only to codenames. The codename table is kept in a local note; agents receive it out-of-band.

### Private — profile Competitor ALPHA (depth)
- **Labels:** `type:research`, `status:ready`, `priority:p1`, `agent:long-running`
- **Deliverable:** `private/research/competitors/alpha.md` following the template. Not committed.
- **Acceptance:**
  - [ ] All template sections filled in
  - [ ] At least 5 weaknesses with cited sources
  - [ ] At least 2 published benchmarks with links
  - [ ] Licence trap analysis (SaaS angle)
  - [ ] One-paragraph executive summary of attack vectors
- **Output to public:** a delta to `docs/requirements/feature-matrix.md` (attribution-free).

### Private — profile Competitor BRAVO, CHARLIE, DELTA, …
- **Labels:** `type:research`, `status:ready`, `priority:p2`, `agent:long-running`
- One issue per codename. Same acceptance structure as ALPHA.

### Private — pain-point mining, batch 1 (Competitor ALPHA)
- **Labels:** `type:research`, `status:ready`, `priority:p1`
- **Acceptance:**
  - [ ] 20+ entries in `private/research/pain-points.md`
  - [ ] Each entry mapped to a requirement in `docs/requirements/feature-matrix.md`

### Private — pain-point mining, batches 2–N
- **Labels:** `type:research`, `status:ready`, `priority:p2`

### Publish `docs/requirements/feature-matrix.md` v1
- **Labels:** `area:docs`, `type:docs`, `status:ready`, `priority:p1`
- **Acceptance:**
  - [ ] Feature list covering storage, query, cluster, ops, ecosystem
  - [ ] Each row tagged as parity / novel / stretch
  - [ ] Zero competitor attribution
  - [ ] Cross-links to relevant ADRs

### Publish `docs/requirements/performance-targets.md` v1
- **Labels:** `area:docs`, `area:benchmark`, `type:docs`, `status:ready`, `priority:p1`
- **Acceptance:**
  - [ ] LDBC SNB Interactive IC-1…14 p50/p95/p99 targets (SF1, SF10, SF100)
  - [ ] LDBC SNB BI BI-1…20 throughput targets
  - [ ] Cold-start recovery time target
  - [ ] Horizontal-scale efficiency target (speedup vs nodes)

### LDBC SNB ingestion harness
- **Labels:** `area:benchmark`, `type:feature`, `status:ready`, `priority:p1`
- **Acceptance:**
  - [ ] Downloader for LDBC SNB SF1 and SF10 datasets
  - [ ] Parser emitting a canonical in-memory representation
  - [ ] `physa-bench load <sf>` command
  - [ ] No DB required — pure data prep
  - [ ] `just bench-macro` wires up SF1 smoke run
