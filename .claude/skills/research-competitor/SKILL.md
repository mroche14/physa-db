---
name: research-competitor
description: >
  Produce a competitor profile for physa-db under the strict research-privacy
  rules (AGENTS.md §7, ADR-0006). Forces codename-only references, private
  storage, and attribution-free public deltas. Refuses to write anything that
  could leak real competitor names into public files.
when_to_use: >
  "profile competitor", "research competitor", "analyse X", "compare us to Y",
  any time a competitor needs to be characterised. Never use this for public
  communication or blog drafts — use `docs/requirements/` directly for those.
argument-hint: "[codename] [real-name]"
user-invocable: true
disable-model-invocation: true
---

# research-competitor — private input, public output

**Arguments:** $ARGUMENTS

This skill is *model-invocation disabled*: only the user may invoke it. Reason:
competitor research involves legal-sensitive characterisation. A misfired
auto-invocation that leaks a real name into a committed file is a §10
violation.

## Step 0 — Verify codename assignment

- The codename (first argument) must be a single uppercase word: `ALPHA`,
  `BRAVO`, `CHARLIE`, etc. Pick the next unused letter in sequence by
  consulting `private/research/codenames.md` (local file only — NEVER
  committed).
- The real-name (second argument) is used ONLY to populate the local codename
  table and to drive the research. It never appears in any file under
  `private/research/competitors/<CODENAME>.md` either.
- If `private/research/codenames.md` does not exist, create it locally with
  this header:

```markdown
# Codename ↔ real-name map

> LOCAL ONLY. Never committed. Never shared outside the founder + his tooling.

| Codename | Real name | Assigned on |
|----------|-----------|-------------|
| ALPHA    | {{…}}     | {{YYYY-MM-DD}} |
```

Add the new mapping row.

## Step 1 — Write the PRIVATE profile

Path: `private/research/competitors/<CODENAME>.md`.

**All references inside this file use the codename, not the real name.** Even
the private file avoids the real name so that an accidental copy-paste into a
public context fails closed.

Template:

```markdown
# Competitor <CODENAME>

> PRIVATE — do not commit, do not share.

## Positioning (one paragraph)

What <CODENAME> claims to be, who they target.

## Feature inventory

A bulleted list of every feature worth comparing against physa-db's
feature-matrix rows. Mark each as:
- strength — they are measurably better than our current plan
- parity — comparable
- gap — we are better or they don't have it

Cross-reference `FM-NNN` where applicable.

## Performance claims

Latency / throughput / scale numbers they publish, with citation URLs.
If they refuse to publish numbers, note that.

## Licence and pricing model

Licence string. Any paid tier / seat cost / usage metering.

## Community pain points (from public sources only)

One paragraph per recurring complaint surfaced on Reddit / X / HN /
GitHub issues / forums. Each entry:
- theme (e.g. "cold-start memory bloat")
- evidence strength (single anecdote vs recurring pattern)
- would physa-db's current plan address it? cite FM-NNN or note "gap"

NO direct quotes. Paraphrase. Never link to a user's profile.

## Risks to physa-db

- Feature they ship that we are slow to match
- Migration story they offer that we don't

## What we will NOT copy

Explicit list of <CODENAME>'s choices we consider wrong on first
principles. Briefly state why.
```

## Step 2 — Extract the PUBLIC delta

From the private profile, derive additions to `docs/requirements/`:

- New `FM-NNN` rows for features we confirm matter (add to
  `docs/requirements/feature-matrix.md` — no competitor attribution,
  anchor to a workload or commercial pillar).
- Performance targets (add to `docs/requirements/performance-targets.md`)
  with numerical bounds, NOT "as fast as <CODENAME>".
- Non-goals surfaced by what <CODENAME> over-invests in (add to
  `docs/requirements/non-goals.md`).

**Public writing rules (no exceptions):**

- No real names.
- No codenames either (codenames are private-only; a codename in public
  invites decoding).
- No direct quotes from users, even paraphrased close-to-original.
- No "because <CODENAME> does X". Phrase every row as a first-principles
  or workload-derived requirement.
- If a rewrite cannot avoid attribution, the requirement is probably too
  specific and should be generalised.

## Step 3 — Safety checks before saving

- [ ] The private file is under `private/` (verify path prefix).
- [ ] `just check-private` passes (no `private/` path staged for commit).
- [ ] The public diff mentions no real names — grep the diff for the
      real name; fail if a match is found.
- [ ] The public diff mentions no codenames either.

## Step 4 — Produce a session summary

Output, to the user in chat (not to a file):

```
Competitor <CODENAME> profile complete.
- Private file: private/research/competitors/<CODENAME>.md
- Public deltas (unstaged diff):
  - feature-matrix.md: N rows added (FM-NNN … FM-MMM)
  - performance-targets.md: N targets added
  - non-goals.md: N clarifications added
- Pain-point themes surfaced: X
- Privacy checks: all passed.
```

If any safety check failed, STOP, report the violation, and fix before
proceeding. Never commit a leak.

## What NOT to do

- Do not invoke this skill for a non-competitor (e.g. a research paper
  or a neutral library). Those go straight to public notes.
- Do not use the real name anywhere in the profile, even under "private".
- Do not publish (even in a PR description) a paragraph that names the
  competitor — §7 prohibits any public attribution.
- Do not skip the codename-assignment step. Agents doing research without
  a codename will corrupt the private archive.
