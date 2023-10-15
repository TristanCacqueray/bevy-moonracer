// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BackgroundShader>::default())
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundShader>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Quad::new([6.0, 6.0].into()).try_into().unwrap()),
        material: materials.add(BackgroundShader {}),
        transform: Transform {
            translation: Vec3::new(0., 0., -1.0),
            scale: Vec3::new(1.6, 0.9, 1.0),
            ..default()
        },
        ..default()
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct BackgroundShader {}

impl Material for BackgroundShader {
    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }
}
