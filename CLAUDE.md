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

<!-- flotte:suivi -->
## Suivi de projet

Le suivi vit dans `projet/suivi.yaml`, versionné, appliqué par la skill globale
`plane-project-tracking`. **L'outil de suivi est une vue du dépôt, pas une source
parallèle** : quand les deux divergent, le dépôt fait foi.

À la fin d'un chantier significatif, mettre à jour le YAML puis :

```bash
export PLANE_API_KEY=...   # ~/.config/systemx/infra-secrets.local.md
python3 ~/.claude/skills/plane-project-tracking/plane_sync.py projet/suivi.yaml --diff
```

⚠️ Ne jamais recycler une `cle` : elle est l'identité de l'item côté outil, la changer
crée un doublon au lieu de renommer.
<!-- /flotte:suivi -->

<!-- flotte:secrets -->
## Secrets

Les noms des secrets sont déclarés dans `projet/secrets.yaml`. **Les valeurs vivent dans
Infisical, jamais dans le dépôt** — ni en clair, ni en exemple réaliste, ni dans un
commentaire. Pour exécuter avec les secrets injectés :

```bash
infisical run --env=dev -- <commande>
```

Ajouter un secret : le déclarer dans `projet/secrets.yaml`, puis
`~/.config/systemx/poser-secret.sh <projet> <env> <CLE> "<usage>"`.
<!-- /flotte:secrets -->
