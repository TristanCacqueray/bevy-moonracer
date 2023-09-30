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
                    shape::Icosphere {
                        radius: STAR_SIZE,
                        subdivisions: 1,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(Color::rgb(2.0, 0.1, 0.1).into()),
                transform: Transform {
                    translation: position.extend(0.),
                    ..default()
                },
                ..default()
            },
        }
    }
}
