// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module defines the main game status.
//!
//! Checkout the bevy's example: ecs/ecs_guide.rs

use bevy::prelude::*;

use crate::app_status::AppStatus;
use crate::entities::goal::Goal;
use crate::entities::*;
use crate::events::NewHighscore;
use crate::level;
use crate::resources;
use crate::resources::GameResources;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStatus {
    #[default]
    Waiting,
    Spawning,
    Idling,
    Flying,
}

fn in_playing_state(gs: GameStatus) -> impl Condition<()> {
    in_state(AppStatus::Playing).and_then(in_state(gs))
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_state::<GameStatus>()
            .init_resource::<resources::GameResources>()
            .insert_resource(crate::level_loader::load())
            .init_resource::<launch_pad::PadMaterials>()
            .add_systems(
                OnEnter(GameStatus::Spawning),
                (update_ghost, level::despawn, level::setup, setup_idling).chain(),
            )
            .add_systems(Update, handle_input.run_if(in_state(AppStatus::Playing)))
            .add_systems(
                Update,
                (goal::animate, velocity_gizmo::update_gizmo)
                    .run_if(in_playing_state(GameStatus::Flying)),
            )
            // Configure how frequently our gameplay systems are run
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .add_systems(
                FixedUpdate,
                ((move_ship, check_goal).after(handle_input))
                    .run_if(in_playing_state(GameStatus::Flying)),
            );
    }
}

// That's not great, this is just to ensure OnEnter(spawning) gets called when reloading a level before flying.
// Perhaps the better solution would be to use a LevelStarted event?
pub fn setup_idling(mut next_game_status: ResMut<NextState<GameStatus>>) {
    next_game_status.set(GameStatus::Idling)
}

pub fn check_goal(
    mut state: ResMut<GameResources>,
    mut query: ParamSet<(
        Query<&Transform, With<ship::Ship>>,
        Query<&mut Transform, With<goal::Goal>>,
    )>,
    mut text: Query<&mut Text>,
    mut next_app_status: ResMut<NextState<AppStatus>>,
    mut highscore_event: EventWriter<NewHighscore>,
    pad_target_material: Res<launch_pad::PadMaterials>,
    mut pad_query: Query<&mut Handle<StandardMaterial>, With<launch_pad::Pad>>,
) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    let ship_pos = query.p0().single().translation.truncate();

    if state.score >= state.goals.len() {
        *text = format!("{}: Land on the green launch pad", state.elapsed());
        // Check if back on the landing pad
        let pad = state.launch_pad;
        if collide(ship_pos.extend(0.0), ship::Ship::size(), pad.0, pad.1).is_some() {
            let level = state.current_level;
            let highscore = state.prev_score(level);
            let score = state.frame_count;
            info!("Completed! score: {}, prev: {}", score, highscore);
            if score < highscore {
                if state.highscores.get(&level).is_some() {
                    state.made_highscore = true;
                }
                highscore_event.send(NewHighscore { level, score });
            }
            next_app_status.set(AppStatus::Completed);
        }
    } else {
        *text = format!("{}: Reach goal with wasd", state.elapsed());
        let mut goal_query = query.p1();
        let mut goal = goal_query.single_mut();
        let goal_pos = goal.translation.truncate();

        if Goal::reached(goal_pos, ship_pos) {
            info!("Reached goal! {}", state.score);
            state.score += 1;
            if let Some(next_goal) = state.goals.get(state.score) {
                goal.translation = next_goal.extend(0.0);
            } else {
                // highlight the launch pad
                let mut pad = pad_query.single_mut();
                *pad = pad_target_material.active.clone();

                goal.translation = level::OFFSCREEN.extend(0.0);
            }
        }
    }
}

pub fn update_ghost(
    mut game_state: ResMut<crate::resources::GameResources>,
    levels: Res<level::Levels>,
    collider_query: Query<&wall::WallPosition>,
) {
    let level = levels.0.get(game_state.current_level).unwrap();
    if let Some(prev_ghost) = &game_state.ghost {
        info!(
            "Prev score/frame {}/{}  current {}/{}",
            prev_ghost.score, prev_ghost.frame_count, game_state.score, game_state.frame_count
        );
        if prev_ghost.score > game_state.score
            || (prev_ghost.score == game_state.score
                && prev_ghost.frame_count <= game_state.frame_count)
        {
            info!("Ignored ghost");
            return;
        }
    }
    let screen = level::Screen::default();
    let ghost = compute_ghost(
        level::initial_ship_pos(&level, &screen),
        &game_state.thrust_history,
        &collider_query,
    );
    info!("Saving new ghost!");
    game_state.ghost = Some(resources::Ghost {
        score: game_state.score,
        frame_count: game_state.frame_count,
        positions: ghost,
    });
}

fn compute_ghost(
    initial_pos: Vec2,
    thrust_history: &Vec<Vec2>,
    collider_query: &Query<&wall::WallPosition>,
) -> Vec<Vec3> {
    let mut ghost = Vec::with_capacity(thrust_history.len());
    let mut velocity = ship::Velocity(Vec2::new(0.0, 0.0));
    let mut pos = initial_pos.extend(0.0);
    for thrust in thrust_history {
        (velocity, pos) = simulate_ship(thrust, &velocity, pos, collider_query);
        ghost.push(pos);
    }
    ghost
}

