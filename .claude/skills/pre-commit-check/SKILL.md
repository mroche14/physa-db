---
name: pre-commit-check
description: >
  Run all physa-db pre-commit gates in one pass — formatting, clippy with
  -D warnings, tests, doc tests, private/ path check, conventional-commit
  message format, and a secrets scan. Must pass locally before any commit.
  Mirrors what CI will run; if it passes here, it passes in CI.
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

# pre-commit-check — local gate mirroring CI

Run every gate below **in order** and report a single pass/fail summary at
the end. Do not skip gates; if one fails, stop, report, and wait for the
user to fix.

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

## Gate 6 — Secrets scan

Run a quick ripgrep for likely-secret patterns over the staged diff:

```bash
git diff --cached | rg -i '(api[_-]?key|secret|token|password|bearer|private[_-]?key|BEGIN (RSA|OPENSSH|PGP))' || echo "no obvious secrets"
```

Review every match. A false positive is fine; a true match MUST be
replaced with an env-var indirection before commit.

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
6 Secrets scan               | PASS (0 matches)
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
