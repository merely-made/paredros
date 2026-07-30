# Paredros: Founding Plan

**Status: plan, 2026-07-30. Nothing implemented.** Vessel 2 of the games
wing. Shared architecture, the three pipeline laws, and the wing vocabulary
live in the wing founding record at
`mesocosm/design_docs/2026-07-30_games_wing_founding.md` (sibling repo; cited
by path because relative links do not cross repos). They are not repeated
here.

---

## 1. The game

**Second person. I live with you.** You play one character among companions
who are peers. You can address them, persuade them, equip them, and help
them. You cannot pilot them. They outlive you, and when you die one of them
becomes the character you play.

A paredros is the one who sits beside you: the Greek Magical Papyri's
acquired companion, and — the sense that decides the design — in classical
civic use an **assessor seated beside a magistrate**. A colleague, not a
servant. Colleagues also succeed to office.

Scope, stated as a canary: **a single character with a simulated entourage.**
The care granularity here is the **individual** — particular others you know —
and that is the thing that must not drift. Care that widens to a community you
administer is Isometry's granularity, not this one.

### Leverage without command

**Permitted 2026-07-30** by the care-granularity relaxation (see
`mesocosm/design_docs/2026-07-30_games_wing_founding.md` §1, which replaced a
strict person-purity rule with a home-person rule). The earlier wording
forbade the player any influence over a companion's conduct, which was too
strict and left the deployment queue as the only lever. Two mechanics are now
available, and both keep companions peers:

- **Configure, don't command.** Standing behaviour negotiated *in advance*, in
  the FFXII gambit shape: you agree how someone acts, you do not drive them in
  the moment. The difference is the whole point — an agreement is something a
  peer can also refuse, revise, or ignore under stress, which a command
  cannot. Refusal is a legitimate outcome and a good source of character.
- **Tag-in.** Temporarily *becoming* a companion, as Crystal Chronicles and
  Gotcha Force allow. This is succession in miniature, and succession is
  already the spine — becoming a companion for a fight is the same mechanic as
  becoming one permanently, rehearsed.

Still forbidden, and this is what the canary now watches: **real-time
puppeteering of a party.** If the player is issuing moment-to-moment orders to
several characters, care has widened to the squad and the vessel has drifted.

### Shape: expeditions and a settlement that keeps

**Ruled 2026-07-30: lean Heroes of Hammerwatch-ward.** Not a continuous
colony simulation but **runs out and back, against a settlement that
persists and improves from what you bring home.** Hammerwatch's structure is
the reference: a town that upgrades from materials found on runs, heroes who
persist and level between them, and co-op where you bring your own character
into a friend's game.

This resolves a real tension in the concept. A full colony sim is heavy,
wants a commander's attention, and would have dragged this vessel toward
third person — the exact drift the canary above forbids. Expedition-and-return
keeps the player embodied in one character throughout, while still letting the
settlement be the thing that accumulates.

It also puts the wing in one rhythm without making the games alike: Mesocosm
runs generations, Paredros runs expeditions, Isometry runs campaigns. And it
delivers the sortie-and-return through-line that sits under half the
influence set (Crystal Chronicles' caravan year, PSO's Pioneer 2, the
Avenger, the Super Destroyer), where the return trip is where the reward
actually lands.

The co-op lane falls out of the same reference and is the first co-op design
in the wing with a concrete shape: **your character visits another player's
settlement**, which is Hammerwatch's "bring your own hero" and, at world
scale, the graft the lineage model already describes.

### The settlement is assembled, not managed

**Ruled 2026-07-30: the Ball × Pit model.** Hammerwatch supplies the rhythm;
Ball × Pit supplies the *shape of the base*, and it is the better reference
for what the settlement actually is.

There, the base is a spatial assemblage you lay out: buildings produce
resources and grant passive bonuses that carry into the run, each new
character needs a residence built for them, characters move between
structures and adjacency determines how much they collect, and rearranging
costs nothing so the layout stays a live decision rather than a commitment.

That is exactly the weight this vessel wants. It gives:

- **A reason each companion is housed somewhere**, which makes the roster
  spatial and legible without a job-priority grid.
- **Assignment without command.** You decide where someone lives and what
  they are near; you do not issue orders. That is the second-person line held
  in the base layer, not just in the field.
- **A base that is read at a glance**, so returning from an expedition means
  seeing what changed rather than auditing a colony.

The distinction from a colony sim is the whole point: **assemble, upgrade,
and assign — not schedule, prioritise, and micromanage.** If a player is ever
tuning a work-priority matrix, care has widened from individuals to an
administered population, which is Isometry's granularity rather than this
vessel's.

### Succession is the spine

Dead is dead; lineage persists. When your character dies, a companion becomes
the played one. This is why the roster must be people rather than equipment:
the entourage is your cast of future protagonists. You care about them partly
because you will *be* one.

Descent requires more than a rebuild. A line is carried by offspring or by a
tended continuation. Impact on the settlement without anyone carrying the
line is not descent; it is persistence as **tulpa**. Fili records continuity
of line; tulpa records what memory keeps.

### The social layer is the base layer

The player controls one character, so the rest of the roster running a social
simulation is what generates the settlement's life. This is the Crusader
Kings stance — one avatar, a court that simulates around you — pointed at a
workshop.

