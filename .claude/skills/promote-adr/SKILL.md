---
name: promote-adr
description: >
  Promote a physa-db ADR from Proposed to Accepted, enforcing the §15 gates:
  the FM rows it addresses must be locked, any upstream sub-ADRs resolved, the
  first-principles derivation must contain numerical claims, and alternatives
  must be explicitly rejected. Refuses promotion otherwise. This skill is the
  only legitimate path to flip an ADR's status field.
when_to_use: >
  "promote ADR", "accept ADR", "ADR is ready", at M2 exit, after an FM row
  referenced by a Proposed ADR is locked.
argument-hint: "[ADR-NNNN]"
user-invocable: true
disable-model-invocation: true
---

# promote-adr — Proposed → Accepted gate

**Target ADR: $ARGUMENTS**

This skill is model-invocation disabled. Promotion is a governance event —
only the user (or a human reviewer) should trigger it. An auto-promotion
that flips an ADR to Accepted without feature lock breaks §15.

Proceed only if you have explicit confirmation.

## Gate A — File and status sanity

1. The ADR file exists at `docs/architecture/adr/NNNN-*.md`.
2. Its current status line matches: `Status: Proposed (pending M1 feature lock — see AGENTS.md §15)` (or a similar Proposed variant).
3. The `Date:` field is the ORIGINAL draft date. Add a new line
   `- **Accepted on:** YYYY-MM-DD` rather than overwriting the date.

If 1-2 don't hold, STOP and report.

## Gate B — Features-first (AGENTS.md §15)

Open [`docs/requirements/feature-matrix.md`](../../../docs/requirements/feature-matrix.md).

- Every `FM-NNN` row listed in the ADR's "Features addressed" header
  must exist in the matrix. If any row is missing, refuse promotion.
- Each of those rows must either:
  - have tier = `M1` exit criterion met (feature ratified), OR
  - have tier = one of the milestones later than the current date AND
    the row is marked "locked" (non-draft), AND the ADR references only
    the locked portions of its scope.
- Each row's `ADR` column should reference this ADR number. If not,
  update it in the same PR.

Report which rows were validated. If any row is unlocked, STOP.

## Gate C — First-principles is quantitative (§11)

Open the ADR's "First-principles derivation" section.

- There must be at least one concrete number (μs, ns, bytes, RTT, %)
  in the "Irreducible costs" subsection.
- The "Theoretical optimum" subsection must express the lower bound
  quantitatively OR explicitly state why the optimum is
  dimensionless (rare; e.g. a decision about language choice).
- Alternatives must be rejected with a constraint-based reason, not a
  taste-based one.

If the derivation is prose-only with no numbers, STOP and ask the
author to add them.

## Gate D — Upstream dependencies resolved

If the ADR's "Open sub-ADRs" section is non-empty, every sub-ADR
listed there must be:

- written and merged (even if itself still Proposed), OR
- explicitly removed from the list with a note "moved to future work,
  not a blocker for this ADR's decision".

An ADR that depends on not-yet-written sub-ADRs cannot be promoted.

## Gate E — No competitor attribution (§7)

Grep the ADR file for any real competitor name in the codenames table.
If a match is found, STOP. Rewrite to codename / generic reference
before re-attempting promotion.

## Gate F — Privacy check

```bash
just check-private
```

Must pass. Ensures no `private/` file was accidentally staged alongside
the ADR edit.

## Action — edit the ADR file

Only if ALL gates passed:

1. Change the status header to:
   ```
   - **Status:** Accepted
   - **Date:** YYYY-MM-DD (original draft)
   - **Accepted on:** YYYY-MM-DD
   ```
2. Replace the "Note on status" block at the top with a short
   acceptance note:
   ```
   > **Note on status.** Promoted from Proposed to Accepted on
   > YYYY-MM-DD, after FM rows {{FM-NNN, …}} were locked at M1/M2
   > and gates A–F of the `promote-adr` skill passed.
   ```
3. Update `project_physa_stack.md` memory (if the ADR is 0002..0005)
   to reflect the Accepted status.
4. Update any ROADMAP.md checkbox that tracks the promotion.

## Commit message

```
docs(adr): promote ADR-NNNN from Proposed to Accepted

FM rows locked: FM-NNN, FM-NNN, ...
Sub-ADRs cleared: ADR-XXXX, ADR-YYYY (or: none).
Gates A–F (promote-adr skill) passed.
```

## Rollback

If a gate was accidentally marked passed and the ADR was promoted in
error, open a PR that reverts the status change and reopen the
original review thread. Do NOT silently edit the file back — the
promotion is a published decision.

## What NOT to do

- Do not promote an ADR because "it seems fine". The gates exist
  specifically to catch the subtle cases where an FM row depends on
  research that hasn't landed.
- Do not bundle promotion with other changes in a single PR. The
  promotion diff must be minimal (status + note block) so reviewers
  can audit the gate outcomes in isolation.
- Do not promote ADR-0001 or ADR-0006 — they are process ADRs, not
  architectural, and not subject to §15.
- Do not mark an ADR Accepted as part of a `promote-adr` skill
  invocation by a non-human caller. Confirm with the user first.
