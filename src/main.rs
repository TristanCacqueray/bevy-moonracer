//! Demo example copied from bloom_3d.rs

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use bevy::sprite::collide_aabb::{collide, Collision};

mod level;
mod moonracer;
mod ship;
mod star;
mod wall;

use ship::*;
use wall::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .insert_resource(moonracer::Scoreboard::new())
        .insert_resource(level::simple())
        .add_systems(Startup, level::setup)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, bevy::window::close_on_esc)
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(
            FixedUpdate,
            (update_scene, moonracer::check_star.after(update_scene)),
        )
        //.add_plugins(LogDiagnosticsPlugin::default())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            projection: OrthographicProjection {
                scale: 3.0,
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
        transform: Transform::from_xyz(1.0, 8.0, 2.0),
        ..default()
    });
}

fn update_scene(
    keyboard_input: Res<Input<ScanCode>>,
    mut ship_query: Query<(&mut Transform, &mut Velocity), With<Ship>>,
    collider_query: Query<&WallPosition>,
    time_step: Res<FixedTime>,
) {
    let ship = ship_query.single_mut();
    let mut ship_transform = ship.0;
    let mut ship_velocity = ship.1;

    let mut dx = 0.0;
    let mut dy = 0.0;

    for ev in keyboard_input.get_pressed() {
        match ev.0 {
            105_u32 | 30_u32 => dx = -1.0,
            106_u32 | 32_u32 => dx = 1.0,
            103_u32 | 17_u32 => dy = 1.0,
            108_u32 | 31_u32 => dy = -1.0,
            _ => {}
        }
    }

    ship_velocity.0.x = 0.8 * (dx + ship_velocity.0.x);
    ship_velocity.0.y = 0.8 * (dy + ship_velocity.0.y - 0.3);

    // Calculate the new horizontal ship position based on player input
    let ts = time_step.period.as_secs_f32();
    let new_x = ship_transform.translation.x + ship_velocity.0.x * ts;
    let new_y = ship_transform.translation.y + ship_velocity.0.y * ts;
    let mut new_pos = [new_x, new_y, 0.0].into();

    for wall in &collider_query {
        if let Some(collision) = collide(new_pos, Ship::size(), wall.translation, wall.size) {
            match collision {
                Collision::Left => {
                    ship_velocity.0.x = 0.;
                    new_pos.x = wall.left() - SHIP_RADIUS;
                }
                Collision::Right => {
                    ship_velocity.0.x = 0.;
                    new_pos.x = wall.right() + SHIP_RADIUS;
                }
                Collision::Top => {
                    ship_velocity.0.y = 0.;
                    new_pos.y = wall.top() + SHIP_RADIUS;
                }
                Collision::Bottom => {
                    ship_velocity.0.y = 0.;
                    new_pos.y = wall.bottom() - SHIP_RADIUS;
                }
                _ => { /* do nothing */ }
            }
        }
    }

    ship_transform.translation.x = new_pos.x;
    ship_transform.translation.y = new_pos.y;
}
