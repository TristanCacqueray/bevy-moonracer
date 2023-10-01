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
    *text = format!(
        "GG! {} (press 'r' to try again)",
        controller.start_time.elapsed().as_millis()
    );

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
    *text = format!(
        "Flying: {} {}",
        controller.score,
        controller.start_time.elapsed().as_millis()
    );

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
    controller: Res<GameResources>,
    mut ship_query: Query<(&mut Transform, &mut ship::Velocity), With<ship::Ship>>,
    collider_query: Query<&wall::WallPosition>,
    time_step: Res<FixedTime>,
) {
    let ship = ship_query.single_mut();
    let mut ship_transform = ship.0;
    let mut ship_velocity = ship.1;

    ship_velocity.0.x = 0.8 * (controller.thrust.x + ship_velocity.0.x);
    ship_velocity.0.y = 0.8 * (controller.thrust.y + ship_velocity.0.y - 0.3);

    let ts = time_step.period.as_secs_f32();
    let new_x = ship_transform.translation.x + ship_velocity.0.x * ts;
    let new_y = ship_transform.translation.y + ship_velocity.0.y * ts;
    let mut new_pos = [new_x, new_y, 0.0].into();

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

    ship_transform.translation.x = new_pos.x;
    ship_transform.translation.y = new_pos.y;
}

pub fn handle_input(
    mut controller: ResMut<GameResources>,
    state: Res<State<GameStatus>>,
    mut next_state: ResMut<NextState<GameStatus>>,
    keyboard_input: Res<Input<ScanCode>>,
    time: Res<Time>,
) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    if keyboard_input.just_released(ScanCode(19)) {
        next_state.set(GameStatus::Reloading);
        return;
    }

    for ev in keyboard_input.get_pressed() {
        match ev.0 {
            105_u32 | 30_u32 => dx = -1.0,
            106_u32 | 32_u32 => dx = 1.0,
            103_u32 | 17_u32 => dy = 1.0,
            108_u32 | 31_u32 => dy = -1.0,
            key => info!("Unknown key code {}", key),
        }
    }
    controller.thrust = Vec2::new(dx, dy);

    if controller.thrust != default() {
        if state.get() == &GameStatus::Spawned {
            info!("Lift off!");
            controller.start_time = time.last_update().unwrap();
            next_state.set(GameStatus::Flying);
        }
    }
}
