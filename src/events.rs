// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module conains the events

use bevy::prelude::*;

#[derive(Event)]
pub struct NewHighscore {
    pub level: usize,
    pub score: usize,
}

#[derive(Event, Default)]
pub enum Thruster {
    #[default]
    Stopped,
    Firing,
}

#[derive(Event, Default)]
struct StoppingThruster;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<NewHighscore>().add_event::<Thruster>();
    }
}
