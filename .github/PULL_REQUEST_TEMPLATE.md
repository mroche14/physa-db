## What
<!-- One paragraph describing the change. -->

## Why
<!-- Link the related issue with `Closes #N`. Summarise motivation. -->
Closes #

## How
<!-- Key design choices. Trade-offs. Alternatives rejected. -->

## Benchmarks
<!-- Required for type:perf PRs. Omit otherwise. -->
<!--
| Workload | Before | After | Delta | Hardware |
|----------|--------|-------|-------|----------|
|          |        |       |       |          |

Commands run:
```
```
-->

## Checklist
- [ ] `cargo fmt --all --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] New code has unit tests (and property tests if touching storage/query)
- [ ] Docs updated (user docs under `docs/`, rustdoc on public items)
- [ ] ADR added if this changes architecture (link: `docs/architecture/adr/NNNN-*.md`)
- [ ] No new `unsafe` without an allowlist entry
- [ ] PR is scoped to a single logical change
