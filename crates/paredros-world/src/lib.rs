// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

//! Persistent Paredros world, embodied subjects, and recorded transitions.
//!
//! Mesocosm currently supplies verb-neutral generated topology and exact
//! material ground. This crate gives that structure Paredros meanings:
//! settlements, ruins, encounters, dungeons, inherited replacements, and
//! edits made by particular subjects. Saves carry only genesis facts and
//! ordered intents; restore regrows the base world before replaying them.
//!
//! A [`SlotId`] survives changes in what occupies it. This is the important
//! distinction for imported history: a fact replaces procedural content at
//! an existing structural address rather than adding a parallel world.
//! [`Movement`] records where a subject is and the accepted inputs that put it
//! there. [`Navigation`] is a disposable query over the same exact ground.
//! [`Bodies`] and [`Items`] own durable physical condition independently;
//! [`GameState`] composes those systems through one subject-addressed intent
//! grammar and regrows them from accepted inputs on restore.

mod bodies;
mod items;
mod movement;
mod navigation;
mod sites;
mod state;
mod transitions;
mod world;

pub use bodies::{
    Bodies, Body, BodyError, BodyProfile, MAX_NEED, MOBILITY_WOUND, Name, Needs, SAFE_FALL,
};
pub use items::{Item, ItemError, ItemId, ItemKind, ItemLocation, Items};
pub use movement::{Movement, MovementError, MovementEvent, MovementIntent, MovementSave};
pub use navigation::{Navigation, NavigationError};
pub use sites::{HistoryFactId, Layer, Site, SiteKind, SiteSource, SlotId, WorldMap};
pub use state::GameState;
pub use transitions::{DeathCause, GAME_STATE_VERSION, GameError, GameEvent, GameIntent, GameSave};
pub use world::{
    GENERATOR_VERSION, World, WorldConfig, WorldError, WorldEvent, WorldIntent, WorldSave,
};
