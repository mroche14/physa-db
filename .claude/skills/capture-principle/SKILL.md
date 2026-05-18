---
name: capture-principle
description: >
  Capture a cross-cutting principle the user (or the design) just stated —
  multi-tenancy ("every request must carry a tenant"), observability ("every
  RPC must emit a span"), performance budgets, configuration discipline,
  error semantics, security defaults. Writes a new PRIN-NNN-<slug>.md under
  docs/principles/ with the canonical frontmatter and lifecycle status
  `draft`. Auto-invoke whenever the user states a cross-cutting rule that
  will constrain future features. The skill itself gates whether the
  utterance is actually cross-cutting before writing anything.
when_to_use: >
  Phrases like "every request should…", "all features must…", "from now on…",
  "we always…", "we never…", "the principle is…", "as a rule…",
  "must include a tenant", "must emit metrics", "must be observable",
  "must be configurable", "must respect the budget". Also invoke explicitly
  via /capture-principle <one-line statement>.
argument-hint: "[one-line principle statement]"
user-invocable: true
allowed-tools:
  - Bash(ls *)
  - Bash(date *)
  - Bash(git diff*)
  - Bash(git log*)
  - Read
  - Write
  - Edit
  - Grep
---

# capture-principle — log a cross-cutting invariant before it is lost

**Draft statement:** $ARGUMENTS

A principle is a project-wide invariant: a rule that will constrain
many future PRs, not a one-off decision. This skill captures it the
moment it surfaces so it survives context compaction and ends up in
`AGENTS.md` or an ADR.

## Step 1 — Is this really a cross-cutting principle?

Refuse to file the principle unless **all** of these are true:

- [ ] It constrains code in more than one subsystem (storage *and*
      query, or every public request path).
- [ ] It is phrased as an **invariant**, not a task ("every request
      carries a tenant" — not "add tenant to the user service").
- [ ] It is **mechanically checkable** in principle (a lint, a test, a
      review checklist could enforce it).
- [ ] It is **not already covered** by `AGENTS.md` or an existing
      `PRIN-*.md` (`grep -r "<keyword>" AGENTS.md docs/principles/`).

If any box is unchecked, this is not a principle — it is a task or an
ADR. Route the user to `/file-issue` or `/write-adr` and stop.

## Step 2 — Pick the next ID and slug

```bash
ls docs/principles/PRIN-*.md 2>/dev/null \
  | sed -E 's|.*/PRIN-([0-9]+)-.*|\1|' \
  | sort -n | tail -1
# → next ID = max + 1, zero-padded to 3 digits (PRIN-001, PRIN-014, …)
```

Slug rules: kebab-case, ≤ 5 words, action or noun-phrase form. Examples
of good slugs: `tenancy-on-every-request`, `span-per-rpc`,
`config-via-env-with-defaults`. Bad: `tenants`, `obs-stuff`,
`config-thing-i-was-talking-about`.

## Step 3 — Confirm with the user via `AskUserQuestion` (mandatory)

Never write the file without an explicit, structured confirmation.
Call `AskUserQuestion` with the proposed statement quoted verbatim so
the user can audit the exact line that will be committed.

**Required tool call shape:**

```jsonc
AskUserQuestion({
  questions: [{
    question: "Record this as PRIN-NNN?\n\n> <one-line statement, verbatim>\n\nApplies to: <surfaces>. Status will be 'draft'.",
    header: "Capture?",
    multiSelect: false,
    options: [
      { label: "Yes — capture as written",
        description: "Write PRIN-NNN-<slug>.md with this exact statement. Status: draft. Review at session end with /update-principles." },
      { label: "Refine the wording first",
        description: "Statement is on the right track but not falsifiable / not specific enough. Loop back to Step 3 with a tighter draft." },
      { label: "No — not a cross-cutting principle",
        description: "This is a task or a one-off decision. Route to /file-issue or /write-adr instead. Nothing is written." },
    ],
  }],
})
```

Decision routing:

- **Yes — capture as written** → proceed to Step 4.
- **Refine the wording first** → rewrite the one-line statement
  (tighter, falsifiable, present tense), then call `AskUserQuestion`
  again. Do not write anything until the user picks "Yes".
- **No — not a cross-cutting principle** → stop. Suggest
  `/file-issue` or `/write-adr` and exit without touching the
  filesystem.

If the statement is hand-wavy ("we should care about perf"), do not
even reach `AskUserQuestion` — push back first and ask the user to
make it falsifiable ("every public read path must return in < 1 ms
p50 on the smoke workload"). Vague principles are worse than no
principles — they create false confidence.

## Step 4 — Write `docs/principles/PRIN-NNN-<slug>.md`

Use this exact template. Every field is required; "N/A" is a valid
value but must say WHY.

```markdown
---
id: PRIN-NNN
title: <one-line invariant — falsifiable, present tense>
status: draft
captured: YYYY-MM-DD
applies-to:
  - <surface 1, e.g. request-path>
  - <surface 2, e.g. storage>
related-fm: []          # FM-NNN rows this principle constrains, if any
related-adr: []         # ADRs that already apply this principle
graduated-to: null      # filled in when promoted to AGENTS.md §X or ADR-NNNN
---

## Statement

<The single sentence. Identical to `title` above. Repeated here so a
reviewer reading only the body sees it.>

## Rationale (first principles)

<Why is this true? Derive it from positioning / workloads / a hard
constraint — not "because it feels right". One paragraph max. If you
cannot derive it, the principle is not ready; downgrade to a `draft`
question and revisit.>

## Applies to

<Concrete surfaces. File globs, crate names, request paths, ADR refs.
A reviewer should be able to grep for the surfaces and check
compliance.>

- `crates/<x>/src/<path>`
- Every handler in `crates/server/src/api/`
- Any ADR whose decision touches <surface>

## Examples

- ✓ <one-line conforming example — paste real code if it exists>
- ✗ <one-line violating example — the kind of thing review must catch>

## How it was surfaced

<One paragraph. Session date, the conversation that produced it (no
verbatim quotes longer than one sentence), the PR or issue that made
it visible. This is the audit trail — keep it factual.>

## Open questions before graduation

<What is not yet settled. The /update-principles skill checks these
boxes before promoting `draft` → `active` → `graduated`. Use real,
answerable questions, not "TBD".>

- [ ] Validated in at least one PR
- [ ] Mechanically checkable (lint, test, or review checklist drafted)
- [ ] No conflict with existing AGENTS.md rules or ADRs
- [ ] <principle-specific question>
```

## Step 5 — Update the index in `docs/principles/README.md`

Append a row to the index table:

```markdown
| PRIN-NNN | <title> | draft | <surfaces, comma-sep> | — |
```

Do not regenerate the whole index — `/update-principles` does that at
session end. Just append.

## Step 6 — Tell the user what happens next

Reply with two sentences:

1. "Captured as PRIN-NNN (draft). Run `/update-principles` before
   ending the session to review and propose graduation."
2. "Until graduated, this principle is advisory — note it in
   `/review-pr` and `/plan-feature` but do not block on it."

Do **not** commit the file. The user (or the end-of-session
`/update-principles` run) decides when to stage and commit. A captured
principle that doesn't survive a session's review is more valuable
deleted than committed.

## What NOT to do

- **Do not** capture tasks as principles. "Add tenancy to the user
  service" is a `/file-issue`, not a principle.
- **Do not** capture project-wide statements that already live in
  `AGENTS.md`. Quote the existing section and stop.
- **Do not** invent the principle yourself. A principle is what the
  user (or a human reviewer) just asserted; the skill records it. If
  the agent thinks something *should* be a principle, raise it as a
  question first.
- **Do not** auto-promote to `active` or `graduated` in this skill.
  Promotion is `/update-principles`' job — and it requires a human
  decision.
- **Do not** include personal information (emails, hostnames, paths
  under `$HOME`) in the "How it was surfaced" section. `docs/` is
  public. Reference contributors by GitHub handle only (AGENTS.md §10).
- **Do not** silently overwrite an existing `PRIN-NNN-*.md`. If the
  number is taken, recompute. The numbering is monotonic and never
  reused — even for retired principles.
- **Do not** capture more than one principle per invocation. One
  statement → one file. If the user asserted two things, run the skill
  twice.