fn simulate_ship(
    current_thrust: &Vec2,
    velocity: &ship::Velocity,
    pos: Vec3,
    collider_query: &Query<&wall::WallPosition>,
) -> (ship::Velocity, Vec3) {
    let thrust_power = Vec2::new(0.01, 0.013);
    let damp = 0.90;
    let gravity = Vec3::new(0.0, -0.01, 0.0);

    let mut new_velocity = ship::Velocity(damp * (*current_thrust * thrust_power + velocity.0));
    let mut new_pos: Vec3 = pos + gravity + new_velocity.0.extend(0.0);

    for wall in collider_query {
        if let Some(collision) = collide(new_pos, ship::Ship::size(), wall.translation, wall.size) {
            match collision {
                Collision::Left => {
                    new_velocity.0.x = 0.;
                    new_pos.x = wall.left() - ship::SHIP_RADIUS;
                }
                Collision::Right => {
                    new_velocity.0.x = 0.;
                    new_pos.x = wall.right() + ship::SHIP_RADIUS;
                }
                Collision::Top => {
                    new_velocity.0.y = 0.;
                    new_pos.y = wall.top() + ship::SHIP_RADIUS;
                }
                Collision::Bottom => {
                    new_velocity.0.y = 0.;
                    new_pos.y = wall.bottom() - ship::SHIP_RADIUS;
                }
                _ => { /* do nothing */ }
            }
        }
    }

    (new_velocity, new_pos)
}

pub fn move_ship(
    mut state: ResMut<GameResources>,
    mut ship_query: ParamSet<(
        Query<(&mut Transform, &mut ship::Velocity), With<ship::Ship>>,
        Query<&mut Transform, With<ship::Ghost>>,
    )>,
    collider_query: Query<&wall::WallPosition>,
) {
    let mut ship_binding = ship_query.p0();
    let mut ship = ship_binding.single_mut();
    let current_thrust: Vec2 = state.thrust;

    let (new_velocity, new_pos) = simulate_ship(
        &current_thrust,
        &ship.1,
        ship.0.translation,
        &collider_query,
    );

    *ship.1 = new_velocity;
    ship.0.translation = new_pos;

    let current_frame = state.frame_count;
    if let Some(pos) = state
        .ghost
        .as_ref()
        .and_then(|ghost| ghost.positions.get(current_frame))
    {
        let mut query = ship_query.p1();
        query.single_mut().translation = *pos;
    }

    state.frame_count += 1;
    state.thrust_history.push(current_thrust);
}

const W: ScanCode = ScanCode(103);
const A: ScanCode = ScanCode(105);
const S: ScanCode = ScanCode(108);
const D: ScanCode = ScanCode(106);

// arrow keys
const AL: ScanCode = ScanCode(30);
const AR: ScanCode = ScanCode(32);
const AU: ScanCode = ScanCode(17);
const AD: ScanCode = ScanCode(31);

// wasm (firefox)
const W_W: ScanCode = ScanCode(87);
const A_W: ScanCode = ScanCode(65);
const S_W: ScanCode = ScanCode(83);
const D_W: ScanCode = ScanCode(68);
const AL_W: ScanCode = ScanCode(37);
const AR_W: ScanCode = ScanCode(39);
const AU_W: ScanCode = ScanCode(38);
const AD_W: ScanCode = ScanCode(40);

pub fn handle_input(
    mut state: ResMut<GameResources>,
    game_status: Res<State<GameStatus>>,
    keyboard_input: Res<Input<ScanCode>>,
    gamepad_button_input: Res<Input<GamepadButton>>,
    mut next_game_status: ResMut<NextState<GameStatus>>,
    mut events: EventWriter<crate::events::Thruster>,
) {
    let mut dx = 0.0;
    let mut dy = 0.0;
    let prev_thrust = state.thrust;
    for ev in gamepad_button_input.get_pressed() {
        match ev.button_type {
            GamepadButtonType::DPadLeft => dx = -1.0,
            GamepadButtonType::DPadRight => dx = 1.0,
            GamepadButtonType::DPadUp => dy = 1.0,
            GamepadButtonType::DPadDown => dy = -1.0,
            _ => {}
        }
    }

    for ev in keyboard_input.get_pressed() {
        match *ev {
            // left
            A | A_W | AL | AL_W => dx = -1.0,
            // right
            D | D_W | AR | AR_W => dx = 1.0,
            // up
            W | W_W | AU | AU_W => dy = 1.0,
            // down
            S | S_W | AD | AD_W => dy = -1.0,
            key => info!("Unknown key code {}", key.0),
        }
    }
    state.thrust = Vec2::new(dx, dy);
    if prev_thrust != state.thrust {
        if prev_thrust == Vec2::ZERO {
            events.send(crate::events::Thruster::Firing)
        } else if state.thrust == Vec2::ZERO {
            events.send(crate::events::Thruster::Stopped)
        }
    }

    if state.thrust != default() && game_status.get() == &GameStatus::Idling {
        info!("Lift off!");
        state.frame_count = 0;
        next_game_status.set(GameStatus::Flying);
    }
}
