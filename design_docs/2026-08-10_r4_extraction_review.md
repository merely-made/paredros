# R4: The Extraction Review (2026-08-10)

**Status: decided 2026-08-10.** Rulings by Mark, evidence and execution
notes here. Two rulings executed immediately; two queued on sibling
trees that carry in-flight work (see Execution state).

## The charge

Inherited from Mesocosm's archived body-pipeline plan (2026-07-30, R4),
transferred here 2026-08-07 to fire after S3:

> Decide what, if anything, becomes a shared runtime crate, with
> Paredros as the second consumer or not at all. Done when the seam is
> either extracted with two consumers named, or explicitly declined in
> writing.

S3's sim half landed 2026-08-08 and armed it.

## The organizing frame (ruled: adopt, symmetric form)

**Each vessel is a mode of the same peopled history.** Isometry is the
fortress and atlas view: care for a squad and a map, prepared,
adjudicated in turns. Paredros is the walk: the same world's people met
one at a time, embodied, negotiated. Mesocosm is the ecology beneath
both. The Dwarf Fortress caveat binds: DF's modes share one save and
one executable, and the wing's vessels do not. They stay sovereign
games joined by the pipeline laws (choices under scarcity cross,
inheritance is pointable, player history displaces procedural content),
never by a shared running world instance.

What the frame decides for extraction: the things that must be
recognizable across vessels are **people** (subjects and their deeds),
**things** (relics with provenance), and **places** (sites with
history). Facts of the world cross via the pipeline. Platform organs
are shared upward, into the stack that owns them. Verbs never cross.

An amendment recording this belongs in the wing founding record
(`mesocosm/design_docs/2026-07-30_games_wing_founding.md`). **Owed**,
not yet applied: the mesocosm tree carries in-flight work.

## Seam verdicts

### 1. The shared runtime crate (the charge's literal seam)

**Ruled: push the seam up into netrender.** Not a sideways wing crate.

Evidence: `paredros-room/src/gpu.rs` (322 lines) is today's only real
consumer of the tenancy glue: boot one `wgpu::Device` with the union of
netrender's and the tenant's required features, render the tenant into
a texture it owns, composite via `ExternalTextureComposite` at scene-op
boundary zero. Mesocosm-genet is netrender-only today; its renderling
adoption is G2, still open. One real consumer plus one anticipated is
below the two-consumer bar for a wing crate, and the propagate-up
doctrine names netrender as the owning layer anyway: the composition
half (`ExternalTextureComposite`) is already netrender API, so the seam
is finishing a wall netrender already started.

What netrender grows (the design, to land when its tree quiets):

- **Tenant requirements at boot.** Device creation accepts the union of
  extra `wgpu::Features` and limits a tenant needs, so one device
  serves both by construction. Today that union lives in
  `paredros-room::gpu::boot`; it moves behind a netrender API and the
  app states requirements instead of running the adapter dance.
- **The tenancy contract, documented.** One device and queue, tenant
  renders to its own texture, composite at a stated scene-op boundary,
  neutral span names, and the tenant identity recorded in receipts, per
  the cohesion contract (landscape §8.9) and the backend-seam findings
  (netrender-notes 2026-08-04: backend-owned state, neutral spans,
  rasterizer in receipts).
- **What stays app-side:** everything tenant-specific. The renderling
  `Context` on the shared device, stages, cameras, content. Netrender
  never learns what a tenant draws.

Consumers, named: `paredros-room` first (its `gpu.rs` shrinks to the
renderling half), mesocosm-genet's G2 second (lands on the same API
instead of copying the glue).

### 2. The body pipeline (mesocosm-mesh)

**Already shared; no action.** Two real consumers exist now
(mesocosm-genet, paredros-room) and the seam already has a crate
boundary. Paredros consuming it as it stands is the sharing. Moving it
between repos buys nothing today.

### 3. Identity (paredros-identity)

**Ruled: promote now.** `paredros-identity` is the wing's identity
crate: `SubjectId`, `BodyRevisionId`, the facet stores, the control
pointer. The four facts stay four. Promotion executed in place:

- The manifest now says what it is and carries the library grant
  (MIT OR Apache-2.0), per the founding license convention: game code
  is MPL, a reusable library states its own license once its boundary
  is real, and this ruling is the boundary being declared real.
- Sibling vessels design against it from now on. Isometry's dead
  companion as a named campaign figure (S6) and any Mesocosm borg
  continuity name `SubjectId`, not a local id.
- The physical home stays this repository until a second vessel takes
  the dependency; where the crate then lives (and whether it moves to
  a wing-neutral home) is decided by that consumer's build, not in
  advance.

### 4. The consequence grammar

**Ruled: refused on principle, not deferred.** Deeds and premises here,
`Resolved` payloads in Isometry, sortie events in this repo: one causal
grammar (authority resolves once, everyone applies named consequences,
answers cite their evidence), sovereign implementations per vessel.
No shared crate, ever, on the same ground as the general model's
evaluator rule and the murm/moot no-neutral-oplog ruling: a grammar
shared as a library becomes a second authority with a schema.

### 5. Place identity

**No crate; the join is the pipeline.** Mesocosm's `Places` and
Isometry's `Overmap` remain two shapes with different jobs. The
adventure-mode frame names the eventual join: a place crossing vessels
as a fact with history (a sortie's trench surfacing as a campaign
site), which is S6-shaped work, not a shared partition library.

### 6. Wound's home (intra-Paredros note)

`Wound` stays in `paredros-sortie` until S4 forces the combat owner's
final shape. Recorded in the execution plan's findings 2026-08-08; this
review does not move it.

## Execution state

- Framing ruling: recorded here; founding-record amendment **owed** to
  mesocosm when its tree quiets.
- Netrender tenant seam: designed above; implementation **queued** on
  the netrender tree quieting (19 files of in-flight filter and
  paint-list work as of this review).
- Identity promotion: **executed** (manifest, license, docs).
- Grammar refusal: **recorded**, closed.

R4's done-condition is met: every seam is either extracted with its
consumers named (identity, promoted in place; tenancy, named to
netrender with both consumers listed) or declined in writing (grammar,
place identity, mesh already shared).
