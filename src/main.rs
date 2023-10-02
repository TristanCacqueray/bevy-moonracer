//! Demo example copied from bloom_3d.rs

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

mod level;
mod moonracer;
mod resources;
mod ship;
mod star;
mod velocity_gizmo;
mod wall;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_wasm_window_resize::WindowResizePlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<moonracer::GameStatus>()
        .init_resource::<resources::GameResources>()
        .insert_resource(level::simple())
        .add_systems(Startup, level::setup)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, moonracer::handle_input)
        .add_systems(
            Update,
            level::reload.run_if(in_state(moonracer::GameStatus::Reloading)),
        )
        .add_systems(
            Update,
            (star::animate, velocity_gizmo::update_gizmo)
                .run_if(in_state(moonracer::GameStatus::Flying)),
        )
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(
            FixedUpdate,
            (moonracer::move_ship, moonracer::check_star)
                .after(moonracer::handle_input)
                .run_if(in_state(moonracer::GameStatus::Flying)),
        )
        .add_systems(
            OnEnter(moonracer::GameStatus::GameOver),
            moonracer::display_score,
        )
        //.add_plugins(LogDiagnosticsPlugin::default())
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}

// remove all entities that are not a camera or window
fn _teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn setup_camera(mut commands: Commands) {
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
        point_light: PointLight {
            intensity: 1600.0,
            ..default()
        },
        transform: Transform::from_xyz(1.0, 8.0, 2.0),
        ..default()
    });
}
