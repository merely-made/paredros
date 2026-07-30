# CLAUDE.md — Paredros Repository Role

This file defines how Claude Code should behave in this repository. Read it
first when starting any session.

---

## Project Identity

**Paredros** is a second-person action RPG: you play one character among
companions who are peers, who outlive you, and who succeed you when you die.
A single character with a simulated entourage, and a settlement that lives
because the entourage does.

Vessel 2 of a three-game wing — Mesocosm (first person), Paredros (second
person), Isometry (third person) — that shares a world substrate, a lineage
model, and a trust plane, but no engine, genre, or schedule.

**Pre-implementation.** The repo currently holds a name reservation and
design docs. There is no game code yet.

See `design_docs/PROJECT_DESCRIPTION.md` for the product description,
`design_docs/DOC_README.md` for the doc index, and the wing-level
architecture in the sibling repo at
`mesocosm/design_docs/2026-07-30_games_wing_founding.md`.

## Terminology

- **critter**: the plain organism word, wing-wide. Not "borg" — that is chat
  shorthand, a Gotcha Force loan with an IP shadow. **The battle-frame noun
  for this game is unnamed**; naming it is an open question, not a gap to
  fill casually.
- **companion / peer**: what the entourage is. Never "unit", never "party
  member" — both imply command.
- **succession**: a companion becoming the played character on death.
- **fili**: lineage across worlds. Not event history.
- **tulpa**: the legend and memorial organ — persistence through memory when
  no one carries the line. Proposed, **not yet inscribed in mere's lexicon**;
  treat as provisional.

Do not coin new names for these concepts mid-session. Naming rounds are
deliberate here: candidates get crates.io, game, studio, and trademark checks
before adoption, and the receipts are recorded.

## Document Structure

All authoritative design material lives in `design_docs/`. Read
`design_docs/DOC_README.md` first.

| Path | What's there |
| ---- | ----------- |
| `design_docs/DOC_README.md` | Index and AI working principles |
| `design_docs/DOC_POLICY.md` | Documentation governance |
| `design_docs/PROJECT_DESCRIPTION.md` | Product goals, pillars (maintainer-owned) |
| `design_docs/<date>_<keyword>_plan.md` | Active plans |
| `design_docs/archive_docs/<date>/` | Retired plans |

Wing-level material lives once, in Mesocosm, and is cited by path. Never copy
it here.

## General Guidelines

- Rust: standard idioms. No `unsafe` without documented justification.
- 600-LOC ceiling per source file. Split before adding when approaching it,
  and trim comment volume while splitting.
- Plans go in `design_docs/` per the date-keyword-plan convention with
  done-conditions, not time estimates. Never `.claude/plans/`.
- Follow `DOC_POLICY.md` for documentation changes.
- Check the Merely ecosystem before writing a new module: mere, genet,
  netrender, isometry, mesocosm, and the wgpu-* repos may already have the
  piece or the pattern. Name the owning layer before building anything
  app-local.
- Prefer runtime verification over extended static code tracing. If runtime
  diagnostics are blocked, surface that blocker early.

## Important Don'ts

- **Do not add real-time puppeteering of a party.** This is the scope canary,
  narrowed 2026-07-30 when the wing replaced person purity with care
  granularity (wing founding record §1). Paredros is care for **individuals**;
  drift means care widening to a squad you administer, which is Isometry's
  granularity. Permitted and encouraged: *configure, don't command* (standing
  behaviour agreed in advance, gambit-shaped, which a peer may refuse) and
  *tag-in* (temporarily becoming a companion, which is succession rehearsed).
  Forbidden: moment-to-moment orders to several characters at once.
- **Do not violate the three pipeline laws** (wing founding record §3). What
  crosses between games is choices under scarcity, not morphology;
  inheritance must be pointable; player history displaces procedural content
  and never gates it.
- **Do not build a Nemesis system.** Procedurally generated rivals with
  promotion hierarchies are patented to August 2036. Generic grudges and
  remembered encounters are fine (Dwarf Fortress prior art); the
  rival-hierarchy-promotion machinery is what to design around.
- **Do not ship the social simulation without its legibility surface.** Depth
  nobody notices reads as procedural noise; the Legends-mode equivalent is
  day-one work, not polish.
- **Do not let relationship drift run without player levers** (the Darkest
  Dungeon 2 lesson): drift the player cannot influence reads as random
  punishment.
- Do not add rollback netcode or CRDTs. Single-player first; the interop
  model is additive facts plus deferred interpretation.
- Do not add features beyond the active plan's current target without
  surfacing the scope change first.
