---
name: research-competitor
description: >
  Produce a competitor profile for physa-db under the strict research-privacy
  rules (AGENTS.md §7, ADR-0006). Forces codename-only references, private
  storage, attribution-free public output, and first-principles feature
  selection via a cross-competitor aggregate matrix.
when_to_use: >
  "profile competitor", "research competitor", "analyse X", "compare us to Y",
  any time a competitor needs to be characterised. Never use this for public
  communication or blog drafts — use `docs/requirements/` directly for those.
argument-hint: "[codename] [real-name]"
user-invocable: true
disable-model-invocation: true
---

# research-competitor — extract, aggregate, inspire, create from scratch

**Arguments:** $ARGUMENTS

This skill is *model-invocation disabled*: only the user may invoke it. Reason:
competitor research involves legal-sensitive characterisation. A misfired
auto-invocation that leaks a real name into a committed file is a §10
violation.

## Workflow (four phases)

1. **Extraction** — cast the widest public-source net; pull every feature, pain-point, and engineering choice observed.
2. **Aggregate** — merge into one cross-competitor feature menu (`AFM-NNN` ids, attribution-free).
3. **Inspiration** — mark each aggregate row as `adopt` / `redesign` / `non-goal` / `open`, reasoning from first principles.
4. **From-scratch creation** — physa-db's own FM rows, perf targets, non-goals, and implementation ADRs, justified by workload + principles, never by "because <CODENAME> does X".

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

## Step 1 — Extraction: cast the widest source net

Partial coverage produces partial conclusions. For each competitor, **every** source class below must be mined:

- **Code** — if OSS, clone or browse the repo. Read the implementation: storage layout, query planner, clustering code, test matrix, benchmark harness. Docs describe intent; code describes reality.
- **Official** — product site, engineering blog, whitepapers, conference talks, design docs, release notes.
- **Community** — Reddit (subreddits + ranked threads), X/Twitter (engineering accounts, thread replies), Hacker News (front-page posts + comments), Stack Overflow (tag trends, recurring pain), Discourse / Slack / Discord archives where public.
- **Issue trackers** — GitHub / GitLab issues, discussions, PR debates. `closed-as-wontfix` and long-running unresolved issues are often more informative than shipped features.
- **Independent benchmarks** — third-party perf reports. Weight independent > vendor-authored.
- **Implementation patterns** — the **engineering choice** for each feature (data structures, wire protocols, consensus algorithms, GC strategies, concrete file formats), not just the feature name.

**Version discipline.** Consult every source at the **current stable / main branch** (for code) or the **latest published release line** (for docs / blogs / benchmarks). Pin the exact commit SHA, tag, or release date in the profile's References section — a two-year-old fork exposes a different feature surface. Stale reads produce stale conclusions.

**Documentation trail (exhaustive).** The References section of the private profile lists **every URL consulted**, not only those cited in the body — including dead-ends and "not applicable" reads. For each entry: URL, title, date read, one-line finding. Silent reads are not reads; the next agent must be able to retrace the full search perimeter without re-doing it.

Coverage gaps are surfaced in the private file's "Open questions" section and filed as `type:research` issues — never silently omitted.

## Step 2 — Write the PRIVATE profile

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

A bulleted list of every feature worth comparing. Mark each as:
- strength — measurably better than our current plan
- parity — comparable
- gap — we are better or they don't have it

Each entry also carries:
- **implementation note** — one line on *how* they do it, derived from code-reading when OSS.
- **aggregate ref** — `AFM-NNN` id (see Step 3).

## Performance claims

Latency / throughput / scale numbers they publish, with citation URLs.
Independent benchmarks weighted higher than vendor-authored ones.

## Licence and pricing model

Licence string. Any paid tier / seat cost / usage metering. Explicit licence-trap clauses (SSPL, BSL, commercial add-ons) that block SaaS builders.

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

## Open questions

Things the public sources did not answer. File as `type:research` issues (using codename only).
```

## Step 3 — Aggregate: merge into the cross-competitor matrix

Maintain `docs/requirements/feature-matrix-aggregate.md` — the menu of every feature seen across any competitor, attribution-free. Each row carries a stable `AFM-NNN` id and the following columns:

| Column | Content |
|--------|---------|
| `AFM-NNN` | Stable id |
| Feature | Generic name, no codename |
| Category | Storage / Query / Cluster / Tenancy / Wire / Ops / Tooling / … |
| Seen in N | Raw count (no codename list) |
| Typical implementation | One-line engineering approach, generic |
| Disposition | `adopt` / `redesign` / `non-goal` / `open` |

After each competitor extraction: merge new features as new rows, bump `Seen in N` on existing rows, refine `Typical implementation` when code-reading surfaces a better summary. The file grows monotonically and becomes the canonical feature universe for physa-db selection.

## Step 4 — Inspiration + from-scratch creation

From the aggregate, derive physa-db's public surface:

- **`docs/requirements/feature-matrix.md`** — one row per `adopt` / `redesign` entry, with `AFM-NNN` backref. Each row's justification cites a workload (`W-A..W-F`) or a commercial pillar, never a competitor. First-principles reasoning only.
- **`docs/requirements/performance-targets.md`** — numerical bounds (latency, throughput, memory), never "as fast as <CODENAME>". Targets derived from workload SLOs, informed by observed implementation patterns.
- **`docs/requirements/non-goals.md`** — every `non-goal` from the aggregate, with a one-line first-principles reason.

**Public writing rules (no exceptions):**

- No real names.
- No codenames either (codenames are private-only; a codename in public invites decoding).
- No direct quotes from users, even close paraphrases.
- No "because <CODENAME> does X". Every row is first-principles or workload-derived.
- If a rewrite cannot avoid attribution, the requirement is probably too specific and should be generalised.

**Implementation decisions** land in the relevant ADR (new or amended), referencing the aggregate's `Typical implementation` column and explicitly stating where physa-db diverges and why — Rust-native, multi-tenancy-first, cost-killer, AI-agent-native, etc.

## Step 5 — Safety checks before saving

- [ ] The private file is under `private/` (verify path prefix).
- [ ] `just check-private` passes (no `private/` path staged for commit).
- [ ] The public diff mentions no real names — grep the diff for the real name; fail on match.
- [ ] The public diff mentions no codenames either.
- [ ] The aggregate file has a new or updated row for every feature claimed in the private profile.

## Step 6 — Produce a session summary

Output, to the user in chat (not to a file):

```
Competitor <CODENAME> profile complete.
- Private file: private/research/competitors/<CODENAME>.md
- Aggregate deltas: N new rows, M bumps in feature-matrix-aggregate.md
- Public deltas (unstaged diff):
  - feature-matrix.md: N rows added (FM-NNN … FM-MMM) with AFM-NNN backref
  - performance-targets.md: N targets added
  - non-goals.md: N clarifications added
- Pain-point themes surfaced: X
- Open questions filed: N (link to issues)
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
- Do not stop at docs — if the competitor is OSS, reading the implementation is mandatory. Skipping code-reading defeats Step 1.
- Do not write `feature-matrix.md` rows directly from the private profile. Go through the aggregate (Step 3) so physa-db's selection stays a principled subset of the universe, not a per-competitor echo.
