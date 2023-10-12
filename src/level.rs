// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module defines the level structure and how to render it.

use bevy::prelude::*;

use crate::entities::{launch_pad::PadMaterials, *};

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Vec2,
    pub size: Vec2,
}

/*
Level coordinate are from inkscape, top left is at (0,0), bottom right at (80,60)
*/

const LEVEL_SIZE: Vec2 = Vec2::new(80.0, 60.0);

impl Rectangle {
    pub fn new(top_left: Vec2, size: Vec2) -> Self {
        Self { top_left, size }
    }

    fn bottom_uv(&self) -> Vec2 {
        let bottom_left = Vec2::new(self.top_left.x, self.top_left.y + self.size.y);
        bottom_left / LEVEL_SIZE
    }

    fn size_uv(&self) -> Vec2 {
        self.size / LEVEL_SIZE
    }
}

pub struct Screen {
    dim: Vec2,
    center: Vec2,
}

impl Screen {
    fn new(dim: Vec2) -> Self {
        Screen {
            dim,
            center: dim / 2.0,
        }
    }
    fn center_pos(&self, rec: &Rectangle) -> (Vec2, Vec2) {
        let mut pos = rec.bottom_uv() * self.dim;
        pos -= self.center;
        pos.y *= -1.0;

        let size = rec.size_uv() * self.dim;
        (pos + size / 2.0, size)
    }

    fn goal_pos(&self, goal: Vec2) -> Vec2 {
        let mut pos = (goal / LEVEL_SIZE) * self.dim;
        pos -= self.center;
        pos.y *= -1.0;
        pos
    }
    pub fn default() -> Self {
        Screen::new(Vec2::new(8.85, 5.0))
    }
}

pub const OFFSCREEN: Vec2 = Vec2::new(50.0, 50.0);

#[derive(Resource, Debug)]
pub struct Level {
    pub name: String,
    pub walls: Vec<Rectangle>,
    pub pad: Rectangle,
    pub goals: Vec<Vec2>,
}

#[derive(Resource)]
pub struct Levels(pub Vec<Level>);

pub fn _simple() -> Level {
    let walls = vec![
        Rectangle {
            // left wall
            top_left: [0., 0.].into(),
            size: [5.0, 60.0].into(),
        },
        Rectangle {
            // right wall
            top_left: [75.0, 0.].into(),
            size: [5.0, 60.0].into(),
        },
        Rectangle {
            // bottom wall
            top_left: [0., 55.].into(),
            size: [80.0, 5.0].into(),
        },
    ];
    let goals = vec![
        [40.0, 8.0].into(),
        [10.0, 16.0].into(),
        [15.0, 50.0].into(),
        [20.0, 20.0].into(),
        [40.0, 40.0].into(),
        [45.0, 45.0].into(),
        [50.0, 40.0].into(),
        [40.0, 30.0].into(),
        [20.0, 10.0].into(),
        [20.0, 20.0].into(),
    ];
    Level {
        name: "simple".into(),
        walls,
        goals,
        pad: Rectangle {
            // center platform
            top_left: [37., 54.5].into(),
            size: [6.0, 0.5].into(),
        },
    }
}

pub fn initial_ship_pos(level: &Level, screen: &Screen) -> Vec2 {
    let (pad_pos, pad_size) = screen.center_pos(&level.pad);
    Vec2::new(pad_pos.x, pad_pos.y - pad_size.y / 2.0)
}

#[derive(Component)]
pub struct LevelComponent;

pub fn setup(
    mut commands: Commands,
    mut game_state: ResMut<crate::resources::GameResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pad_materials: Res<PadMaterials>,
    levels: Res<Levels>,
) {
    info!("Level setup called!");
    let screen = Screen::default();

    let level = levels.0.get(game_state.current_level).unwrap();

    // walls
    let wmat = wall::WallBundle::material(&mut materials);
    for wall in level.walls.iter() {
        let (pos, sz) = screen.center_pos(wall);

        commands.spawn((
            wall::WallBundle::new(&mut meshes, &wmat, pos, sz),
            wall::Wall,
            LevelComponent,
        ));
    }

    // Reset controller
    game_state.thrust = default();
    game_state.score = 0;
    game_state.made_highscore = false;
    game_state.thrust_history.clear();

    // register goals
    game_state.goals.clear();
    for goal in level.goals.iter() {
        game_state.goals.push(screen.goal_pos(*goal));
    }

    let (pad_pos, pad_size) = screen.center_pos(&level.pad);
    let pad_bundle =
        launch_pad::PadBundle::new(&mut meshes, &pad_materials.idle, pad_pos, pad_size);
    game_state.launch_pad = (pad_pos.extend(0.0), pad_size);
    commands.spawn((pad_bundle, launch_pad::Pad, LevelComponent));

    // spawn first goal
    let goal_pos = screen.goal_pos(level.goals[0]);
    info!("goal: {}", goal_pos);
    commands.spawn((
        goal::GoalBundle::new(&mut meshes, &mut materials, goal_pos),
        goal::Goal,
        LevelComponent,
    ));

    // spawn the ship on the pad
    let ship_pos = initial_ship_pos(&level, &screen);
    commands
        .spawn((
            ship::ShipBundle::new(&mut meshes, &mut materials, ship_pos, 0.0),
            ship::Ship,
            LevelComponent,
        ))
        .with_children(|parent| {
            parent.spawn((
                velocity_gizmo::new(&mut meshes, &mut materials),
                velocity_gizmo::VelocityGizmo,
            ));
        });

    commands.spawn((
        ship::ShipBundle::new(&mut meshes, &mut materials, OFFSCREEN, 4.5),
        ship::Ghost,
        LevelComponent,
    ));

    // example instructions
    commands.spawn((
        TextBundle::from_section(
            &level.name,
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        LevelComponent,
    ));
}

pub fn despawn(mut commands: Commands, entities: Query<Entity, With<LevelComponent>>) {
    let mut count = 0;
    for entity in &entities {
        count += 1;
        commands.entity(entity).despawn_recursive();
    }
    info!("Despawned level data {}", count);
}
