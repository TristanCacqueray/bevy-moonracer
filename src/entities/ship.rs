// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module contains the ship bundle.

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct Ghost;

#[derive(Bundle)]
pub struct ShipBundle {
    pub vel: Velocity,
    pub pbr: PbrBundle,
}

pub const SHIP_SIZE: f32 = 0.1;
pub const SHIP_RADIUS: f32 = SHIP_SIZE / 2.0;

impl Ship {
    pub fn size() -> Vec2 {
        [SHIP_SIZE, SHIP_SIZE].into()
    }
}

impl ShipBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        pos: Vec2,
        color: f32,
    ) -> Self {
        let translation = Vec3::new(pos.x, pos.y + SHIP_RADIUS, 0.0);
        Self {
            vel: Velocity([0., 0.].into()),
            pbr: PbrBundle {
                mesh: meshes.add(shape::Cube { size: SHIP_SIZE }.try_into().unwrap()),
                material: materials.add(StandardMaterial {
                    emissive: Color::rgb_linear(5.0 - color, 5.0 - color, 5.0),
                    metallic: 1.0,
                    perceptual_roughness: 0.0,
                    ..default()
                }),
                transform: Transform {
                    translation,
                    ..default()
                },
                ..default()
            },
        }
    }
}
