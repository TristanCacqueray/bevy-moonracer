// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

use bevy::prelude::*;

use crate::app_status::{MenuAction, MenuElem};
use crate::level::Levels;

pub fn spawn(mut commands: Commands, levels: Res<Levels>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            MenuElem,
        ))
        .with_children(|parent| {
            for (pos, level) in levels.0.iter().enumerate() {
                crate::ui::button::spawn_button(parent, &level.name, MenuAction::LoadLevel(pos));
            }
        });
}
