# Paredros Execution Plan (2026-08-07)

**Status: in progress (2026-08-08).** S0 landed; S1 landed with its headed
judgment open; S2 is next. The
[founding plan](2026-07-30_paredros_founding_plan.md)
remains the charter: its rulings (agreements as the interface, tag-in as
rehearsed succession, the puppeteering canary, settlement at
returning-character scale) all bind here. Its phase section is superseded
by this plan, on the 2026-08-07 audit finding that the old order could not
test the premise: P1 introduced socially conditioned offers while deeds,
relationships, and explanation waited until P3, and succession arrived
before any of them. Companions cannot demonstrably act *for reasons* if
the reasons ship two phases after the acting.

**The wing question, in this vessel** (founding record, ruled 2026-08-07):
Paredros asks whether a **community remains itself as control, bodies, and
generations change**. Every gate below is a partial answer.

**The synthesis ruling (Mark, 2026-08-07).** The first real sortie is the
wing's extraction trigger, preferred over another infrastructure proof:
companions negotiate participation, a standing agreement governs tag-in,
combat produces bodily and social consequence, scavenge changes the
settlement, death changes who the player inhabits, and deeds explain later
offers and refusals. The sortie is where everything already designed
meets an actual game.

---

## 1. Identity prerequisites

Per the audit, succession and inheritance are unbuildable until these are
separate facts (they are the wing's four — subject, body, role, lineage —
landed in the founding record 2026-08-07):

- **`SubjectId`**: the continuing person, across bodies and control.
- **`BodyRevisionId`**: which body, at which revision, per the wing
  phenotype contract.
- **Controlled-subject session state**: who the player is being, as
  recorded state with the same discipline as Mesocosm's control pointer
  (moves only through a recorded intent; replays).
- **Character and faction facets kept separate**: a role is held, not
  been. Offices survive their holders, which is half of what community
  continuity means.

Tag-in, succession, chassis replacement, and biological descent are then
four different operations on these four facts, not one muddle.

## 2. The two owners, and their joint receipt

The audit's structural correction, adopted: **social willingness and
combat execution are separate owners.** Willingness (agreements, deeds,
confidence, refusal) never reads combat internals; combat adjudication
never decides what anyone was willing to do. They meet in exactly one
place: the sortie (S3), which is the joint receipt.

The puppeteering canary binds both: the player embodies **one** subject at
a time; companions execute their own agreed parts. Tag-in switches who you
are; it never becomes issuing orders to several characters.

## 3. Gates

### S0 — The room probe (landed 2026-08-08)

One body, one room, close camera, fixed input trace. The renderling
tenant renders on netrender's device per the cohesion contract
(mesocosm landscape §8.9); movement rides the near-tier kinematics
already landed in mesocosm-core (step, stands, sees). Save, reload,
replay, state hash.

**Done when:** the same input trace replays to the same hash across
save/reload; a headed screenshot receipt exists; frame spans are recorded
beside netrender's.

**Done when, 2026-08-08.** All three hold. The probe is
`crates/paredros-room` (lib plus `src/bin/room.rs`), the repo's first
game code.

Determinism: a 64-tick const trace of per-tick headings drives one body
through `near::step` over `Ground`. Two straight runs produce the same
position log; a save taken at tick 32, restored into a freshly grown
world and run to the end, produces the same log, the same final position
`[39, 2, -39]`, and the same hash. Position-log hash
`0x27a905731c6bfc61`, ground hash `0x728a7687af5408a9`, both FNV-1a
(`mesocosm_core::snapshot::hash_bytes`) over postcard bytes. The save
carries the seed and the ground hash rather than the world, and a restore
that regrows a different ground is refused (`ProbeError::GroundDiverged`)
instead of replayed over. Proven by `tests/replay.rs`; 13 tests green,
`cargo clippy --workspace --all-targets -- -D warnings` clean. The hashes
are dated against mesocosm as of this entry, not pinned in an assertion:
they witness a replay, and relief changes upstream are allowed to move
them.

Picture: `ROOM_TRACE=1 cargo run -p paredros-room --bin room` opens a
winit window presenting netrender's composed master, which is the
renderling room composited at scene-op boundary 0 with a vello chrome bar
over it, both on one device. It drives itself from the trace, captures,
and exits. Receipt at `Code/testing/paredros/s0_room.png`, 1280x720, 243
distinct colours, checked in the capture path so a blank frame fails
rather than writing a file.

