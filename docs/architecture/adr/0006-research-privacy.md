# ADR-0006: Research & strategy privacy — private input, public output

- **Status:** Accepted
- **Date:** 2026-04-20
- **Context issue:** _(to be filed as `area:infra priority:p1`)_

## Context

The project's success depends on thorough competitive research: which features the market already ships, where users are genuinely unhappy, and where public benchmarks set the bar. That research is sensitive:

1. **Legal exposure.** Characterising competitor weaknesses in public files invites defamation risk and misses nuance that can be litigated.
2. **Strategic leak.** A public "here's what Neo4j gets wrong" document hands competitors a free roadmap to fix their weaknesses before we ship.
3. **Community relations.** The OSS ecosystem is small and interconnected. Public trash-talk of peer projects invites retaliation, hurts hiring, and alienates contributors who may have prior relationships with those projects.

At the same time, the *output* of that research — the feature set we commit to, the performance targets we will hit — has to be public so contributors and agents can align. We need a way to keep the input private and publish the output.

The founder considered two mechanisms:

- **Option A**: `.gitignore` the research folder; do not mention it publicly.
- **Option B**: encode competitor names and `.gitignore` the decoder.

## Decision

We adopt Option A with two refinements (and absorb the spirit of Option B):

1. All raw competitor research and pain-point mining lives under **`private/`**, which is `.gitignore`d end-to-end. See `AGENTS.md` §7.
2. The **synthesised, attribution-free output** lives under `docs/requirements/` (public). It describes what physa-db will ship, not what others got wrong.
3. Inside `private/`, competitors are referred to by **codename** (`ALPHA`, `BRAVO`, …). The codename ↔ real-name table (`private/research/codenames.md`) is local-only. This preserves the spirit of Option B — even the private files avoid real names — so that accidental leaks (screenshots, a file pasted into a public ticket by mistake) are harder to map back to real companies.
4. Agents that need competitor context are given the codename map out-of-band by the human coordinator at prompt time. They must produce artifacts in two buckets: the private per-codename profile (not committed) and the public attribution-free feature-set delta (committed).

### Belt and suspenders

- `.gitignore` covers `/private/`.
- `docs/dev-setup.md` shows `just check-private`, which fails a working-tree check if anything under `private/` is staged.
- A pre-commit hook (M0 scope) rejects any staged path under `private/`.
- `AGENTS.md` §10 prohibits committing `private/` content without explicit human approval and marks the act as a §10 violation.

### Future evolution

If the project grows beyond a handful of trusted contributors, we may migrate `private/` to a **separate private GitHub repository** (e.g. `dynovant/physa-db-internal`) so that the main repo never contains strategic material even in gitignored form, and so that multiple maintainers can collaborate on research with version control. Flagged as a candidate ADR under M4.

## Consequences

**Positive**
- Public repo contains no attributable negative characterisation of peer projects.
- Strategic conclusions drive the product (via `docs/requirements/`) without giving anyone a roadmap for free.
- Codename discipline limits the blast radius of any accidental leak.
- Clear rule for agents: two artifacts per research task, one private, one public.

**Negative**
- Double-writing: the same insight has to land twice (detailed-attributed in private, synthesised-anonymised in public).
- If a non-maintainer contributor never sees `private/`, they may duplicate research already done. Mitigation: well-maintained `docs/requirements/feature-matrix.md` should obviate most duplication.
- Enforcement depends on discipline + hook. Hooks can be bypassed. Audit periodically.

## Alternatives considered

- **Pure encoded names, everything else public.** Rejected: content leaks identity even when names are stripped ("JVM-based system with GC pauses" = Neo4j). Encoding alone is not protection.
- **Separate private repo from day one.** Rejected for now as overhead before the team is larger; revisited when we have > ~3 maintainers.
- **Public research with fair, neutral tone.** Rejected: even rigorously fair characterisations publish our strategic focus.

## References

- DMCA Section 512 considerations when citing competitors.
- OSS community norms: Prometheus-style "we do not benchmark ourselves against competitors in public" has been a workable discipline for similar projects.
