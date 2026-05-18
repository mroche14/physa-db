# Emerging cross-cutting principles

This directory tracks **cross-cutting principles** that surface during
development before they are mature enough to live in
[`AGENTS.md`](../../AGENTS.md) or an ADR. The goal is to stop losing
them: a principle stated in chat ("every request must carry a tenant",
"every public endpoint must emit a span") is captured here the moment
it is uttered, then promoted into the canonical contract once it has
been validated against real code.

Principles are **not** ADRs. An ADR records one architectural decision
with quantitative derivation (`AGENTS.md` §11). A principle is a
cross-cutting invariant that constrains many decisions — multi-tenancy,
observability defaults, configuration discipline, error semantics,
security posture. ADRs apply principles; this directory holds the
principles themselves until they graduate.

## Lifecycle

Every principle moves through a fixed status pipeline:

| Status       | Meaning                                                              |
|--------------|----------------------------------------------------------------------|
| `draft`      | Captured this session, not yet validated against code or other agents |
| `active`     | Reviewed, applied in at least one PR, must be respected going forward |
| `graduated`  | Promoted into `AGENTS.md` or an ADR; this file is now a historical pointer |
| `retired`    | Withdrawn — superseded by a different decision, or proven wrong       |

Graduation is the goal. A principle that stays in `draft` for more than
two sessions is either too vague (rewrite it) or not actually
cross-cutting (retire it).

## File layout

```
docs/principles/
  README.md              # this file — index + conventions
  PRIN-001-<slug>.md     # one principle per file, numbered, kebab-case slug
  PRIN-002-<slug>.md
  ...
```

The numbering is monotonic. Never reuse a retired number — the history
matters.

## Index

<!-- Maintained by /update-principles. Do not hand-edit; entries
     are regenerated from the PRIN-*.md frontmatter. -->

| ID | Title | Status | Applies to | Graduated to |
|----|-------|--------|------------|--------------|
| _none yet_ | | | | |

## Authoring

Do not hand-author principle files. Invoke the
[`capture-principle`](../../.claude/skills/capture-principle/SKILL.md)
skill — it enforces the frontmatter schema, picks the next number, and
makes sure the principle is actually cross-cutting before writing
anything.

## Reviewing

At the end of every working session, invoke
[`update-principles`](../../.claude/skills/update-principles/SKILL.md).
It walks every `draft` entry, asks whether to promote, retire, or
leave it, and regenerates the index above. The Stop hook configured in
[`.claude/settings.json`](../../.claude/settings.json) nudges you if
there are pending drafts.

## Related

- [`../../AGENTS.md`](../../AGENTS.md) §11 (first-principles thinking),
  §15 (features first), §16 (skills)
- [`../architecture/adr/`](../architecture/adr/) — ADRs apply principles
  to specific decisions
- [`../requirements/feature-matrix.md`](../requirements/feature-matrix.md)
  — FM rows reference principles when relevant