Frame spans, this machine: `probe_frame` 14 to 21 ms wall (tick, re-shade,
tenant draw, compose, present) against netrender's own `total` of 1.7 to
2.1 ms, of which `vello_render` 1.5 to 1.9 ms, `master_compose` 70 to
85 µs, `dirty_tile_rebuild` 65 to 85 µs, `tile_invalidate` 48 to 58 µs.
The probe's span
dominates because it re-uploads the room's 2,834 triangles every frame to
move the torch; that is the first thing to fix when a frame budget
matters.

The room: `Places::grown(4242, 4, 64)` and `Ground::grow`, then one
`carve` at `[35, 6, -35]` into the first hillside a deterministic outward
ring scan finds with enough overburden to keep a floor, walls, and a
roof. Nine voxels cubed. Nothing about the terrain is Paredros's.

Two deliberate choices are ours and worth naming. The camera backs off
*toward the middle of the room* rather than straight behind the heading:
an over-the-shoulder rig in a nine-voxel chamber puts the eye inside a
wall the moment the body reaches a corner, which the trace does eight
times. And the tenant applies a torch falloff from the eye per vertex,
because greedy meshing turns a wall into one quad and one quad of one
colour has no near side.

### S1 — The refusal scene (landed 2026-08-08, headed judgment open)

Three companions, with reasons. They receive the same offer and respond
differently, and
**every response exposes its premises**. This requires the minimal forms,
present from the start: a deed log (append-only), a relationship fact,
confidence, refusal, and the explanation surface. Minimal is fine;
absent is not — that was the old P1's flaw.

**Done when:** a playtester can say why each of the three answered as
they did, from what the game showed them; and one standing agreement is
formed, exercised, and renegotiated or terminated, all legibly.

**Done when, 2026-08-08: the machine-checkable half holds. The headed
judgment is open.** The scene is `crates/paredros-social` (lib,
`src/bin/refusal.rs`, `tests/refusal.rs`); the §1 identity facts are
`crates/paredros-identity`. 33 tests green across the workspace,
`cargo clippy --workspace --all-targets -- -D warnings` clean.

Aud puts the same offer to three people at tick 8: scout ahead, grade 3,
danger 4, share 2, up to danger 5. Three answers, and the premises say
why. Aud, Bram, Odris and Sela are fixture names for this scene, not
lore.

- **Bram accepts.** Aud stood by him at ticks 1 and 2, which is trust 6
  and liking 4. Scouting asks grade 3 and he holds 4. Danger 4 asks trust
  4 and he holds 6; he would bear 5 for her.
- **Odris refuses.** Aud shared with him at tick 3 and left him at tick 4,
  which is trust -4 against the 4 that danger asks. He is the *best* scout
  of the three, grade 5 against the 3 asked, and refuses anyway. The model
  does not confuse capability with willingness, and
  `the_one_who_refuses_is_the_most_capable_of_the_three` fails if it ever
  starts to.
- **Sela counteroffers**, at share 3 and a danger cap of 3. Three deeds
  (shared, shared, stood by) put her at trust 5 and liking 6, so the ask
  is fine. She is the most careful of the three, and danger 4 is past the
  3 she would carry for anyone. Her answer is the terms she would take
  instead.

The premises are data rather than prose. The test asserts that the three
cited deed sets are non-empty, pairwise disjoint, and made only of deeds
Aud actually did to that person; that the deciding gate is stated; and
that gates never reached are never claimed (Odris stops at trust, so no
danger premise appears in his answer).

The agreement, whole, with the one who accepted. Formed at tick 9 on the
offered terms, after the weighing runs again, so an arrangement nobody
would accept cannot be created. Exercised at tick 10 and performed with no
renegotiation. Renegotiated at tick 11, Bram's proposal, down to share 3
and a danger cap of 3. Asked again at tick 12 at danger 4 and answered
`OutsideTerms`, which is the changed term biting. Ended at tick 13 for
`WorkDone`, with Aud's standing toward Bram and the reason both in the
premises. All four transitions are deeds in the log, and the test walks
the agreement's own history back to each one.

