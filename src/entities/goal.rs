// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module contains the goal bundle.

use bevy::prelude::*;

#[derive(Component)]
pub struct Goal;

#[derive(Bundle)]
pub struct GoalBundle {
    pub pbr: PbrBundle,
}

pub const GOAL_SIZE: f32 = 0.1;

impl Goal {
    pub fn reached(position: Vec2, ship: Vec2) -> bool {
        (position.x - ship.x).abs() <= GOAL_SIZE && (position.y - ship.y).abs() <= GOAL_SIZE
    }
}

impl GoalBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec2,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(
                    shape::Cube {
                        size: GOAL_SIZE,
                        // subdivisions: 1,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    // base_color: Color::rgb_linear(30.0, 0., 0.), // Color::hex("#f50000").unwrap(),
                    emissive: Color::rgb_linear(3.0, 0., 0.),
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                transform: Transform {
                    translation: position.extend(0.),
                    rotation: Quat::from_euler(bevy::math::EulerRot::XYZ, 1.0, 0.9, 2.1),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn animate(time: Res<Time>, mut goal_query: Query<&mut Transform, With<Goal>>) {
    let delta = time.delta_seconds();
    for mut goal in goal_query.iter_mut() {
        goal.rotate_y(delta);
    }
}
