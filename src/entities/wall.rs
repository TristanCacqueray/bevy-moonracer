// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module contains the wall bundle.

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct WallPosition {
    pub translation: Vec3,
    pub size: Vec2,
}

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub pos: WallPosition,
    pub pbr: PbrBundle,
}

impl WallBundle {
    pub fn material(materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: Color::hex("#f5f5f5").unwrap(),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            ..default()
        })
    }

    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: &Handle<StandardMaterial>,
        pos: Vec2,
        size: Vec2,
    ) -> Self {
        let translation = pos.extend(0.0);
        Self {
            pos: WallPosition { translation, size },
            pbr: PbrBundle {
                mesh: meshes.add(shape::Box::new(size.x, size.y, 0.4).try_into().unwrap()),
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

impl WallPosition {
    pub fn top(&self) -> f32 {
        self.translation.y + self.size.y / 2.0
    }
    pub fn bottom(&self) -> f32 {
        self.translation.y - self.size.y / 2.0
    }
    pub fn left(&self) -> f32 {
        self.translation.x - self.size.x / 2.0
    }
    pub fn right(&self) -> f32 {
        self.translation.x + self.size.x / 2.0
    }
}
