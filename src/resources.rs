//! This module contains the global state
use bevy::prelude::*;

pub const FREQ: f32 = 1.0 / 60.0;

#[derive(Resource)]
pub struct GameResources {
    pub thrust: Vec2,
    pub frame_count: usize,
    pub score: usize,
    pub goals: Vec<Vec2>,
}

impl GameResources {
    pub fn elapsed_sec(&self) -> f32 {
        self.frame_count as f32 * FREQ
    }
    pub fn elapsed(&self) -> String {
        format!("{:.03} sec", self.elapsed_sec())
    }
}

impl Default for GameResources {
    fn default() -> Self {
        Self {
            thrust: default(),
            frame_count: 0,
            score: 0,
            goals: vec![],
        }
    }
}
