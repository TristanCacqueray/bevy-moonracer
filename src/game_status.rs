// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module defines the main game status.
//!
//! Checkout the bevy's example: ecs/ecs_guide.rs

use bevy::prelude::*;

use crate::app_status::AppStatus;
use crate::entities::*;
use crate::{level, moonracer, resources};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStatus {
    #[default]
    Waiting,
    Spawning,
    Flying,
    GameOver,
}

fn in_playing_state(gs: GameStatus) -> impl Condition<()> {
    in_state(AppStatus::Playing).and_then(in_state(gs))
}

impl GameStatus {
    pub fn is_playing(&self) -> bool {
        *self != GameStatus::Waiting
    }
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_state::<GameStatus>()
            .init_resource::<resources::GameResources>()
            .insert_resource(crate::level_loader::load())
            .add_systems(
                OnEnter(GameStatus::Spawning),
                (moonracer::update_ghost, level::despawn, level::setup).chain(),
            )
            .add_systems(
                Update,
                moonracer::handle_input.run_if(in_state(AppStatus::Playing)),
            )
            .add_systems(
                Update,
                (goal::animate, velocity_gizmo::update_gizmo)
                    .run_if(in_playing_state(GameStatus::Flying)),
            )
            // Configure how frequently our gameplay systems are run
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_systems(
                FixedUpdate,
                ((moonracer::move_ship, moonracer::check_goal).after(moonracer::handle_input))
                    .run_if(in_playing_state(GameStatus::Flying)),
            )
            .add_systems(OnEnter(GameStatus::GameOver), moonracer::display_score);
    }
}