The canary, as a test: `a_standing_agreement_is_not_a_command` forms the
arrangement, records Aud abandoning Bram, and then asks under it. Bram
declines and names the deed that changed. Nothing in the crate can make
anyone act; offers are put and answers come back.

Determinism: building the scene twice and running both the offers and the
whole lifecycle gives equal answers, equal rulings, and equal societies.
Society hash `0x49860851d09fdd84` over 14 deeds, FNV-1a
(`mesocosm_core::snapshot::hash_bytes`) over postcard bytes. Dated here
rather than pinned in an assertion, as S0's are; what the test asserts is
that two runs agree.

**Open: the headed judgment.** Whether a playtester can say why each of
the three answered, from what the game showed them, is not a thing a test
settles. `cargo run -p paredros-social --bin refusal` prints the premises
as lines and is the surface to judge, but nothing has been put in front of
a player and no in-game presentation exists. That half of the
done-condition stays open, to be closed by S2 or by a headed session.

### S2 — The negotiated home

Offer someone housing and work. They accept, refuse, or counteroffer,
from an intelligible history. This is the settlement tested as **peer
agency** before it is tested as production.

**Done when:** at least one refusal and one counteroffer occur for
reasons the player can trace; an accepted agreement changes where someone
lives and what they do daily.

### S3 — One sortie and return (the joint receipt)

A bounded expedition: companions negotiate participation under standing
agreements; real-time embodied action for the played subject with
companions executing their agreed parts; tag-in under pressure; bodily
consequence (injury as body-revision fact); scavenged material returns;
deeds are recorded and later *explain* something.

**Done when:** a full sortie-and-return replays to the same hash; a
tag-in occurs mid-action under a pre-agreed condition; an injury
persists as a body fact; one post-sortie offer or refusal is explained
by a deed from the sortie; and the canary holds (no moment-to-moment
multi-character orders anywhere).

### S4 — Death and succession

Death during action, and a companion becomes the played subject, through
the identity facts of §1.

**Done when:** the first succession lands with `SubjectId` continuity for
the community (the office, agreements, and deeds survive the holder), and
the player minds who they became — the charter's own bar.

### S5 — The settlement that keeps

The founding P4, unchanged in intent: expedition-and-return feeds the
settlement; the settlement's output changes what the played character can
attempt; one building's history makes its effect particular.

### S6 — Inheritance and export

The founding P5, unchanged in intent, now buildable on §1: Mesocosm
critters and RNG critters through one slot (Law C receipt); a dead
companion appears as a named figure in an Isometry campaign; **the
inherited tool** travels the full arc — a tool from a Mesocosm body part
enters with provenance intact, accumulates deeds, and surfaces in
Isometry as an artifact. Shapes become relics; values become factions.

## 4. Stop rules

- The canary above all: real-time puppeteering of a party is drift, full
  stop.
- Willingness and combat stay separately owned; only S3 joins them.
- No universal character schema: the four facts stay four.
- Nothing here grows an engine organ Mesocosm already owns; the room
  probe *consumes* near-tier kinematics and the tenant seam, it does not
  fork them.
- The consequence envelope (intent, adjudication, explicit consequences,
  durable facts, projections) is **anticipated, not extracted**: Paredros
  implements its own transition layer concretely, and extraction waits
  for the third sovereign proof per the general model's evaluator rule.
- **R4 lives here now** (inherited 2026-08-07 from Mesocosm's archived
  body-pipeline plan): the extraction review — what, if anything, becomes
  a shared runtime crate, with Paredros as the second consumer or not at
  all — fires **after S3**, the sortie being the wing's ruled extraction
  trigger. Done when the seam is either extracted with two consumers
  named, or explicitly declined in writing.

## Findings

- **2026-08-08 (S1):** the two-owners rule is a dependency fact, and that
  decided a crate boundary. `SubjectId` cannot live in either owner: if it
  lived in the willingness crate, the combat owner would have to depend on
  willingness to name a person, which is the wall breached from the other
  side. `crates/paredros-identity` is therefore a foundation crate that
  knows about neither, holding `SubjectId`, `BodyRevisionId`, `FactionId`,
  `OfficeId`, `Tick`, the facet stores, and the control pointer. It is the
  one place the four facts are named.
