# Codex skill bridge

Codex discovers repository-scoped skills from `.agents/skills/`.

The project already keeps its canonical skill definitions in `.claude/skills/`.
Each entry in `.agents/skills/` is therefore a symlink to the matching Claude
skill directory. This keeps one source of truth while making the same `SKILL.md`
files visible to Codex.

If you add, rename, or remove a project skill under `.claude/skills/`, refresh
the bridge from the repository root:

```bash
mkdir -p .agents/skills
for skill in .claude/skills/*; do
  [ -d "$skill" ] || continue
  name=$(basename "$skill")
  ln -sfn "../../.claude/skills/$name" ".agents/skills/$name"
done
```