It also sidesteps the failure that kills most social simulations: they die on
contact with combat, because companions driven by feelings play badly. Here
the social layer never has to make anyone competent in a fight. It has to
make the settlement alive and decide who is in the deployment queue.

**Architecture: a layered hybrid.** Research on 2026-07-30 found five
shipped approaches, each of which caps out alone — opinion ledgers (RimWorld
vanilla; the Psychology mod's existence marks the ceiling), needs and
advertisement autonomy (The Sims: generates activity, never drama), trait
vectors with event accretion (Dwarf Fortress: real depth, illegible without
Legends-mode tooling), rule-based social exchanges (Comme il Faut, Prom Week,
Ensemble, Versu: richest micro-drama ever shipped, never scaled past a cast
of about eighteen), and belief/gossip propagation (Talk of the Town, Shadows
of Doubt, Norland: the underused frontier). The recipe:

- a relationship graph plus an append-only deed log (memories are views over
  the log; an emblem is a distinguished entry)
- Sims-style off-duty autonomy for texture
- a small exchange library fired by a storyteller-cadence director, so drama
  is scheduled rather than simulated into soup
- a belief layer, where characters may act on false beliefs and gossip
  propagates
- **a legibility surface built on day one**, because Dwarf Fortress depth
  nobody notices reads as procedural noise

Bonds carry mechanical weight: hidden compatibility, cohesion from shared
deployments, and bond levels unlocking paired actions — the Crystal
Chronicles spell-fusion shape, expressed on the deployment queue. The Darkest
Dungeon 2 caution applies: relationship drift the player cannot influence
reads as random punishment, so the player must hold levers.

**Legal note:** the Nemesis system (procedurally generated rivals with
promotion hierarchies) is patented to August 2036. Generic grudges and
remembered encounters are fine and have Dwarf Fortress prior art; the
rival-hierarchy-promotion machinery is what to design around.

### Identity in three layers

- **Chassis** — parts and gear, swapped and lost freely
- **Skills** — use-based, accrued to the mind, surviving reassembly, so limb
  loss never costs skill (the Kenshi rule)
- **Quirks and emblems** — event-granted only, never purchasable

Bonds are the fourth thing and are not owned by any of the three.

### Expression and tone

Nonverbal, or Tomodachi Life-style quirk vignettes; gibberish voice is fine.
RimWorld vanilla in tone: sincere, affectionate, mortal. Dark events may
happen; the organ-theft register does not.

### Where the world comes from

Playable on RNG worlds, always (Law C). A world inherited from Mesocosm
displaces the procedural one; it never gates it. Whether Paredros reuses
Isometry's typed worldgen or grows its own is deliberately undecided — the
answer falls out of the world-noun profile extraction, which happens after
the proof pair, not before.

---

## 2. What is genuinely new here

- **Settlement simulation at expedition scale.** Needs, jobs, stockpiles, and
  production, bounded to what a returning character can read rather than a
  colony manager's dashboard. Nothing in the stack does this today, though
  armillary supplies actors, chartulary supplies containment, and codicil's
  append-only log is the right shape for character history.
- **The social hybrid above**, which no shipped game has assembled whole.
- **Real-time embodied combat.** Note that this vessel likely carries the
  wing's heaviest renderer requirement (close camera, 3D), so it does *not*
  simply inherit Mesocosm's lane — Mesocosm is proposed at 2.5D. Renderers are
  per-vessel by rule; see
  `mesocosm/design_docs/2026-07-30_engine_and_render_lane_landscape.md` §5.
  A heavier engine (Fyrox ships the only Rust scene editor) is a live option
  here precisely because it need not be shared.

**Vocabulary guard:** *second person* describes agency — peers you address
rather than units you command. It does not describe the camera, which may sit
close. Never write "first-person Paredros"; say close camera.

Deliberately parked: real-time co-op netcode. Single-player first, so
prediction and rollback stay out of scope.

---

## 3. Phases

Done-conditions, not estimates.

### P0 — One character, one place
Embodied second-person control in a settlement. No companions yet.

**Done when** moving and working in the space is worth doing for its own
sake.

### P1 — Companions as peers
Two or three companions with autonomy, addressable and helpable, never
pilotable.

**Done when** a playtester asks a companion for something rather than
reaching for a command menu.

### P2 — Succession
Death, and a companion becomes the played character.

**Done when** the first succession lands and the player minds who they
became.

### P3 — The deed log and bonds
Relationship graph, append-only deeds, bonds with mechanical weight, and the
legibility surface that makes them visible.

**Done when** a player can explain why two characters are close, from what
the game showed them.

### P4 — The settlement that keeps
Expedition-and-return: what you bring home upgrades the settlement, and the
settlement's output feeds the deployment queue. Production and jobs at the
scale a returning character can read, not a colony manager's dashboard.

**Done when** the settlement's output visibly changes what the played
character can attempt, and a player chooses a run for what it will bring
back rather than for the fight.

### P5 — Inheritance
Accept Mesocosm critters and RNG critters through the same slot; export the
settlement toward Isometry campaign shape.

**Done when** neither import is distinguishable structurally, and a dead
companion appears as a named figure in an Isometry campaign.

---

## 4. Findings

*Verified facts discovered during the work, dated, with references. Empty at
founding.*

---

## 5. Progress

- **2026-07-30**: repo founded, name reserved, design recorded. No code.
