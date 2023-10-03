use bevy::prelude::*;

use crate::resources::GameResources;
use crate::star::Star;
use crate::{ship, wall};
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameStatus {
    #[default]
    Spawned,
    Flying,
    GameOver,
    Reloading,
}

pub fn display_score(
    controller: Res<GameResources>,
    mut text: Query<&mut Text>,
    mut ship_query: Query<&mut Transform, With<crate::ship::Ship>>,
    mut star_query: Query<&mut Transform, (With<crate::star::Star>, Without<crate::ship::Ship>)>,
) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    *text = format!("GG! {} (press 'r' to try again)", controller.elapsed());

    star_query.single_mut().translation = [50.0, 50.0, 50.0].into();
    ship_query.single_mut().translation = [50.0, 50.0, 50.0].into();
}

pub fn check_star(
    mut controller: ResMut<GameResources>,
    ship_query: Query<&Transform, With<crate::ship::Ship>>,
    mut star_query: Query<&mut Transform, (With<crate::star::Star>, Without<crate::ship::Ship>)>,
    mut text: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameStatus>>,
) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    *text = format!("Flying: {} {}", controller.score, controller.elapsed());

    let ship_pos = ship_query.single().translation.truncate();
    let mut star = star_query.single_mut();
    let star_pos = star.translation.truncate();

    if Star::reached(star_pos, ship_pos) {
        info!("Reached star! {}", controller.score);
        if let Some(next_start) = controller.goals.get(controller.score) {
            star.translation = next_start.extend(0.0);
            controller.score += 1;
        } else {
            next_state.set(GameStatus::GameOver);
        }
    }
}

pub fn move_ship(
    mut controller: ResMut<GameResources>,
    mut ship_query: Query<(&mut Transform, &mut ship::Velocity), With<ship::Ship>>,
    collider_query: Query<&wall::WallPosition>,
) {
    let ship = ship_query.single_mut();
    let mut ship_transform = ship.0;
    let mut ship_velocity = ship.1;

    let thrust_power = Vec2::new(0.01, 0.013);
    let damp = 0.90;

    *ship_velocity = ship::Velocity(damp * (controller.thrust * thrust_power + ship_velocity.0));

    let gravity = Vec3::new(0.0, -0.01, 0.0);
    let mut new_pos: Vec3 = ship_transform.translation + gravity + ship_velocity.0.extend(0.0);

    for wall in &collider_query {
        if let Some(collision) = collide(new_pos, ship::Ship::size(), wall.translation, wall.size) {
            match collision {
                Collision::Left => {
                    ship_velocity.0.x = 0.;
                    new_pos.x = wall.left() - ship::SHIP_RADIUS;
                }
                Collision::Right => {
                    ship_velocity.0.x = 0.;
                    new_pos.x = wall.right() + ship::SHIP_RADIUS;
                }
                Collision::Top => {
                    ship_velocity.0.y = 0.;
                    new_pos.y = wall.top() + ship::SHIP_RADIUS;
                }
                Collision::Bottom => {
                    ship_velocity.0.y = 0.;
                    new_pos.y = wall.bottom() - ship::SHIP_RADIUS;
                }
                _ => { /* do nothing */ }
            }
        }
    }

    ship_transform.translation = new_pos;

    controller.frame_count += 1;
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
        next_state.set(GameStatus::Reloading);
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

    if controller.thrust != default() {
        if state.get() == &GameStatus::Spawned {
            info!("Lift off!");
            controller.frame_count = 0;
            next_state.set(GameStatus::Flying);
        }
    }
}
