// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

//! F0: the persistent structure of a Paredros world and one exact journey.
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
//! [`Journey`] then proves one still-unnamed subject can cross those addresses
//! through exact ground and survive regrow-and-replay persistence.

mod journey;
mod sites;
mod world;

pub use journey::{Journey, JourneyError, JourneyIntent, JourneySave, RouteBudget};
pub use sites::{HistoryFactId, Layer, Site, SiteKind, SiteSource, SlotId, WorldMap};
pub use world::{
    GENERATOR_VERSION, World, WorldConfig, WorldError, WorldEvent, WorldIntent, WorldSave,
};
