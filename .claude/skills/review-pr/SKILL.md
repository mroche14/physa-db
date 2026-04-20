---
name: review-pr
description: >
  Structured review checklist for a physa-db pull request. Enforces AGENTS.md
  §§1, 5, 7, 8, 11, 12, 15 by requiring explicit sign-off on each rule the PR
  could violate. Use when reviewing a peer agent's PR or when self-reviewing
  before requesting human review.
when_to_use: >
  "review PR", "review this PR", "code review", "check PR X", PR author asks
  for a review, maintainer triaging.
argument-hint: "[PR-number or URL]"
user-invocable: true
---

# review-pr — physa-db review checklist

PR under review: **$ARGUMENTS**

Walk the checklist below top to bottom. For each item, output one of:

- `[x] pass` — explicit statement that you verified it
- `[ ] fail — <reason>` — blocking; requester must address
- `[~] n/a — <why>` — non-blocking; explain briefly

No item is "I'll skim that". A review that skips items is worse than
no review.

## 0 — Scope

- [ ] The PR does one logical thing (`AGENTS.md` §1.4). If it touches
      > 600 lines of non-generated code, it should be split.
- [ ] The PR title is an imperative sentence ≤ 72 chars.
- [ ] The body has `## What / ## Why / ## How / ## Benchmarks (if perf)
      / ## Stress (if concurrency) / ## Checklist` sections
      (`AGENTS.md` §8).
- [ ] The PR links the issue it closes with `Closes #N` or
      `Refs #N`. No orphan PRs.

## 1 — Features-first (`AGENTS.md` §15)

- [ ] If the PR introduces a new capability, it cites an existing
      `FM-NNN` row in the body (or lands the row in the same PR with
      a workload anchor).
- [ ] If the PR touches an ADR, the status respects §15: a new ADR is
      *Proposed*; an existing ADR changes status only via the
      `promote-adr` skill.
- [ ] No hidden architectural commitment. A storage layout change,
      wire protocol change, or data-type change MUST be backed by an
      ADR that is at least in *Proposed*.

## 2 — First-principles (`AGENTS.md` §11)

- [ ] If the PR adds non-trivial logic, the author cited the
      constraint / optimum. "Because it's faster" without numbers is
      not a derivation.
- [ ] If the implementation chooses between N approaches, the PR body
      (or a linked ADR) explains the tradeoff in physical terms.

## 3 — No shortcuts (`AGENTS.md` §12)

- [ ] No `todo!()` or `unimplemented!()` in shipped code.
- [ ] No "for now" / "temporary" / "we'll fix later" comments without
      a linked issue number.
- [ ] No feature-flag gates on core subsystems to hide a half-finished
      implementation.
- [ ] No `#[cfg(feature = "stub")]` paths in tests to avoid the hard
      case.

## 4 — Correctness (`AGENTS.md` §§1.3, 5)

- [ ] Every new public function has at least one unit test OR is
      covered by an existing integration / property test (point to
      the test).
- [ ] If the code touches a storage codec, serialisation, or query
      parser: a proptest target exists (`just test-prop` covers it).
- [ ] If the code uses `unsafe`, there is an entry in
      `docs/architecture/unsafe-allowlist.md` AND a loom / miri
      test validating it.
- [ ] If the code introduces concurrency primitives (locks, atomics,
      channels): a `loom` test exists.
- [ ] If the code changes cluster behaviour: a `turmoil` scenario
      exists.

## 5 — Benchmarks (`AGENTS.md` §5, §8)

- [ ] If the PR is labelled `type:perf`, the PR body has a
      `## Benchmarks` section with: hardware spec, dataset, command,
      before/after numbers, std dev.
- [ ] No perf claim without a reproducible bench. "~10x faster" is
      not acceptable.
- [ ] The nightly bench-regression CI either passed, or the PR
      explicitly waives a regression > 2% with a justification
      (rare).

## 6 — Stress (`AGENTS.md` §5)

- [ ] If the PR changes storage, MVCC, clustering, or anything in
      `physa-cluster`: the relevant stress scenarios (`just stress
      <scenario>`) run clean. Paste the summary or attach a log.
- [ ] New concurrency code ships with a `loom` test OR a note
      explaining why loom is not applicable.

## 7 — Privacy (`AGENTS.md` §7, ADR-0006)

- [ ] No real competitor name anywhere in the diff. Grep the full
      diff for likely competitor names.
- [ ] No path under `private/` staged (CI enforces this, but a human
      check is cheap insurance).
- [ ] If the PR references research, it cites `docs/requirements/`
      only, not `private/`.

## 8 — Dependencies

- [ ] No new top-level dep with a restrictive licence (GPL, AGPL,
      SSPL, BSL, commercial).
- [ ] `cargo audit` passes with no known-vulnerable advisories
      unacknowledged.
- [ ] Any dep added for a single function was evaluated: could we
      write the function in < 50 LOC instead?

## 9 — Docs track (`AGENTS.md` §13 "Docs track")

- [ ] User-facing behaviour changes updated the user doc in the
      same PR.
- [ ] ADR status changes reflected in `ROADMAP.md` / relevant memo
      files if necessary.
- [ ] Any new `just` recipe is documented inline in the `justfile`.

## 10 — CI signals

- [ ] `just ci` passed locally (stated in the PR body).
- [ ] GitHub Actions workflows green: `ci`, `snapshot-dashboard`,
      `bench-regression` (if applicable).
- [ ] No red tests waived with `#[ignore]` without an issue link.

## Final call

If all items are `[x]` or `[~]`:

```
APPROVE. Ship it.
```

If any item is `[ ] fail`:

```
REQUEST CHANGES. Blocking:
- <item label>: <reason>
- <item label>: <reason>
```

Post the full checklist (not just the final call) as a review comment so
later readers can see what was verified.

## What NOT to do

- Do not approve a PR you haven't read. "LGTM" without the checklist
  is noise.
- Do not reject a PR on style alone if `just fmt` and `just lint`
  pass — if the CI is happy, so are we.
- Do not ask the author to add scope outside their issue's acceptance
  criteria. Scope creep is a §1.4 violation on the reviewer side too.
