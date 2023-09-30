//! Demo example copied from bloom_3d.rs

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::sprite::collide_aabb::{collide, Collision};

mod wall;
use wall::*;

mod ship;
use ship::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_bloom_settings, bounce_spheres))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(FixedUpdate, update_scene)
        //.add_plugins(LogDiagnosticsPlugin::default())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(13.99, 5.32, 2.0),
        ..default()
    });
    let material_emissive2 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(2.0, 13.99, 5.32),
        ..default()
    });
    let material_emissive3 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(5.32, 2.0, 13.99),
        ..default()
    });
    let material_non_emissive = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
    });

    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    for x in -5..5 {
        for y in 0..5 {
            let mut hasher = DefaultHasher::new();
            (x, y).hash(&mut hasher);
            let rand = (hasher.finish() - 2) % 6;

            let material = match rand {
                0 => material_emissive1.clone(),
                1 => material_emissive2.clone(),
                2 => material_emissive3.clone(),
                3..=5 => material_non_emissive.clone(),
                _ => unreachable!(),
            };

            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material,
                    transform: Transform::from_xyz(
                        x as f32 * 2.0,
                        0.0,                   // y as f32,
                        y as f32 * 2.0 - 10.0, // (y as f32).abs() * -2.0 - 1.0,
                    ),
                    ..default()
                },
                Bouncing,
            ));
        }
    }

    commands.spawn((ShipBundle::new(&mut meshes, &mut materials), Ship));

    // walls
    let wmat = WallBundle::material(&mut materials);
    commands.spawn((
        WallBundle::new(&mut meshes, &wmat, [-5., -2.5].into(), [0.5, 5.0].into()),
        Wall,
    ));
    commands.spawn((
        WallBundle::new(&mut meshes, &wmat, [4.5, -2.5].into(), [0.5, 5.0].into()),
        Wall,
    ));
    commands.spawn((
        WallBundle::new(&mut meshes, &wmat, [-5., -2.5].into(), [10.0, 0.5].into()),
        Wall,
    ));

    // example instructions
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::BLACK,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });
}

// ------------------------------------------------------------------------------------------------

fn update_bloom_settings(
    mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>,
    mut text: Query<&mut Text>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let bloom_settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    match bloom_settings {
        (entity, Some(mut bloom_settings)) => {
            *text = "BloomSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Intensity: {}\n", bloom_settings.intensity));

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<BloomSettings>();
            }

            let dt = time.delta_seconds();

            if keycode.pressed(KeyCode::A) {
                bloom_settings.intensity -= dt / 8.0;
            }
            if keycode.pressed(KeyCode::Q) {
                bloom_settings.intensity += dt / 8.0;
            }
            bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);
        }

        (entity, None) => {
            *text = "Bloom: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(BloomSettings::default());
            }
        }
    }
}

#[derive(Component)]
struct Bouncing;

fn bounce_spheres(time: Res<Time>, mut query: Query<&mut Transform, With<Bouncing>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y =
            (transform.translation.x + transform.translation.z + time.elapsed_seconds()).sin();
    }
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
