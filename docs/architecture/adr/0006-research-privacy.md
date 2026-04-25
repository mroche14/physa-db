# ADR-0006: Research & strategy privacy - private input, public output

- **Status:** Accepted
- **Date:** 2026-04-20
- **Features addressed:** FM-122, FM-127
- **Workloads addressed:** W-A, W-B, W-C, W-D, W-E, W-F
- **Context issue:** _(to be filed as `area:infra priority:p1`)_

## Context

The project's success depends on thorough competitive research: which features the market already ships, where users are genuinely unhappy, and where public benchmarks set the bar. That research is sensitive:

1. **Legal exposure.** Characterising peer weaknesses in public files invites defamation risk and misses nuance that can be litigated.
2. **Strategic leak.** A public "here is what others get wrong" document hands peers a free roadmap before physa-db ships.
3. **Community relations.** The OSS ecosystem is small and interconnected. Public trash-talk of peer projects invites retaliation, hurts hiring, and alienates contributors.

At the same time, the *output* of that research - the feature set we commit to and the performance targets we will hit - has to be public so contributors and agents can align. We need a way to keep the input private and publish the output.

The founder considered two mechanisms:

- **Option A**: `.gitignore` the research folder and do not mention it publicly.
- **Option B**: encode competitor names and `.gitignore` the decoder.

## Decision

We adopt Option A with two refinements and absorb the spirit of Option B:

1. All raw competitor research and pain-point mining lives under **`private/`**, which is `.gitignore`d end-to-end. See `AGENTS.md` section 7.
2. The **synthesised, attribution-free output** lives under `docs/requirements/` (public). It describes what physa-db will ship, not what others got wrong.
3. Inside `private/`, competitors are referred to by local aliases whose decoder remains private. This preserves the spirit of Option B, so even accidental screenshots or pasted snippets are harder to map back to real companies.
4. Agents that need competitor context are given the codename map out-of-band by the human coordinator. They must produce artifacts in two buckets: the private per-codename profile (not committed) and the public attribution-free feature-set delta (committed).

### Belt and suspenders

- `.gitignore` covers `/private/`.
- `docs/dev-setup.md` shows `just check-private`, which fails a working-tree check if anything under `private/` is staged.
- A pre-commit hook (M0 scope) rejects any staged path under `private/`.
- `AGENTS.md` section 10 prohibits committing `private/` content without explicit human approval and marks the act as a section 10 violation.

### Future evolution

If the project grows beyond a handful of trusted contributors, we may migrate `private/` to a separate private repository so the main repo never contains strategic material even in gitignored form, and so multiple maintainers can collaborate on research with version control.

## First-principles derivation

### 1. Irreducible constraints

1. Public product documents must stay attribution-free under `AGENTS.md` section 7.
2. Agent and client surfaces covered by FM-122 and FM-127 must not leak hidden research, peer attribution, PII, or policy-only internal material.
3. A private input can feed a public requirement only if the public artifact remains sufficient on its own and does not require private-path references to understand.
4. The operational cost of the guardrail must be low enough that agents actually follow it on every research pass.

### 2. Theoretical optimum

The optimum is a hard separation: sensitive research never enters public versioned files, while the resulting requirements and safety constraints do. That gives the minimum leak surface compatible with public planning. Any weaker design either leaks attribution into public docs or forces public architecture to depend on inaccessible private references.

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- one gitignored private research tree for sensitive inputs;
- one public requirements tree for attribution-free outputs;
- one mechanical check that blocks accidental staging of private files;
- one written rule that agent-visible public surfaces never cite private paths.

### 4. Prior art reused patternwise

This follows the standard pattern used in security-sensitive product planning: keep sensitive inputs private, publish only the neutralized requirement deltas, and enforce the split with both tooling and process. The point is not secrecy for its own sake; it is preserving strategic freedom while keeping public architecture reviewable.

## Consequences

**Positive**
- Public repo contains no attributable negative characterisation of peer projects.
- Strategic conclusions drive the product via `docs/requirements/` without giving anyone a free roadmap.
- Codename discipline limits the blast radius of any accidental leak.
- Clear rule for agents: two artifacts per research task, one private and one public.

**Negative**
- Double-writing: the same insight lands twice, once detailed and private and once synthesised and public.
- If a contributor never sees `private/`, they may repeat work already done. Mitigation: keep `docs/requirements/feature-matrix.md` current.
- Enforcement depends on discipline plus hooks. Hooks still need auditing.

## Alternatives considered

- **Pure encoded names, everything else public.** Rejected: content leaks identity even when names are stripped.
- **Separate private repository from day one.** Rejected for now as overhead before the team is larger; revisit later.
- **Public research with neutral tone.** Rejected: even fair public characterisations leak strategy.

## FM coverage

- FM-122: tool-facing public surfaces must not depend on private research or leak it through schema/evidence endpoints.
- FM-127: PII, redaction, and audited-read posture depend on the same public/private boundary discipline.

## References

- `AGENTS.md` section 7.
- `AGENTS.md` section 10.

## Changelog

- 2026-04-25: Accepted in the Campaign M1-Lock ADR sweep (policy retained; no architectural rewrite).
