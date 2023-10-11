// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module conains the events

use bevy::prelude::*;

#[derive(Event, Default)]
pub struct LevelCompleted;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelCompleted>();
    }
}