- **2026-08-08 (S1):** storing standing would have made the explanation
  surface a second job. Folding it out of the deed log on each call made
  the surface honest for free: a `Standing` carries the ids of the deeds
  that produced it, so a premise can only ever cite entries that exist.
  The test asserts on ids resolved back through the log, so a rule that
  kept answering correctly while inventing its reasons would fail.
- **2026-08-08 (S1):** refusing weighs (0, 0) in the deed table. That is a
  ruling rather than an omission: refusal is a legitimate answer in this
  vessel, so it costs the refuser nothing, and
  `a_refusal_is_an_outcome_and_costs_the_refuser_nothing` pins it.
- **2026-08-08 (S1):** the played character is admitted to the society the
  same way anyone else is, because succession means the distinction is
  temporary. There is no second record shape for the one being played, and
  the willingness rule never asks who the player is.
- **2026-08-08 (S1):** `paredros-social` takes a **dev**-dependency on
  `mesocosm-core`, for `snapshot::encode` and `hash_bytes` and nothing
  else. Judgment call, recorded because it brushes two rules at once: the
  stop rule forbids growing an organ Mesocosm already owns, so the replay
  receipt consumes the wing's hash rather than writing a second FNV-1a,
  and the shipped dependency list of the willingness owner still contains
  only `serde` and `paredros-identity`.
- **2026-08-08 (S1):** §1's controlled-subject session state landed and S1
  does not use it. `Control` is tested on its own (tag-in, tag-out,
  succession, replay from intents) and waits for S3 and S4. Recorded so it
  is not rediscovered as missing.
- **2026-08-08 (S0):** the stop rule holds in practice. The room probe
  writes no terrain, no kinematics, and no mesher: `Places::grown`,
  `Ground::grow`, `Ground::carve`, `near::step`, `Ground::stands`, and
  `mesh_volume` are all consumed as they stand. What Paredros wrote is
  the room's siting rule, the trace, the camera, and the save
  discipline.
- **2026-08-08 (S0):** `near::step` refuses a blocked move rather than
  sliding along a full-height wall, so a body in a sealed chamber holds
  position at the wall. The trace relies on this, and
  `probe::tests::the_trace_ends_pressed_against_walls` asserts it, so a
  kinematics change upstream that starts letting bodies through walls
  fails here loudly.
- **2026-08-08 (S0):** the execution plan cited a hillside-hunt
  reference at `mesocosm-core/src/places/hunt.rs` (test `the_chase`)
  that does not exist. The nearest real stage-scans are
  `places/near.rs::tests::a_stance` and
  `places/bricks.rs::tests::hills_block_sight_and_tunnels_grant_it`;
  this probe's hunt is modelled on those.
- **2026-08-08 (S0):** the probe's crate uses path dependencies on
  mesocosm, netrender, and the local renderling fork, against the
  workspace convention of branch-tracked git deps. The fork has no
  published home, and S0 exists to consume all three exactly as they
  stand on the machine. Revisit when the probe grows into a shipped
  target.

## Progress

- **2026-08-08:** S1 landed with its headed judgment open.
  `crates/paredros-social` is the willingness owner (deeds, standing,
  confidence, refusal, standing agreements, premises) and
  `crates/paredros-identity` is the foundation both owners share. Three
  companions answer one offer three ways with their reasons attached, and
  one standing agreement is formed, exercised, renegotiated, and ended
  with every transition in the deed log. 33 tests green across the
  workspace, clippy clean. Scene, premises, agreement receipt, and the
  open half recorded in the S1 section above.
- **2026-08-08:** S0 landed. `crates/paredros-room` is the first game
  code in the repo: a room carved into a grown hillside, one body under
  `near::step`, a 64-tick fixed trace, save/reload/replay to a matching
  position-log hash, and a headed winit run presenting netrender's
  composed master with the renderling room inside it. Receipt, hashes,
  and frame spans recorded in the S0 section above.
- **2026-08-07:** founded from the audit (old phase order could not test
  the premise; no action-RPG slice existed; identity facts were missing)
  and Mark's synthesis ruling (the sortie as the wing's extraction
  trigger). Charter rulings preserved; phase section superseded.
