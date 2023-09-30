use bevy::prelude::*;

use crate::star::Star;

#[derive(Resource)]
pub struct Scoreboard {
    score: usize,
}

impl Scoreboard {
    pub fn new() -> Self {
        Scoreboard { score: 0 }
    }
}

pub fn check_star(
    ship_query: Query<&Transform, With<crate::ship::Ship>>,
    star_query: Query<&mut Transform, (With<crate::star::Star>, Without<crate::ship::Ship>)>,
) {
    let ship_pos = ship_query.single().translation.truncate();
    let star = star_query.single();
    let star_pos = star.translation.truncate();

    if Star::reached(star_pos, ship_pos) {
        // println!("Reached star!");
    }
}
