# wait-ci predicates library

Each `- [ ]` line in a PR's Test plan section gets classified into one of
four buckets. `/wait-ci` scans the buckets in the order below and stops at
the first match — **order matters**: a line that could be `auto-skip` AND
`auto-verify` is treated as verified, on purpose.

Adding a pattern: append to the correct bucket, include a commented
example of a real PR line that matches. A predicate without an example is
untested and will be rejected in review.

Patterns are `grep -iE` regex fragments. They are matched against the
item text **after** the leading `- [ ] ` has been stripped. Matches are
case-insensitive.

## Bucket 1 — auto-verify

Items covered by a CI job or by `/wait-ci`'s own built-in checks. The
skill flips `[ ]` → `[x]` and appends `— verified by /wait-ci @ <sha>`.

```regex
# "CI green" / "all checks pass" — trivially covered by the CI_VERDICT==green branch.
^(all\s+(required\s+)?(ci\s+)?checks?\s+(are\s+)?green|ci\s+(is\s+)?green|all\s+green)
# example: "- [ ] all required CI checks green"

# "docs-link-check" / "lychee" — covered by the dedicated docs-link-check CI job.
docs[- ]link[- ]?check|lychee\s+(passes|green|clean)
# example: "- [ ] docs-link-check (lychee) passes."

# "privacy check" / "just check-private" — covered by the privacy tree check CI job.
privacy\s+(tree\s+)?check|just\s+check-private
# example: "- [ ] just check-private still passes (no private/ paths in diff)."

# "tests pass" — covered by the "just ci" matrix job running nextest.
^(all\s+)?tests?\s+(pass|green)
# example: "- [ ] tests pass"

# "clippy clean" — covered by "just ci" running clippy -D warnings.
clippy\s+(is\s+)?(clean|green|passes?)|no\s+clippy\s+warnings
# example: "- [ ] clippy clean"

# "conventional commit" — covered by the conventional-commits CI job.
conventional[- ]commit
# example: "- [ ] conventional commits workflow passes"

# "gate 6a" / "gate 6b" / "PII scan self-check" — author ran the gate inline in the session
# and noted "clean" / "confirmed" / "no leak".
gate\s*6[ab]?\b.*(self[- ]check|clean|confirmed|no leak|no email literal)
# example: "- [ ] Gate 6a (PII email scan) self-check on this diff — clean, confirmed."

# "no X remains" / "grep confirms" — author's static grep assertion, reproducible in CI.
grep\s+confirms|no\s+.{0,30}\s+remains|returns\s+only\s+prose\s+mentions
# example: "- [ ] No abbreviation (AC, FM, …) remains in the section — grep confirms."

# "grep/rg <cmd> returns N hits/matches/results/lines" — same intent as the above,
# different phrasing. Narrow: requires the line to name grep or rg, and to end the
# assertion with one of the counted-noun keywords. Zero, no, and 0 are all accepted.
(grep|rg)\s.{0,120}returns?\s+(zero|no|0)\s+(hits?|matches?|results?|lines?)
# example: "- [ ] grep -rE 'foo' . returns zero hits across the working tree."
```

## Bucket 2 — auto-skip (manual-by-design)

Items whose verdict requires a human eyeball or a manual environment.
`[ ]` stays unchecked; the item is listed in the summary as
"needs human verification".

```regex
# Visual smoke / render on GitHub / Mermaid render.
render(s)?\s+(correctly|on\s+github|natively)|mermaid\s+render|manual\s+smoke
# example: "- [ ] Mermaid renders on github.com (manual smoke after merge)."

# "verify X on the deployed site" / "open in browser" / "load the page".
(open|load|verify|check).{0,40}\b(in|on)\s+(the\s+)?(browser|github|deployed|prod)
# example: "- [ ] open in browser and verify the hero image loads"

# "dogfood" / "fresh clone" / "test on clean checkout".
dogfood|fresh\s+clone|clean\s+checkout|from\s+scratch\s+(setup|install)
# example: "- [ ] dogfood /wait-ci on the PR itself to validate the happy path."

# "smoke test" / "smoke-test" not already covered by CI.
^smoke[- ]test|\bsmoke\s+(a|the)\b
# example: "- [ ] smoke-test the install flow on a fresh clone"

# "manual verification" / "human to verify".
(manual|human)\s+(verification|verify|check|inspection)|visually\s+(confirm|check)
# example: "- [ ] visually confirm spacing on narrow viewports"
```

## Bucket 3 — auto-defer (follow-up work)

Items that describe work the PR is explicitly NOT doing — the checklist
is tracking them as follow-up. `[ ]` stays unchecked; the summary marks
them with a `🔁` and suggests a `/file-issue` invocation.

```regex
# "track in a follow-up issue" / "file a follow-up".
(track(ed)?\s+in|file(d)?\s+a?\s*)?follow[- ]?up\s+(issue|ticket|pr)
# example: "- [ ] file a follow-up issue for the anticipation layer"

# "future work" / "later PR" / "future PR".
future\s+(work|pr|iteration)|later\s+pr|out\s+of\s+scope.{0,30}follow
# example: "- [ ] future PR: add predicates for plan-feature's AC fields"

# "backlogged" / "add to backlog".
backlogged?|add\s+to\s+(the\s+)?backlog
# example: "- [ ] backlogged under #52"
```

## Bucket 4 — fallthrough

Anything that matches no pattern. Treated as `auto-skip` (conservative
default): `[ ]` stays unchecked, item listed as "needs human verification".

Reason for the conservative default: an agent that auto-verifies a
predicate it doesn't understand ships false confidence. Better to surface
unfamiliar items than to pretend they're resolved.

## Extension rules

When a predicate needs to be added or changed:

- open a PR that edits this file and *also* touches the SKILL.md
  classifier docs if the bucket semantics are changing;
- every new regex MUST ship with a real PR-line example in a comment on
  the line above;
- keep regexes `grep -iE`-compatible; avoid features that require `rg`
  or PCRE (the skill runs pure `grep`/`awk` for portability);
- prefer narrow patterns over broad ones — false positives in
  `auto-verify` silently check a box the author meant to track; false
  negatives in `auto-skip` just move the item to the manual-pending
  list, which is safe.
