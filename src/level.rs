use bevy::prelude::*;

use crate::{ship, star, wall};

struct Rectangle {
    top_left: Vec2,
    size: Vec2,
}

/*
Level coordinate are from inkscape, top left is at (0,0), bottom right at (80,60)
*/

const LEVEL_SIZE: Vec2 = Vec2::new(80.0, 60.0);

impl Rectangle {
    fn bottom_uv(&self) -> Vec2 {
        let bottom_left = Vec2::new(self.top_left.x, self.top_left.y + self.size.y);
        bottom_left / LEVEL_SIZE
    }

    fn size_uv(&self) -> Vec2 {
        self.size / LEVEL_SIZE
    }
}

struct Screen {
    dim: Vec2,
    center: Vec2,
}

impl Screen {
    fn new(dim: Vec2) -> Self {
        Screen {
            dim,
            center: dim / 2.0,
        }
    }
    fn center_pos(&self, rec: &Rectangle) -> (Vec2, Vec2) {
        let mut pos = rec.bottom_uv() * self.dim;
        pos -= self.center;
        pos.y *= -1.0;

        let size = rec.size_uv() * self.dim;
        (pos + size / 2.0, size)
    }

    fn goal_pos(&self, goal: Vec2) -> Vec2 {
        let mut pos = (goal / LEVEL_SIZE) * self.dim;
        pos -= self.center;
        pos.y *= -1.0;
        pos
    }
}

struct Goal(Vec2);

#[derive(Resource)]
pub struct Level {
    name: String,
    walls: Vec<Rectangle>,
    pad: Rectangle,
    goals: Vec<Goal>,
}

pub fn simple() -> Level {
    let walls = vec![
        Rectangle {
            // left wall
            top_left: [0., 0.].into(),
            size: [5.0, 60.0].into(),
        },
        Rectangle {
            // right wall
            top_left: [75.0, 0.].into(),
            size: [5.0, 60.0].into(),
        },
        Rectangle {
            // bottom wall
            top_left: [0., 55.].into(),
            size: [80.0, 5.0].into(),
        },
    ];
    let goals = vec![Goal([40.0, 8.0].into())];
    Level {
        name: "simple".into(),
        walls,
        goals,
        pad: Rectangle {
            // center platform
            top_left: [37., 54.5].into(),
            size: [6.0, 0.5].into(),
        },
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level: Res<Level>,
) {
    let screen = Screen::new(Vec2::new(10.0, 5.0));

    // walls
    let wmat = wall::WallBundle::material(&mut materials);
    for wall in level.walls.iter() {
        let (pos, sz) = screen.center_pos(wall);

        commands.spawn((
            wall::WallBundle::new(&mut meshes, &wmat, pos, sz),
            wall::Wall,
        ));
    }

    let pad_mat = materials.add(Color::rgb(0.9, 0.9, 0.2).into());
    let (pad_pos, pad_size) = screen.center_pos(&level.pad);
    commands.spawn((
        wall::WallBundle::new(&mut meshes, &pad_mat, pad_pos, pad_size),
        wall::Wall,
    ));

    // spawn first goal
    let goal_pos = screen.goal_pos(level.goals[0].0);
    println!("goal: {}", goal_pos);
    commands.spawn((
        star::StarBundle::new(&mut meshes, &mut materials, goal_pos),
        star::Star,
    ));

    // spawn the ship on the pad
    let ship_pos = Vec2::new(pad_pos.x, pad_pos.y + pad_size.y / 2.0);
    commands.spawn((
        ship::ShipBundle::new(&mut meshes, &mut materials, ship_pos),
        ship::Ship,
    ));

    // example instructions
    commands.spawn(TextBundle::from_section(
        &level.name,
        TextStyle {
            font_size: 20.0,
            color: Color::BLACK,
            ..default()
        },
    ));
}
