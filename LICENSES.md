# Licensing

Paredros deliberately separates the game, reusable libraries, and game
assets. A license applies according to the material's owning boundary, not
merely because its text is present in the repository.

## Game and repository

Game code, application and host crates, tests, examples, and repository
documentation are licensed under the Mozilla Public License 2.0
(`MPL-2.0`). See `LICENSE-MPL-2.0`.

The `0.0.1` name-reservation package was published under `MIT OR
Apache-2.0`; those existing grants remain in force. The MPL game-code grant
begins with repository version `0.0.2`.

## Reusable libraries

A separately identified reusable library crate may be licensed under either:

- the MIT License; see `LICENSE-MIT`
- the Apache License 2.0; see `LICENSE-APACHE`

Such a crate must declare `MIT OR Apache-2.0` in its own `Cargo.toml`.
Keeping those license texts at repository root does not dual-license the
MPL-covered game code. Extraction requires a real reusable boundary and
should not be used merely to evade the game license.

One crate holds this status: `crates/paredros-identity`, promoted to the
wing's identity crate by the R4 extraction review
(`design_docs/2026-08-10_r4_extraction_review.md`).

## Assets

Original game assets under `assets/` are licensed under Creative Commons
Attribution-ShareAlike 4.0 International (`CC-BY-SA-4.0`) unless an adjacent
notice says otherwise. See `LICENSE-CC-BY-SA-4.0` and
`assets/ATTRIBUTION.md`.

Imported or third-party assets retain their own licenses. Add their creator,
source, license, and modification history to `assets/ATTRIBUTION.md`; never
silently relicense them as project originals.

## Retained licenses

The paths below are **not** covered by this repository's MPL-2.0 grant. They
are the machine-readable form of the boundaries stated above, and the house
header tool (`mere/scripts/relicense_headers.py`) reads this table's first
column as its skip list, so no file under these paths receives the MPL
Exhibit A header. Nothing here is third-party code: both scopes are Mark's own
work held under a different grant by a recorded ruling.

| Path | License | Upstream / origin | Notice file |
|---|---|---|---|
| `crates/paredros-identity` | MIT OR Apache-2.0 | own work; promoted to the wing identity crate by the R4 extraction review, 2026-08-10 (`design_docs/2026-08-10_r4_extraction_review.md`) | `LICENSE-MIT`, `LICENSE-APACHE` |
| `assets` | CC BY-SA 4.0 | own work; the games-wing asset grant of 2026-07-31 | `LICENSE-CC-BY-SA-4.0`, `assets/ATTRIBUTION.md` |

`assets/` tracks only Markdown today, which the tool cannot reach in any case;
the row is listed so a shader or script placed there later is still skipped.
