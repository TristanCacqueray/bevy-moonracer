use bevy::prelude::*;

use crate::ship;

#[derive(Component)]
pub struct VelocityGizmo;

pub fn update_gizmo(
    ship_query: Query<&ship::Velocity, With<ship::Ship>>,
    mut gizmo_query: Query<&mut Transform, With<VelocityGizmo>>,
) {
    let ship = ship_query.single();
    let mut gizmo = gizmo_query.single_mut();
    let speed = ship.length();
    let angle = Vec2::new(0.0, 1.0).angle_between(ship.0);

    gizmo.rotation = Quat::from_rotation_z(angle);
    gizmo.scale.y = speed / 3.0;
}

pub fn new(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(mesh()),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform {
            scale: Vec3::new(1.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    }
}

use bevy::render::mesh::VertexAttributeValues::Float32x3;
fn mesh() -> Mesh {
    let mut base: Mesh = shape::Cylinder {
        resolution: 3,
        radius: 0.05,
        height: 0.5,
        segments: 1,
    }
    .into();
    // Adjust origin to the bottom of the cylinder.
    if let Some(Float32x3(all_pos)) = base.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in all_pos.iter_mut() {
            pos[1] += 0.25;
        }
    }
    base
}
