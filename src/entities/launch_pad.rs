// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module contains the launch pad bundle.

use bevy::prelude::*;

#[derive(Bundle)]
pub struct PadBundle {
    pub pbr: PbrBundle,
}

impl PadBundle {
    pub fn material(materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
        materials.add(Color::rgba(0.0, 1.0, 0.0, 0.5).into())
    }

    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: &Handle<StandardMaterial>,
        pos: Vec2,
        size: Vec2,
    ) -> Self {
        let translation = pos.extend(0.0);
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
