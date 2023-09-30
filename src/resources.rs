//! This module contains the global state
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameResources {
    pub thrust: Vec2,
    pub start_time: std::time::Instant,
    pub score: usize,
    pub goals: Vec<Vec2>,
}

impl Default for GameResources {
    fn default() -> Self {
        Self {
            thrust: default(),
            start_time: std::time::Instant::now(),
            score: 0,
            goals: vec![],
        }
    }
}
