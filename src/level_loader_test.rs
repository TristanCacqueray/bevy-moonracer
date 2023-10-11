// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module is an example tool to validate the level loader without running the game.
//! Run with `cargo watch -x "run --example level"`
use bevy_moonracer::level_loader;

fn main() {
    for level in level_loader::load() {
        println!("{:?}", level);
    }
}
