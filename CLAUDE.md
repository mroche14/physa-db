# CLAUDE.md

physa-db keeps its canonical agent contract in **[AGENTS.md](AGENTS.md)** —
one file, read by Claude Code, Codex, and every other agent. Claude Code
pulls this `CLAUDE.md` in first; when they disagree, AGENTS.md wins.

Before anything else, read AGENTS.md in full, then skim
[`.claude/skills/README.md`](.claude/skills/README.md) for the slash
commands you should prefer over ad-hoc shell.

## The rules you will touch most often

- **§6 — Project tracking.** GitHub Issues are the system of record.
  Never work off-book. The `/next` skill implements the claim protocol.
- **§7 — Research protocol.** Anything under `private/` stays private.
  Public docs are attribution-free.
- **§10 — No credentials, no PII on public surfaces.** This is the one
  most likely to bite you. Do **not** read `git config --get user.email`,
  `whoami`, `hostname`, or `$HOME` paths into any `gh` write (issue
  comment, PR body, release note, commit pushed to origin). Use the
  identity resolver defined in §10 — it returns the GitHub handle, which
  is already public. `/pre-commit-check` Gate 6 enforces this
  mechanically on every staged diff.
- **§11, §12, §15 — First principles, no shortcuts, features first.**
  Skim these before any design work.

## If you are unsure whether something is a public surface

Ask the question the other way: "will this string end up on github.com
as something a stranger can read?" If yes, it's public; route the
identity through the resolver and keep personal info out. When in
doubt, ask the human.
