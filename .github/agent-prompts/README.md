# Agent prompts

Reusable, versioned prompts for specific agent-driven tasks. Each prompt lives in its own file and is referenced from the matching issue template or label.

## When to add a prompt here

- A task is repeated frequently (e.g., "profile this new competitor", "run benchmark X and file results").
- The task has a non-obvious method or tooling chain.
- The output needs a precise format (file path, schema, tone).

## Initial prompts to author

- `profile-competitor.md` — walk an agent through producing a `docs/research/competitors/<name>.md` file from the `_template.md`.
- `mine-pain-points.md` — systematic search of Reddit/HN/X/forum for a given competitor and appending to `docs/research/pain-points.md`.
- `benchmark-run.md` — reproducible LDBC SNB run on a given commit.
- `bootstrap-adr.md` — scaffold a new ADR from a design discussion.

These prompts double as **issue acceptance criteria** for the tasks they automate.
