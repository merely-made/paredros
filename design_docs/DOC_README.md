# design_docs Index

Canonical index for `design_docs/`. Per DOC_POLICY §5, this file wins over
any other index and is updated in the same session as any doc change.

## Working principles for AI assistants

- Read `../CLAUDE.md` first for repo role, terminology, and don'ts.
- Verify claims against the codebase and the sibling repos, not doc-to-doc
  consistency.
- Plans carry done-conditions, not time estimates.
- `PROJECT_DESCRIPTION.md` is maintainer-owned; surface contradictions, do
  not edit unasked.
- Wing-level architecture lives in the Mesocosm repo at
  `mesocosm/design_docs/2026-07-30_games_wing_founding.md` and is cited, never
  copied. The three pipeline laws there govern anything crossing between
  games.
- **The invariant is care granularity, not person purity** (relaxed
  2026-07-30; wing founding record §1). Paredros is care for **individuals**:
  particular others you know. Drift means care widening to a squad you
  administer, which is Isometry's granularity. Person may shift — tag-in is
  permitted — provided second person stays home. Forbidden: real-time
  puppeteering of a party.

## Active docs

| Doc | What it is |
| --- | ---------- |
| [DOC_POLICY.md](DOC_POLICY.md) | Documentation governance |
| [PROJECT_DESCRIPTION.md](PROJECT_DESCRIPTION.md) | Product goals and pillars (maintainer-owned): companions retain agency, with bounded tag-in allowed without turning the entourage into a puppeteered party. |
| [2026-07-30_paredros_founding_plan.md](2026-07-30_paredros_founding_plan.md) | Vessel 2's design and phases: companions as peers, offers becoming low-friction standing agreements as trust grows, succession, Heroes of Hammerwatch expeditions, a Ball × Pit/RimWorld base assembled from buildings and named inhabitants, place lineage from camp through city and faction, the layered social simulation, composable critter/body/character engrams, the wing license split, tone, the Nemesis-patent constraint, and P0–P5 done-conditions. |
| [2026-08-07_paredros_execution_plan.md](2026-08-07_paredros_execution_plan.md) | **The executable plan; supersedes the charter's phase section** (audit 2026-08-07: the old order could not test the premise). **In progress: S0 landed 2026-08-08** (`crates/paredros-room`, replay hash and screenshot receipt); **S1 landed 2026-08-08 with its headed judgment open** (`crates/paredros-social` + `crates/paredros-identity`: three companions answer one offer three ways with their premises attached, one standing agreement formed, exercised, renegotiated and ended); **S2 landed 2026-08-08 with its headed judgment open** (settlement as peer agency: homes offered with daily work, residence and the daily round derived from agreement state, moving out = the agreement ending); **S3's sim half landed 2026-08-08** (`crates/paredros-sortie`, the joint receipt: negotiated participation, terrain falls as body-revision wounds, pact-governed tag-in, the dig rule, and a sortie deed flipping a post-sortie answer; headed real-time action open, **R4 armed**); S4, death and succession, next. Identity prerequisites first (SubjectId, BodyRevisionId, controlled-subject session state, character/faction facets = the wing's subject/body/role/lineage). Social willingness and combat execution as **separate owners with one joint receipt**: S0 room probe (renderling tenant, replay hash), S1 the refusal scene (three companions, deeds and explanation from the start), S2 the negotiated home (peer agency before production), **S3 one sortie and return** (the synthesis ruling: tag-in under pressure, injury as body fact, deeds that later explain), S4 death and succession, S5 the settlement that keeps, S6 inheritance and export incl. the inherited tool. Canary binds throughout. |

## Archive

None yet. Retired plans go to `archive_docs/<YYYY-MM-DD>/`.
