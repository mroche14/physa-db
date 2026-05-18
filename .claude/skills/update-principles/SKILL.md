---
name: update-principles
description: >
  End-of-session review for docs/principles/. Walks every PRIN-*.md, asks
  the user to promote draft → active, active → graduated (into AGENTS.md
  or an ADR), or retire; regenerates the index table in
  docs/principles/README.md; and stages a single commit with all the
  changes. Invoke before ending any session that captured or referenced
  principles. The Stop hook nudges if drafts are pending.
when_to_use: >
  "wrap session", "update principles", "review principles", "before I
  stop", "end of session", or any time docs/principles/ has files with
  status:draft after the working portion of the session is done. Also
  triggered by the Stop hook in .claude/settings.json.
argument-hint: "[--dry-run]"
user-invocable: true
disable-model-invocation: false
allowed-tools:
  - Bash(ls *)
  - Bash(grep *)
  - Bash(git diff*)
  - Bash(git status*)
  - Bash(git log*)
  - Bash(git add docs/principles/*)
  - Bash(git commit*)
  - Read
  - Write
  - Edit
---

# update-principles — promote, retire, or refresh the principles log

End-of-session ritual for `docs/principles/`. Three jobs:

1. Walk every `PRIN-*.md` and decide what happens to it next.
2. Regenerate the index table in `docs/principles/README.md`.
3. Stage a single commit so the log moves with the rest of the work.

Run this before any session that touched principles ends. Skipping it
turns the log into write-only debris.

## Step 1 — Inventory

```bash
ls docs/principles/PRIN-*.md 2>/dev/null
grep -lE '^status: draft' docs/principles/PRIN-*.md 2>/dev/null
grep -lE '^status: active' docs/principles/PRIN-*.md 2>/dev/null
grep -lE '^status: graduated' docs/principles/PRIN-*.md 2>/dev/null
```

If there are no principles, regenerate the index (it should show
`_none yet_`) and exit — nothing else to do.

## Step 2 — Walk every `draft` (one `AskUserQuestion` per file)

For each `draft` principle, read the file and call `AskUserQuestion`:

```jsonc
AskUserQuestion({
  questions: [{
    question: "PRIN-NNN — \"<title>\"\n\nStatus: draft. What should happen to it?",
    header: "Draft → ?",
    multiSelect: false,
    options: [
      { label: "Promote to active",
        description: "Validated; must be respected in every future PR. /review-pr will quote it." },
      { label: "Keep as draft",
        description: "Not ready yet. Carries open questions; revisit next session." },
      { label: "Retire",
        description: "Withdraw. File is kept with a retirement note; ID is never reused." },
    ],
  }],
})
```

Do not propose a default. The user picks.

### If "active"

Flip the frontmatter `status` to `active`. Verify all "Open questions
before graduation" boxes are honestly checkable — if any are unchecked
and unanswered, push back and keep as draft.

### If "retire"

Flip `status` to `retired`. Append a one-paragraph "Retirement note"
section at the end explaining what superseded it. Never delete the
file — the audit trail matters.

### If "keep as draft"

No file change. Note in the wrap-up summary that this principle is
still pending and should be revisited next session.

## Step 3 — Walk every `active` (one `AskUserQuestion` per file)

For each `active` principle, call `AskUserQuestion`:

```jsonc
AskUserQuestion({
  questions: [{
    question: "PRIN-NNN — \"<title>\"\n\nStatus: active. Time to graduate?",
    header: "Graduate?",
    multiSelect: false,
    options: [
      { label: "Graduate into AGENTS.md",
        description: "Cross-cutting rule that every agent must respect. Insert as a paragraph; principle file becomes a historical pointer." },
      { label: "Graduate into a new ADR",
        description: "Architecture-specific application of the principle. Hand off to /write-adr; principle is cited in the ADR's first-principles derivation." },
      { label: "Leave active",
        description: "Continues to apply but is not yet ready for the canonical contract. Revisit later." },
    ],
  }],
})
```

### If "graduate into AGENTS.md"

This is the canonical destination for principles that constrain every
agent. Do NOT edit `AGENTS.md` in this skill — propose the patch and
hand off:

1. Pick the section number (usually a new sub-bullet under §1, §10,
   or a new top-level section if the principle is its own pillar).
2. Draft the exact paragraph to insert. Show it to the user.
3. On approval, edit `AGENTS.md`, flip the principle's frontmatter to
   `status: graduated`, fill `graduated-to: AGENTS.md §X`, and leave a
   one-line "Graduated to AGENTS.md §X on YYYY-MM-DD" note at the top
   of the principle body.
4. The principle file stays — it becomes a historical pointer.

### If "graduate into an ADR"

Hand off to `/write-adr`. The new ADR must cite the principle in its
"First principles derivation" section. On the ADR's creation, flip the
principle's `graduated-to: ADR-NNNN` and `status: graduated`.

### If "leave active"

No change. The principle continues to apply.

## Step 4 — Regenerate the index

Rewrite the `## Index` table in `docs/principles/README.md` from the
frontmatter of every `PRIN-*.md`. Columns: ID, Title, Status, Applies
to, Graduated to. Sort ascending by ID. Replace the entire table
between the `<!-- Maintained by /update-principles -->` marker and the
next `##` heading.

Example output:

```markdown
## Index

<!-- Maintained by /update-principles. Do not hand-edit; entries
     are regenerated from the PRIN-*.md frontmatter. -->

| ID | Title | Status | Applies to | Graduated to |
|----|-------|--------|------------|--------------|
| PRIN-001 | Every request carries a tenant | active | request-path, storage | — |
| PRIN-002 | Every RPC emits a span | graduated | server, query | AGENTS.md §4 |
| PRIN-003 | Config is env-only with defaults | draft | server, cli | — |
```

## Step 5 — Stage and commit

If `--dry-run`, stop here and print the diff. Otherwise:

```bash
git add docs/principles/
git add AGENTS.md            # only if Step 3 graduated something into it
git diff --staged --stat
```

Commit message format (conventional commits, `AGENTS.md` §8):

```
docs(principles): refresh principles log (N draft → active, M graduated)

- PRIN-001: draft → active
- PRIN-002: active → graduated (AGENTS.md §4)
- PRIN-003: still draft (open question: <…>)
```

Do **not** push from this skill. The next `/pre-commit-check` →
`gh pr create` flow handles push.

## Step 6 — Wrap-up summary

End the skill with a two-line summary the user can paste into a
session log:

```
Principles updated: <counts>. Drafts still pending: <ids>.
Next session: revisit <ids> or invoke /write-adr for graduated entries.
```

## What NOT to do

- **Do not** silently auto-graduate. Every promotion is a user
  decision; this skill is a facilitator, not a decider.
- **Do not** edit `AGENTS.md` without showing the user the exact
  inserted paragraph first. `AGENTS.md` is the contract — changes need
  explicit approval (and a separate PR review, normally).
- **Do not** delete retired principles. Append a retirement note;
  retain the file.
- **Do not** renumber. PRIN IDs are monotonic and never reused, even
  after retirement.
- **Do not** commit if the only change is whitespace in the index
  table. The skill is allowed to no-op.
- **Do not** include personal information (emails, hostnames, `$HOME`
  paths) in commit messages or principle bodies. `docs/` and git
  history are public (AGENTS.md §10).
- **Do not** run this skill mid-feature. Principles are reviewed at
  session boundaries; mid-feature reviews dilute the signal.
