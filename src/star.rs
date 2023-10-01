use bevy::prelude::*;

#[derive(Component)]
pub struct Star;

#[derive(Bundle)]
pub struct StarBundle {
    pub pbr: PbrBundle,
}

pub const STAR_SIZE: f32 = 0.1;

impl Star {
    pub fn reached(position: Vec2, ship: Vec2) -> bool {
        (position.x - ship.x).abs() <= STAR_SIZE && (position.y - ship.y).abs() <= STAR_SIZE
    }
}

impl StarBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec2,
    ) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: meshes.add(
                    shape::Cube {
                        size: STAR_SIZE,
                        // subdivisions: 1,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(StandardMaterial {
                    // base_color: Color::rgb_linear(30.0, 0., 0.), // Color::hex("#f50000").unwrap(),
                    emissive: Color::rgb_linear(3.0, 0., 0.),
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                transform: Transform {
                    translation: position.extend(0.),
                    rotation: Quat::from_euler(bevy::math::EulerRot::XYZ, 1.0, 0.9, 2.1),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn animate(time: Res<Time>, mut star_query: Query<&mut Transform, With<crate::star::Star>>) {
    let delta = time.delta_seconds();
    for mut star in star_query.iter_mut() {
        star.rotate_y(delta);
    }
}
