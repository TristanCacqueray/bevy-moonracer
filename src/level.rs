use bevy::prelude::*;

use crate::moonracer::GameStatus;
use crate::{ship, star, velocity_gizmo, wall};

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

#[derive(Resource)]
pub struct Level {
    name: String,
    walls: Vec<Rectangle>,
    pad: Rectangle,
    goals: Vec<Vec2>,
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
    let goals = vec![
        [40.0, 8.0].into(),
        [10.0, 16.0].into(),
        [15.0, 50.0].into(),
        [20.0, 20.0].into(),
        [40.0, 40.0].into(),
        [45.0, 45.0].into(),
        [50.0, 40.0].into(),
        [40.0, 30.0].into(),
        [20.0, 10.0].into(),
        [20.0, 20.0].into(),
    ];
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

pub fn reload(
    mut game_state: ResMut<crate::resources::GameResources>,
    mut next_state: ResMut<NextState<GameStatus>>,
    mut star_query: Query<&mut Transform, (With<crate::star::Star>, Without<crate::ship::Ship>)>,
    mut ship_query: Query<(&mut Transform, &mut ship::Velocity), With<ship::Ship>>,
    level: Res<Level>,
) {
    let screen = Screen::new(Vec2::new(10.0, 5.0));

    info!("Reloading!");
    // should we despawn and re-setup the level instead?
    next_state.set(GameStatus::Spawned);
    *game_state = default();
    for goal in level.goals.iter().skip(1) {
        game_state.goals.push(screen.goal_pos(*goal));
    }

    // reset star
    let mut star = star_query.single_mut();
    star.translation = screen.goal_pos(level.goals[0]).extend(0.0);

    // reset ship
    let (pad_pos, pad_size) = screen.center_pos(&level.pad);
    let ship_pos = Vec2::new(pad_pos.x, pad_pos.y - pad_size.y / 2.0);
    let mut ship = ship_query.single_mut();
    ship.0.translation = ship_pos.extend(0.0);
    *ship.1 = ship::Velocity(Vec2::new(0.0, 0.0));
}

pub fn setup(
    mut commands: Commands,
    mut game_state: ResMut<crate::resources::GameResources>,
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

    // register goals
    for goal in level.goals.iter().skip(1) {
        game_state.goals.push(screen.goal_pos(*goal));
    }

    let pad_mat = materials.add(Color::rgba(0.0, 1.0, 0.0, 0.5).into());
    let (pad_pos, pad_size) = screen.center_pos(&level.pad);
    commands.spawn((
        wall::WallBundle::new(&mut meshes, &pad_mat, pad_pos, pad_size),
        wall::Wall,
    ));

    // spawn first goal
    let goal_pos = screen.goal_pos(level.goals[0]);
    info!("goal: {}", goal_pos);
    commands.spawn((
        star::StarBundle::new(&mut meshes, &mut materials, goal_pos),
        star::Star,
    ));

    // spawn the ship on the pad
    let ship_pos = Vec2::new(pad_pos.x, pad_pos.y - pad_size.y / 2.0);
    commands
        .spawn((
            ship::ShipBundle::new(&mut meshes, &mut materials, ship_pos),
            ship::Ship,
        ))
        .with_children(|parent| {
            parent.spawn((
                velocity_gizmo::new(&mut meshes, &mut materials),
                velocity_gizmo::VelocityGizmo,
            ));
        });

    // example instructions
    commands.spawn(TextBundle::from_section(
        &level.name,
        TextStyle {
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}
