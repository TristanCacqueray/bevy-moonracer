// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! The moonracer entry point
//!
//! Checkout the bevy's example: app/plugin.rs

#![allow(clippy::type_complexity)]

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

mod app_status;
mod game_status;

mod entities;
mod events;
mod level;
pub mod level_loader;
mod resources;
mod ui;

mod boot {
    //! This module initialize the engine.
    use bevy::prelude::*;
    pub struct Plug;
    impl Plugin for Plug {
        fn build(&self, app: &mut App) {
            app.add_plugins(DefaultPlugins)
                .insert_resource(ClearColor(Color::BLACK))
                .add_plugins(bevy_wasm_window_resize::WindowResizePlugin);
        }
    }
}

pub fn moonracer_main() {
    App::new()
        .add_plugins(boot::Plug)
        .add_systems(Startup, setup_camera)
        .add_plugins(ui::background::Plug)
        .add_plugins(events::Plug)
        .add_plugins(resources::save::Plug)
        .add_plugins(app_status::Plug)
        .add_plugins(game_status::Plug)
        //.add_plugins(LogDiagnosticsPlugin::default())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            projection: OrthographicProjection {
                scale: 2.65,
                scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1600.0,
            ..default()
        },
        transform: Transform::from_xyz(1.0, 8.0, 2.0),
        ..default()
    });
}
