// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// SPDX-License-Identifier: MPL-2.0

use std::collections::BTreeSet;

use mesocosm_core::places::{WALKER_HEIGHT, step};
use paredros_identity::{SubjectId, Tick};
use paredros_world::{
    Journey, JourneyError, JourneyIntent, Layer, RouteBudget, SiteKind, World, WorldConfig,
};

const SEED: u64 = 4_242;

fn journey() -> Journey {
    let world = World::generate(SEED, WorldConfig::default()).unwrap();
    Journey::plan(world, SubjectId(1), RouteBudget::default()).unwrap()
}

#[test]
fn one_body_walks_the_four_place_itinerary_over_exact_ground() {
    let mut journey = journey();
    let initial_hash = journey.route_hash();
    journey.run_to_end().unwrap();

    assert!(journey.finished());
    assert_eq!(journey.reached(), journey.itinerary());
    assert_eq!(journey.route_hash(), initial_hash);
    assert_eq!(journey.log().len(), journey.intents().len() + 1);
    for pair in journey.log().windows(2) {
        assert_eq!(step(journey.world().ground(), pair[0], pair[1]), pair[1]);
        assert!(journey.world().ground().stands(pair[1], WALKER_HEIGHT));
    }

    let layers = journey
        .reached()
        .iter()
        .map(|slot| slot.layer)
        .collect::<BTreeSet<_>>();
    let kinds = journey
        .reached()
        .iter()
        .filter_map(|slot| journey.world().map().site(*slot))
        .map(|site| site.kind)
        .collect::<Vec<_>>();
    assert_eq!(layers, BTreeSet::from([Layer::Surface, Layer::Underground]));
    assert!(kinds.contains(&SiteKind::Dungeon));
    assert!(kinds.contains(&SiteKind::Ruin));
    assert!(kinds.contains(&SiteKind::Settlement));

    for (slot, at) in journey.waypoints() {
        if slot.layer == Layer::Underground {
            assert!(journey.world().ground().stands(at, WALKER_HEIGHT));
            assert!(
                journey
                    .world()
                    .ground()
                    .solid([at[0], at[1] + WALKER_HEIGHT, at[2]]),
                "the underground waypoint has no roof"
            );
        } else {
            assert_eq!(journey.world().grown().places.at(at), Some(slot.place));
        }
    }
}

#[test]
fn a_mid_journey_save_regrows_and_replays_to_the_straight_run() {
    let mut straight = journey();
    let halfway = straight.planned_steps() / 2;
    assert!(halfway > 0);
    straight.run(halfway).unwrap();
    assert!(!straight.finished());
    let saved = straight.save().unwrap();
    let mut restored = Journey::restore(&saved).unwrap();
    assert_eq!(restored, straight);
    assert_eq!(
        restored.state_hash().unwrap(),
        straight.state_hash().unwrap()
    );

    straight.run_to_end().unwrap();
    restored.run_to_end().unwrap();
    assert!(straight.intents().len() > halfway);
    assert_eq!(restored, straight);
    assert_eq!(
        restored.state_hash().unwrap(),
        straight.state_hash().unwrap()
    );
}

#[test]
fn twin_plans_and_runs_are_identical() {
    let mut first = journey();
    let mut second = journey();
    assert_eq!(first.route_hash(), second.route_hash());
    first.run_to_end().unwrap();
    second.run_to_end().unwrap();
    assert_eq!(first, second);
    assert_eq!(first.state_hash().unwrap(), second.state_hash().unwrap());
}

#[test]
fn restore_refuses_a_changed_exact_route() {
    let journey = journey();
    let mut save = journey.save_record();
    let saved = save.route_hash ^ 1;
    save.route_hash = saved;
    assert!(matches!(
        Journey::restore_record(save),
        Err(JourneyError::PathDiverged { saved: got, .. }) if got == saved
    ));
}

#[test]
fn restore_refuses_movement_past_the_end() {
    let mut journey = journey();
    journey.run_to_end().unwrap();
    let mut save = journey.save_record();
    save.intents.push(JourneyIntent::Step {
        tick: Tick(save.intents.len() as u64),
        toward: journey.position(),
    });
    assert_eq!(
        Journey::restore_record(save),
        Err(JourneyError::JourneyFinished)
    );
}
