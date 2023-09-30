use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct WallPosition {
    pub translation: Vec3,
    pub size: Vec2,
}

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub pos: WallPosition,
    pub pbr: PbrBundle,
}

impl WallBundle {
    pub fn material(materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
        materials.add(Color::rgb(0.1, 0.9, 0.2).into())
    }

    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: &Handle<StandardMaterial>,
        pos: Vec2,
        size: Vec2,
    ) -> Self {
        // move pos to the center
        let translation = [pos.x + size.x / 2., pos.y + size.y / 2., 0.].into();
        Self {
            pos: WallPosition { translation, size },
            pbr: PbrBundle {
                mesh: meshes.add(shape::Quad::new(size).try_into().unwrap()),
                material: material.clone(),
                transform: Transform {
                    translation,
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl WallPosition {
    pub fn top(&self) -> f32 {
        self.translation.y + self.size.y / 2.0
    }
    pub fn bottom(&self) -> f32 {
        self.translation.y - self.size.y / 2.0
    }
    pub fn left(&self) -> f32 {
        self.translation.x - self.size.x / 2.0
    }
    pub fn right(&self) -> f32 {
        self.translation.x + self.size.x / 2.0
    }
}
