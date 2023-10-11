// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module contains the player save data.

use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

use bevy_pkv::PkvStore;

#[derive(Default, Resource, Serialize, Deserialize)]
pub struct Save {
    cadet_times: HashMap<String, usize>,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(PkvStore::new("MoonRacer", "Save"))
            .insert_resource(Save::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                update_save.run_if(on_event::<crate::events::LevelCompleted>()),
            );
    }
}

fn update_save(mut pkv: ResMut<PkvStore>) {
    info!("Saving data");
}

fn setup(mut save: ResMut<Save>, pkv: ResMut<PkvStore>) {
    if let Ok(times) = pkv.get("cadet") {
        info!("Loading saved data");
        save.cadet_times = times;
    } else {
        info!("New save data");
    }
}
