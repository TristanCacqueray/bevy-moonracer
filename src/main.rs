//! Demo example copied from bloom_3d.rs

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (update_bloom_settings, bounce_spheres))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(FixedUpdate, move_ship)
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

    let ship = meshes.add(shape::Cube { size: 0.1 }.try_into().unwrap());
    commands.spawn((
        PbrBundle {
            mesh: ship,
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        },
        Ship,
    ));

    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::new(10.0, 5.0)).try_into().unwrap()),
        material: materials.add(Color::rgb(0.1, 0.1, 0.6).into()),
        transform: Transform {
            // rotation: Quat::from_rotation_y(1.0),
            translation: Vec3::new(0.0, 0.0, -5.0),
            ..default()
        },
        ..default()
    },));

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
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
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

#[derive(Component)]
struct Ship;

fn move_ship(
    keyboard_input: Res<Input<ScanCode>>,
    mut query: Query<&mut Transform, With<Ship>>,
    time_step: Res<FixedTime>,
) {
    let mut ship_transform = query.single_mut();
    let mut dx = 0.0;
    let mut dy = 0.0;

    for ev in keyboard_input.get_pressed() {
        let code = ev.0;
        match ev.0 {
            105_u32 | 30_u32 => dx = -1.0,
            106_u32 | 32_u32 => dx = 1.0,
            103_u32 | 17_u32 => dy = 1.0,
            108_u32 | 31_u32 => dy = -1.0,
            _ => {}
        }
    }

    /*
    for ev in keyboard_input.get_just_pressed() {
        println!("{} just ev: {:?}", now.as_nanos(), ev);
    }
    */

    /*
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }
    */

    // Calculate the new horizontal ship position based on player input
    let ts = time_step.period.as_secs_f32();
    let new_x = ship_transform.translation.x + dx * ts;
    ship_transform.translation.x = new_x;
    let new_y = ship_transform.translation.y + dy * ts;
    ship_transform.translation.y = new_y;
}
