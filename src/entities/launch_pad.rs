// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module contains the launch pad bundle.

use bevy::prelude::*;

#[derive(Bundle)]
pub struct PadBundle {
    pub pbr: PbrBundle,
}

#[derive(Resource)]
pub struct PadMaterials {
    pub idle: Handle<StandardMaterial>,
    pub active: Handle<StandardMaterial>,
}
impl FromWorld for PadMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        Self {
            idle: materials.add(Color::rgba(0.0, 1.0, 0.0, 0.5).into()),
            active: materials.add(Color::rgba(0.0, 5.0, 0.0, 1.0).into()),
        }
    }
}

#[derive(Component)]
pub struct Pad;

impl PadBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: &Handle<StandardMaterial>,
        pos: Vec2,
        size: Vec2,
    ) -> Self {
        let translation = pos.extend(0.01);
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(shape::Quad::new(size).try_into().unwrap()),
                material: material.clone(),
                transform: Transform {
                    translation,
                    ..default()
                },
                ..default()
            },
        }
    }
}
