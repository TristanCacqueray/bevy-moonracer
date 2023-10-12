// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module contains the player save data.

use bevy::prelude::*;

use bevy_pkv::PkvStore;

use crate::events::NewHighscore;
use crate::level::Levels;
use crate::resources::GameResources;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(PkvStore::new("MoonRacer", "Save"))
            .add_systems(Startup, setup)
            .add_systems(Update, save_highscore.run_if(on_event::<NewHighscore>()));
    }
}

fn save_highscore(
    mut events: EventReader<NewHighscore>,
    mut state: ResMut<GameResources>,
    mut pkv: ResMut<PkvStore>,
) {
    for event in events.read() {
        state.highscores.insert(event.level, event.score);
        pkv.set("cadet", &state.highscores)
            .expect("failed to store highscore");
    }
}

fn setup(mut state: ResMut<GameResources>, levels: Res<Levels>, pkv: ResMut<PkvStore>) {
    if let Ok(times) = pkv.get::<bevy::utils::HashMap<usize, usize>>("cadet") {
        info!("Loading saved data");
        // restore current_level to the last highscore.
        state.current_level = times.len().min(levels.0.len() - 1);
        state.highscores = times;
    } else {
        info!("New save data");
    }
}
