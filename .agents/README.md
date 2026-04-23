# Codex skill bridge

Codex discovers repository-scoped skills from `.agents/skills/`. physa-db keeps
its canonical skill definitions in `.claude/skills/`, and `.agents/skills` is a
**single directory-level symlink** to that canonical location:

```
.agents/skills -> ../.claude/skills
```

Every skill that lives under `.claude/skills/<name>/` is therefore automatically
visible to Codex at `.agents/skills/<name>/`. No per-skill symlink to maintain,
no "refresh the bridge" step when you add or remove a skill.

If `.agents/skills/` ever looks broken (empty or showing a stale set of entries)
— most likely because a tool that doesn't follow symlinks well clobbered it —
recreate it from the repository root:

```bash
rm -rf .agents/skills
ln -s ../.claude/skills .agents/skills
git add .agents/skills
```

Invocation syntax differs by agent:

- **Claude Code** — slash commands (`/next`, `/plan-feature`, …).
- **Codex** — prefix invocation (`$next`, `$plan-feature`, …) or "use the named
  skill" in a prompt.

Skill semantics, allowed-tools, and argument hints are identical across agents —
the `SKILL.md` file is the single source of truth.
