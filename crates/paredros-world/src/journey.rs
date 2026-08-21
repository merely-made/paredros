// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

//! One body crossing the F0 structure over exact ground.
//!
//! The structural graph chooses the itinerary. This module turns each slot
//! transition into positions by expanding Mesocosm's shared [`step`] law over
//! the authoritative [`Ground`]. It owns route choice, not collision. That is
//! deliberately product-local until another real consumer earns extraction.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use mesocosm_core::places::{Ground, SURFACE_BAND, WALKER_HEIGHT, step};
use mesocosm_core::snapshot::{self, hash_bytes};
use paredros_identity::{SubjectId, Tick};
use serde::{Deserialize, Serialize};

use crate::{Layer, SlotId, World, WorldError, WorldSave};

const DIRECTIONS: [[i32; 2]; 8] = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1],
    [1, 1],
    [-1, 1],
    [-1, -1],
    [1, -1],
];

/// A caller-visible bound on one exact-ground search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteBudget {
    pub maximum_stances: u32,
    /// How far from a nest's host centre its roofed geometry may be claimed
    /// as that underground slot. Kept explicit because generation scale is a
    /// caller choice.
    pub underground_search_radius: i32,
}

impl Default for RouteBudget {
    fn default() -> Self {
        Self {
            maximum_stances: 100_000,
            underground_search_radius: 16,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JourneyIntent {
    Step { tick: Tick, toward: [i32; 3] },
}

impl JourneyIntent {
    pub const fn tick(self) -> Tick {
        match self {
            Self::Step { tick, .. } => tick,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JourneySave {
    pub world: WorldSave,
    pub subject: SubjectId,
    pub budget: RouteBudget,
    pub route_hash: u64,
    pub intents: Vec<JourneyIntent>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JourneyError {
    World(WorldError),
    EmptyBudget,
    InvalidUndergroundRadius,
    MissingFoundationRoute,
    MissingParent(SlotId),
    MissingSurface(SlotId),
    MissingUnderground(SlotId),
    NoExactRoute {
        from: [i32; 3],
        toward: [i32; 3],
    },
    SearchLimit {
        from: [i32; 3],
        toward: [i32; 3],
    },
    RouteTooLong,
    JourneyFinished,
    WrongTick {
        expected: Tick,
        actual: Tick,
    },
    PathDiverged {
        saved: u64,
        regrown: u64,
    },
    StepDiverged {
        expected: [i32; 3],
        actual: [i32; 3],
    },
    Encode,
    Decode,
}

impl From<WorldError> for JourneyError {
    fn from(error: WorldError) -> Self {
        Self::World(error)
    }
}

/// A planned and replayable traversal by one still-unnamed subject.
/// Position is authoritative; the route is a deterministic derived plan.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Journey {
    world: World,
    subject: SubjectId,
    budget: RouteBudget,
    itinerary: Vec<SlotId>,
    waypoint_steps: Vec<u32>,
    path: Vec<[i32; 3]>,
    path_index: u32,
    reached: u32,
    intents: Vec<JourneyIntent>,
    log: Vec<[i32; 3]>,
}

impl Journey {
    pub fn plan(
        world: World,
        subject: SubjectId,
        budget: RouteBudget,
    ) -> Result<Self, JourneyError> {
        if budget.maximum_stances == 0 {
            return Err(JourneyError::EmptyBudget);
        }
        if budget.underground_search_radius < 1 {
            return Err(JourneyError::InvalidUndergroundRadius);
        }
        let foundation = world
            .map()
            .foundation_journey()
            .ok_or(JourneyError::MissingFoundationRoute)?;
        let underground = foundation[0];
        let parent = world
            .map()
            .site(underground)
            .and_then(|site| site.parent)
            .ok_or(JourneyError::MissingParent(underground))?;

        // Begin outside, enter the generated underground, return to its
        // parent surface, then follow every remaining structural waypoint.
        let mut itinerary = vec![parent, underground];
        itinerary.extend(foundation.iter().copied().skip(1));

        let start = surface_stance(&world, parent)?;
        let underground_stances = underground_stances(&world, underground, budget)?;
        if underground_stances.is_empty() {
            return Err(JourneyError::MissingUnderground(underground));
        }
        let hint = place_hint(&world, underground)?;
        let inward = exact_route(
            world.ground(),
            world.config().extent,
            start,
            &underground_stances,
            hint,
            budget,
        )?;

        let mut path = inward;
        let mut waypoint_steps = vec![0, path_index(&path)?];
        for slot in itinerary.iter().copied().skip(2) {
            let target = surface_stance(&world, slot)?;
            let from = *path.last().expect("inward route has a start");
            let route = exact_route(
                world.ground(),
                world.config().extent,
                from,
                &BTreeSet::from([target]),
                target,
                budget,
            )?;
            path.extend(route.into_iter().skip(1));
            waypoint_steps.push(path_index(&path)?);
        }

        Ok(Self {
            world,
            subject,
            budget,
            itinerary,
            waypoint_steps,
            path,
            path_index: 0,
            reached: 1,
            intents: Vec::new(),
            log: vec![start],
        })
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub const fn subject(&self) -> SubjectId {
        self.subject
    }

    pub fn position(&self) -> [i32; 3] {
        self.path[self.path_index as usize]
    }

    pub fn itinerary(&self) -> &[SlotId] {
        &self.itinerary
    }

    pub fn reached(&self) -> &[SlotId] {
        &self.itinerary[..self.reached as usize]
    }

    pub fn waypoints(&self) -> impl Iterator<Item = (SlotId, [i32; 3])> + '_ {
        self.itinerary.iter().copied().zip(
            self.waypoint_steps
                .iter()
                .map(|index| self.path[*index as usize]),
        )
    }

    pub fn planned_steps(&self) -> usize {
        self.path.len() - 1
    }

    pub fn log(&self) -> &[[i32; 3]] {
        &self.log
    }

    pub fn intents(&self) -> &[JourneyIntent] {
        &self.intents
    }

    pub fn route_hash(&self) -> u64 {
        hash_bytes(&snapshot::encode(&self.path).expect("route always encodes"))
    }

    pub fn state_hash(&self) -> Result<u64, JourneyError> {
        snapshot::encode(self)
            .map(|bytes| hash_bytes(&bytes))
            .map_err(|_| JourneyError::Encode)
    }

    pub fn finished(&self) -> bool {
        self.path_index as usize + 1 == self.path.len()
    }

    pub fn advance(&mut self) -> Result<bool, JourneyError> {
        if self.finished() {
            return Ok(false);
        }
        let intent = JourneyIntent::Step {
            tick: Tick(self.intents.len() as u64),
            toward: self.path[self.path_index as usize + 1],
        };
        self.apply(intent)?;
        Ok(true)
    }

    pub fn run(&mut self, steps: usize) -> Result<(), JourneyError> {
        for _ in 0..steps {
            if !self.advance()? {
                break;
            }
        }
        Ok(())
    }

    pub fn run_to_end(&mut self) -> Result<(), JourneyError> {
        while self.advance()? {}
        Ok(())
    }

    fn apply(&mut self, intent: JourneyIntent) -> Result<(), JourneyError> {
        let expected_tick = Tick(self.intents.len() as u64);
        if intent.tick() != expected_tick {
            return Err(JourneyError::WrongTick {
                expected: expected_tick,
                actual: intent.tick(),
            });
        }
        if self.finished() {
            return Err(JourneyError::JourneyFinished);
        }
        let JourneyIntent::Step { toward, .. } = intent;
        let expected = self.path[self.path_index as usize + 1];
        if toward != expected {
            return Err(JourneyError::StepDiverged {
                expected,
                actual: toward,
            });
        }
        let actual = step(self.world.ground(), self.position(), toward);
        if actual != expected {
            return Err(JourneyError::StepDiverged { expected, actual });
        }
        self.path_index += 1;
        self.intents.push(intent);
        self.log.push(actual);
        while (self.reached as usize) < self.waypoint_steps.len()
            && self.path_index >= self.waypoint_steps[self.reached as usize]
        {
            self.reached += 1;
        }
        Ok(())
    }

    pub fn save_record(&self) -> JourneySave {
        JourneySave {
            world: self.world.save_record(),
            subject: self.subject,
            budget: self.budget,
            route_hash: self.route_hash(),
            intents: self.intents.clone(),
        }
    }

    pub fn save(&self) -> Result<Vec<u8>, JourneyError> {
        snapshot::encode(&self.save_record()).map_err(|_| JourneyError::Encode)
    }

    pub fn restore(bytes: &[u8]) -> Result<Self, JourneyError> {
        let save: JourneySave = snapshot::decode(bytes).map_err(|_| JourneyError::Decode)?;
        Self::restore_record(save)
    }

    pub fn restore_record(save: JourneySave) -> Result<Self, JourneyError> {
        let world = World::restore_record(save.world)?;
        let mut journey = Self::plan(world, save.subject, save.budget)?;
        let regrown = journey.route_hash();
        if regrown != save.route_hash {
            return Err(JourneyError::PathDiverged {
                saved: save.route_hash,
                regrown,
            });
        }
        for intent in save.intents {
            journey.apply(intent)?;
        }
        Ok(journey)
    }
}

fn path_index(path: &[[i32; 3]]) -> Result<u32, JourneyError> {
    u32::try_from(path.len() - 1).map_err(|_| JourneyError::RouteTooLong)
}

fn place_hint(world: &World, slot: SlotId) -> Result<[i32; 3], JourneyError> {
    let [x, z] = world
        .grown()
        .places
        .get(slot.place)
        .ok_or(JourneyError::MissingSurface(slot))?
        .centre;
    Ok([x, 0, z])
}

fn surface_stance(world: &World, slot: SlotId) -> Result<[i32; 3], JourneyError> {
    if slot.layer != Layer::Surface {
        return Err(JourneyError::MissingSurface(slot));
    }
    let [x, _, z] = place_hint(world, slot)?;
    let top = world
        .ground()
        .surface(x, z)
        .ok_or(JourneyError::MissingSurface(slot))?;
    let stance = [x, top + 1, z];
    world
        .ground()
        .stands(stance, WALKER_HEIGHT)
        .then_some(stance)
        .ok_or(JourneyError::MissingSurface(slot))
}

fn underground_stances(
    world: &World,
    slot: SlotId,
    budget: RouteBudget,
) -> Result<BTreeSet<[i32; 3]>, JourneyError> {
    if slot.layer != Layer::Underground {
        return Ok(BTreeSet::new());
    }
    let extent = world.config().extent;
    let [centre_x, _, centre_z] = place_hint(world, slot)?;
    let radius = budget.underground_search_radius;
    let mut stances = BTreeSet::new();
    for z in (centre_z - radius).max(-extent)..=(centre_z + radius).min(extent) {
        for x in (centre_x - radius).max(-extent)..=(centre_x + radius).min(extent) {
            for y in 1..SURFACE_BAND {
                let at = [x, y, z];
                if world.ground().stands(at, WALKER_HEIGHT)
                    && world.ground().solid([x, y + WALKER_HEIGHT, z])
                {
                    stances.insert(at);
                }
            }
        }
    }
    Ok(stances)
}

fn exact_route(
    ground: &Ground,
    extent: i32,
    from: [i32; 3],
    goals: &BTreeSet<[i32; 3]>,
    hint: [i32; 3],
    budget: RouteBudget,
) -> Result<Vec<[i32; 3]>, JourneyError> {
    if goals.contains(&from) {
        return Ok(vec![from]);
    }
    let mut frontier = BinaryHeap::from([Reverse((heuristic(from, hint), 0u32, from))]);
    let mut cost = BTreeMap::from([(from, 0u32)]);
    let mut previous = BTreeMap::new();

    while let Some(Reverse((_, walked, at))) = frontier.pop() {
        if cost.get(&at).copied() != Some(walked) {
            continue;
        }
        for [dx, dz] in DIRECTIONS {
            let toward = [at[0] + dx, at[1], at[2] + dz];
            let next = step(ground, at, toward);
            if next == at
                || next[0].abs() > extent
                || next[2].abs() > extent
                || !ground.stands(next, WALKER_HEIGHT)
            {
                continue;
            }
            let next_cost = walked + 1;
            if cost.get(&next).is_some_and(|old| *old <= next_cost) {
                continue;
            }
            if !cost.contains_key(&next) && cost.len() as u32 >= budget.maximum_stances {
                return Err(JourneyError::SearchLimit { from, toward: hint });
            }
            cost.insert(next, next_cost);
            previous.insert(next, at);
            if goals.contains(&next) {
                return Ok(reconstruct(from, next, &previous));
            }
            frontier.push(Reverse((
                next_cost.saturating_add(heuristic(next, hint)),
                next_cost,
                next,
            )));
        }
    }
    Err(JourneyError::NoExactRoute { from, toward: hint })
}

fn heuristic(at: [i32; 3], target: [i32; 3]) -> u32 {
    (at[0] - target[0]).abs().max((at[2] - target[2]).abs()) as u32
}

fn reconstruct(
    from: [i32; 3],
    mut at: [i32; 3],
    previous: &BTreeMap<[i32; 3], [i32; 3]>,
) -> Vec<[i32; 3]> {
    let mut route = vec![at];
    while at != from {
        at = previous[&at];
        route.push(at);
    }
    route.reverse();
    route
}
