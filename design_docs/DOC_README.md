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
- **The invariant is care granularity, not metaphysical person purity**
  (relaxed 2026-07-30; revised 2026-08-13; wing founding record §1). Paredros
  is care for **individuals**: particular others you know. Ordinary play stays
  with one named creature until death. Control may shift through an explicit
  world event or optional player rule, with its consequences recorded. Drift
  means free roster control or care widening to a squad you administer, which
  is Isometry's granularity.

## Active docs

| Doc | What it is |
| --- | ---------- |
| [DOC_POLICY.md](DOC_POLICY.md) | Documentation governance |
| [PROJECT_DESCRIPTION.md](PROJECT_DESCRIPTION.md) | Product goals and pillars (maintainer-owned, revised by instruction 2026-08-13): one named life in a persistent generated world; autonomous inhabitants; control changes only through death, an explicit world event, or an optional player rule; culture has pointable causes. |
| [2026-07-30_paredros_founding_plan.md](2026-07-30_paredros_founding_plan.md) | Vessel 2's active founding record, revised 2026-08-13: one embodied life among autonomous named creatures; persistent settlements, dungeons, ruins, and surface/underground places; cooperation without party control; construction as a world verb; causal culture; recorded and socially contested body continuity; composable subject/body/role/lineage facts; wing license split, tone, and Nemesis-patent constraint. Its P0-P5 phase section is preserved as superseded history. |
| [2026-08-07_paredros_execution_plan.md](2026-08-07_paredros_execution_plan.md) | **The executable plan; rebased 2026-08-13.** S0-S3 remain landed foundation receipts rather than the required game loop. Future ordering follows fundamental layers: persistent world, one embodied life, other autonomous lives, memory/standing, coordination, material life, settlement/culture, danger, and death/continuation. Ordinary control stays with one named creature until death; tag-in is an optional player rule or explicit world process. R4 was decided 2026-08-10. **R1 shared traversal landed 2026-08-20 as an opt-in proof:** the real S0 perspective camera and room consume Mesocosm's exact brick ABI and DDA implementation, with a headed capture, 64-frame timing receipt, unchanged replay hash, and zero steady brick upload. **V1 continuous-zoom residency landed 2026-08-21:** a 96-frame headed Paredros planning scene keeps its 127-voxel visible radius within a 1 MiB exact-page budget and recovers one frame after abrupt zooms. It also proves the current physical path is whole-map CPU upload and texture replacement, so a stable `ResidentChunk`-backed cache with explicit projection revision is the permanent seam before travel. **F0a persistent structure landed 2026-08-21:** `paredros-world` owns stable surface/underground slots, generated site meanings, routes and containment, same-slot inherited replacement, multi-author material edits, and regrow-plus-replay persistence. Its four-place journey is a graph receipt; embodied travel remains the next F0 gate. The direct Mesocosm path dependencies remain product evidence, not an ownership ruling. |
| [2026-08-10_r4_extraction_review.md](2026-08-10_r4_extraction_review.md) | **R4 decided 2026-08-10.** The wing frame adopted symmetrically: each vessel is a mode of the same peopled history (facts cross via the pipeline, platform organs go up the stack, verbs never cross). Tenancy seam pushed **up into netrender** and landed there the same day (`TenantNeeds` + `boot_shared`/`boot_on`, contract documented, four receipts; paredros-room consumes it, mesocosm G2 is the second consumer). `paredros-identity` **promoted to the wing identity crate** in place (MIT OR Apache-2.0). Consequence grammar extraction **refused on principle**. mesocosm-mesh already shared; place identity joins via the pipeline, not a crate. Founding record amended. **All four rulings executed.** |

## Archive

None yet. Retired plans go to `archive_docs/<YYYY-MM-DD>/`.
