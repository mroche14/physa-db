---
name: pre-commit-check
description: >
  Run all physa-db pre-commit gates in one pass — formatting, clippy with
  -D warnings, tests, doc tests, private/ path check, conventional-commit
  message format, and a secrets + PII scan. Must pass locally before any
  commit. Runs the CI core gate plus additional pre-commit-only checks
  such as secrets / PII scanning and commit-message validation.
when_to_use: >
  "pre-commit", "check before commit", "ready to commit", right before
  staging a change for commit.
argument-hint: ""
user-invocable: true
allowed-tools:
  - Bash(just *)
  - Bash(git *)
  - Bash(cargo *)
  - Bash(grep *)
  - Bash(rg *)
  - Bash(awk *)
  - Read
  - Grep
---

# pre-commit-check — local gate aligned with CI core

Run every gate below **in order** and report a single pass/fail summary at
the end. Do not skip gates; if one fails, stop, report, and wait for the
user to fix.

This skill is intentionally a **superset** of the CI core gate. CI checks the
shared workspace gate; this skill also checks local-only concerns such as
staged secrets and commit-message shape.

## Gate 1 — Working tree sanity

```bash
git status --porcelain
git diff --cached --stat
```

Verify:
- no untracked `private/` paths staged;
- no very large files (> 1 MB) staged that shouldn't be (use `git
  ls-files --cached -s` and look at the size column);
- no `.env`, `.pem`, `id_rsa`, `credentials.json` equivalents staged.

## Gate 2 — Formatting

```bash
just fmt-check
```

If it fails, run `just fmt` and re-stage.

## Gate 3 — Clippy with -D warnings

```bash
just lint
```

No warnings allowed (`AGENTS.md` §4). If clippy flags something, fix the
code — do NOT silence it with `#[allow(...)]` unless you add a comment
explaining WHY the allow is correct in this case (§17-style note).

## Gate 4 — Tests

```bash
just test
just test-doc
```

Both must pass. A red test is a hard stop.

## Gate 5 — Privacy check (§7, ADR-0006)

```bash
just check-private
```

This refuses the commit if any path under `private/` is staged. Never
bypass with `--no-verify`.

## Gate 6 — Secrets & PII scan

Run a ripgrep sweep for likely-secret patterns over the staged diff:

```bash
git diff --cached | rg -i '(api[_-]?key|secret|token|password|bearer|private[_-]?key|BEGIN (RSA|OPENSSH|PGP))' || echo "no obvious secrets"
```

Review every match. A false positive is fine; a true match MUST be
replaced with an env-var indirection before commit.

Then run the PII sweep (AGENTS.md §10). Two checks:

**Check 6a — email literals in the staged diff.** Any real-address
email is a hard stop; only GitHub noreply aliases and lines listed in
`.pii-allowlist` are permitted.

```bash
# Extract staged additions, strip noreply aliases, match emails,
# suppress allowlisted literals.
ALLOWLIST=.pii-allowlist
ALLOWLIST_ARGS=()
[[ -f "$ALLOWLIST" ]] && ALLOWLIST_ARGS=(-v -f "$ALLOWLIST")

PII_HITS="$(git diff --cached -U0 \
  | rg '^\+' \
  | rg -v '^\+\+\+' \
  | rg -o '[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}' \
  | rg -v '@users\.noreply\.github\.com$' \
  | (if [[ -f "$ALLOWLIST" ]]; then rg -v -f "$ALLOWLIST"; else cat; fi))"

if [[ -n "$PII_HITS" ]]; then
  echo "BLOCKED: PII email literals in staged diff:"
  echo "$PII_HITS"
  echo "Fix: remove the literal, refer to the GitHub handle, or add a"
  echo "deliberate allow-line to .pii-allowlist with a comment explaining why."
  exit 1
fi
```

**Check 6b — banned identity source in staged skill bodies.** A
committed skill file MUST NOT call `git config --get user.email` from
an executable code block, because the value will later be published
by whatever public-surface writer the skill feeds. The only allowed
usage is in documentation text that explicitly forbids the pattern.

```bash
SKILL_LEAKS="$(git diff --cached --name-only --diff-filter=AM \
  | rg '\.claude/skills/.*\.md$|\.github/agent-prompts/' \
  | while read -r f; do
      [[ -f "$f" ]] || continue
      # Flag the pattern inside ```bash fences, ignore it in prose.
      awk '
        /^```bash/ { in_block = 1; next }
        /^```/     { in_block = 0; next }
        in_block && /git config[^#]*user\.email/ { print FILENAME ":" NR ": " $0 }
      ' "$f"
    done)"

if [[ -n "$SKILL_LEAKS" ]]; then
  echo "BLOCKED: skill body reads user.email in an executable block:"
  echo "$SKILL_LEAKS"
  echo "Fix: use the resolver chain from AGENTS.md §10 (physa.agent-id"
  echo "→ gh api user --jq .login → interactive prompt)."
  exit 1
fi
```

`.pii-allowlist` format: one regex per line, `#`-comments allowed.
Use sparingly — every entry weakens the gate. Typical case: a test
fixture that intentionally embeds a fake email.

## Gate 7 — Conventional commit message

If a commit message is being drafted (the user shared it, or it's in
`.git/COMMIT_EDITMSG`):

- Verify the prefix is one of `feat(scope): …`, `fix(scope): …`,
  `perf(scope): …`, `refactor(scope): …`, `docs: …`, `test: …`,
  `bench: …`, `chore(scope): …`.
- The scope should match a workspace crate (`physa-core`, `physa-query`,
  …) or a cross-cutting area (`docs`, `ci`, `deps`).
- Subject line ≤ 72 chars.
- Body wraps at 100 chars.
- No trailing period on the subject.

Release-plz relies on this format (`AGENTS.md` §14).

## Gate 8 — Link integrity (cheap check)

If the diff touches any `.md` file:

```bash
# report broken relative links in staged Markdown files
git diff --cached --name-only --diff-filter=AM | grep '\.md$' | while read f; do
    [ -f "$f" ] || continue
    echo "--- $f"
    grep -nE '\]\(\.\./|\./' "$f" || true
done
```

Inspect the output. A broken relative link after a file move is a
common PR failure — fix before committing.

## Output

Print a final table like:

```
Gate                         | Result
-----------------------------|--------
1 Working tree sanity        | PASS
2 Formatting                 | PASS
3 Clippy -D warnings         | PASS
4 Tests                      | PASS
5 Privacy check              | PASS
6 Secrets & PII scan         | PASS (0 matches)
7 Conventional commit        | N/A (no message drafted)
8 Markdown link integrity    | PASS

All gates passed. Safe to commit.
```

If ANY gate failed, output:

```
BLOCKED: gate N failed.
<details>
Please fix and re-run pre-commit-check.
```

## What NOT to do

- Do not run `git commit --no-verify` to bypass failures. Ever.
- Do not silence clippy warnings with `#[allow]` just to pass gate 3.
- Do not stage files under `private/` to "just test something" — use
  `/tmp/` or an unversioned path.
- Do not commit if gate 5 fails. Leaking `private/` is a §10 violation.
