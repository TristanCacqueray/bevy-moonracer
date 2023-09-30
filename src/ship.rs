use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Ship;

#[derive(Bundle)]
pub struct ShipBundle {
    pub vel: Velocity,
    pub pbr: PbrBundle,
}

pub const SHIP_SIZE: f32 = 0.1;
pub const SHIP_RADIUS: f32 = SHIP_SIZE / 2.0;

impl Ship {
    pub fn size() -> Vec2 {
        [SHIP_SIZE, SHIP_SIZE].into()
    }
}

impl ShipBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            vel: Velocity([0., 0.].into()),
            pbr: PbrBundle {
                mesh: meshes.add(shape::Cube { size: SHIP_SIZE }.try_into().unwrap()),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                ..default()
            },
        }
    }
}
