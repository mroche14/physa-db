---
name: write-adr
description: >
  Template and checklist to author an Architecture Decision Record for
  physa-db. Enforces AGENTS.md §11 (first-principles derivation section) and
  §15 (must cite the FM-NNN rows it addresses). A freshly-written ADR always
  starts as Proposed; it is promoted to Accepted only via the `promote-adr`
  skill once the feature rows it addresses are locked.
when_to_use: >
  "write ADR", "open ADR", "need a design doc", "architecture decision", when
  a feature plan (via `plan-feature`) concluded an ADR is required.
argument-hint: "[ADR-NNNN] [short-title]"
user-invocable: true
---

# write-adr — ADR authoring playbook

The user wants an ADR for: **$ARGUMENTS**

Architectural decisions are binding. The process below exists so that every
ADR can be audited later for (a) correct problem framing, (b) a real
first-principles derivation, and (c) a traceable link to the features it
serves.

## Step 0 — Pick the ADR number

List existing ADRs under [`docs/architecture/adr/`](../../../docs/architecture/adr/).
Use the next integer. Do NOT reuse a number.

## Step 1 — Find every FM row this ADR addresses

An ADR without an `FM-NNN` anchor is premature (`AGENTS.md` §15).

- Open [`docs/requirements/feature-matrix.md`](../../../docs/requirements/feature-matrix.md).
- List every row this ADR decides. If you find zero, invoke `plan-feature`
  first to add the row(s).
- If a row exists but does not yet reference your in-progress ADR number,
  plan to update the `ADR` column in the same PR that lands the ADR.

## Step 2 — Write the ADR using this template

Save to `docs/architecture/adr/NNNN-short-title.md`. Replace every `{{...}}`.

```markdown
# ADR-{{NNNN}}: {{One-line title starting with a verb}}

- **Status:** Proposed (pending M1/M2 feature lock — see `AGENTS.md` §15)
- **Date:** {{YYYY-MM-DD}}
- **Features addressed:** {{FM-NNN, FM-NNN, …}}
- **Workloads addressed (if AI-native):** {{W-X, W-Y}}
- **Context issue:** _(to be filed as `type:feature area:XXX`)_

> **Note on status.** New ADRs start *Proposed*. They move to *Accepted*
> only when the FM rows above are locked AND any upstream dependencies
> (e.g. open sub-ADRs) are resolved. Use the `promote-adr` skill for the
> promotion — never edit the status field by hand.

## Context

{{What problem are we solving? Which workloads / pillar / incumbent
constraint forces this decision? Quote the relevant paragraphs from
`ai-agent-workloads.md` or `positioning.md`. Do NOT restate unrelated
motivation.}}

## Decision

{{State the decision in 3–5 bullets. A reader should know EXACTLY what
was chosen without reading the rest of the file.}}

## First-principles derivation (`AGENTS.md` §11)

### Irreducible costs

{{List the physical / informational constraints. Examples: NVMe 4 KB read
= 80 μs; L1 cache line = 64 B; f32 vector of dim 1024 = 4 KB = one NVMe
block; Raft consensus = 1 RTT + disk fsync; Bolt roundtrip = 1 RTT.
Numbers where possible.}}

### Theoretical optimum

{{Given the constraints above, what is the minimum cost (bytes moved,
round-trips, syncs) any correct design must pay? Show the math. If you
cannot express the optimum quantitatively, you have not yet understood
the problem — go back.}}

### Design that realises that optimum

{{The smallest structure that approaches the optimum. Identify which
pieces are reused from prior art (cite papers/blogs) and which are
novel to physa-db.}}

### Why alternatives miss the optimum

{{Brief: for each rejected alternative, which constraint it violates
or which overhead it forces.}}

## Consequences

**Positive**
- {{…}}

**Negative**
- {{…}}

Accepted under `AGENTS.md` §§11, 12 (and §15 once the gated FM rows are
locked).

## Alternatives considered

- **{{Alternative A}}.** {{Why rejected.}}
- **{{Alternative B}}.** {{Why rejected.}}

## Open sub-ADRs

{{If this ADR defers decisions to later sub-ADRs, list them with
working titles. Example: "ADR-NNNN: on-disk page format".}}

## References

- {{Paper / blog / standard with URL.}}
- {{…}}
```

## Step 3 — Local validation

Before opening the PR, verify:

- [ ] Status is `Proposed` (never `Accepted` in a brand-new ADR).
- [ ] `Features addressed` header lists at least one real `FM-NNN` row.
- [ ] First-principles derivation contains **numbers** (bytes, μs, RTTs),
      not only prose.
- [ ] At least one alternative is rejected with a constraint-based
      argument (not "it's uglier").
- [ ] No competitor is named in public text. Use codenames or generic
      phrases ("a popular LSM-based KV store").
- [ ] Every external reference URL is stable and non-pirated.
- [ ] The ADR file is ≤ 600 lines. If longer, split into sub-ADRs.

## Step 4 — File the tracking issue and PR

- Open a GitHub Issue (`type:feature` + relevant `area:` label) with the
  ADR text pasted into the body. Title: `ADR-NNNN proposal: {{title}}`.
- Open a PR that lands the `.md` file under `docs/architecture/adr/`, with
  the issue link in the PR body and `area:docs type:adr` labels.

## Step 5 — What NOT to do

- Do not mark as `Accepted` yourself. Promotion is a separate event that
  requires the FM rows to be locked — use the `promote-adr` skill.
- Do not bundle ADR text with code changes. ADRs land in their own PR.
- Do not ship an ADR whose first-principles section is hand-wavy. If you
  can't put numbers to it, you haven't derived it.
- Do not copy another ADR's structure wholesale. Every ADR is an
  independent derivation — if two ADRs share the same derivation, one of
  them has a scope problem.
