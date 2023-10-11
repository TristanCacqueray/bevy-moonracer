// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module implements the core game mechanics.

use bevy::prelude::*;

use crate::entities::goal::Goal;
use crate::entities::*;
use crate::game_status::GameStatus;
use crate::level;
use crate::resources;
use crate::resources::GameResources;
use bevy::sprite::collide_aabb::{collide, Collision};

pub fn display_score(
    controller: Res<GameResources>,
    mut text: Query<&mut Text>,
    mut query: ParamSet<(
        Query<&mut Transform, With<ship::Ship>>,
        Query<&mut Transform, With<goal::Goal>>,
    )>,
    mut event_writer: EventWriter<crate::events::LevelCompleted>,
) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    *text = format!("GG! {} (press 'r' to try again)", controller.elapsed());

    query.p0().single_mut().translation = [50.0, 50.0, 50.0].into();
    query.p1().single_mut().translation = [50.0, 50.0, 50.0].into();

    event_writer.send_default();
}

pub fn check_goal(
    mut controller: ResMut<GameResources>,
    mut query: ParamSet<(
        Query<&Transform, With<ship::Ship>>,
        Query<&mut Transform, With<goal::Goal>>,
    )>,
    mut text: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameStatus>>,
) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    *text = format!("Flying: {} {}", controller.score, controller.elapsed());

    let ship_pos = query.p0().single().translation.truncate();

    if controller.score >= controller.goals.len() {
        // Check if back on the landing pad
        let pad = controller.launch_pad;
        if collide(ship_pos.extend(0.0), ship::Ship::size(), pad.0, pad.1).is_some() {
            next_state.set(GameStatus::GameOver);
        }
    } else {
        let mut goal_query = query.p1();
        let mut goal = goal_query.single_mut();
        let goal_pos = goal.translation.truncate();

        if Goal::reached(goal_pos, ship_pos) {
            info!("Reached goal! {}", controller.score);
            controller.score += 1;
            if let Some(next_goal) = controller.goals.get(controller.score) {
                goal.translation = next_goal.extend(0.0);
            } else {
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
    mut controller: ResMut<GameResources>,
    mut ship_query: ParamSet<(
        Query<(&mut Transform, &mut ship::Velocity), With<ship::Ship>>,
        Query<&mut Transform, With<ship::Ghost>>,
    )>,
    collider_query: Query<&wall::WallPosition>,
) {
    let mut ship_binding = ship_query.p0();
    let mut ship = ship_binding.single_mut();
    let current_thrust: Vec2 = controller.thrust;

    let (new_velocity, new_pos) = simulate_ship(
        &current_thrust,
        &ship.1,
        ship.0.translation,
        &collider_query,
    );

    *ship.1 = new_velocity;
    ship.0.translation = new_pos;

    let current_frame = controller.frame_count;
    if let Some(pos) = controller
        .ghost
        .as_ref()
        .and_then(|ghost| ghost.positions.get(current_frame))
    {
        let mut query = ship_query.p1();
        query.single_mut().translation = *pos;
    }

    controller.frame_count += 1;
    controller.thrust_history.push(current_thrust);
}

const R: ScanCode = ScanCode(19);

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
const R_W: ScanCode = ScanCode(82);
const W_W: ScanCode = ScanCode(87);
const A_W: ScanCode = ScanCode(65);
const S_W: ScanCode = ScanCode(83);
const D_W: ScanCode = ScanCode(68);
const AL_W: ScanCode = ScanCode(37);
const AR_W: ScanCode = ScanCode(39);
const AU_W: ScanCode = ScanCode(38);
const AD_W: ScanCode = ScanCode(40);

pub fn handle_input(
    mut controller: ResMut<GameResources>,
    state: Res<State<GameStatus>>,
    mut next_state: ResMut<NextState<GameStatus>>,
    keyboard_input: Res<Input<ScanCode>>,
) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    if keyboard_input.just_released(R) || keyboard_input.just_released(R_W) {
        next_state.set(GameStatus::Spawning);
        return;
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
    controller.thrust = Vec2::new(dx, dy);

    if controller.thrust != default() && state.get() == &GameStatus::Spawning {
        info!("Lift off!");
        controller.frame_count = 0;
        next_state.set(GameStatus::Flying);
    }
}
