// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

use bevy::prelude::*;

use crate::app_status::{AppStatus, MenuAction, MenuElem};

use super::button::STYLE_TEXT;

pub fn spawn(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MenuElem,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Paused", STYLE_TEXT.clone()));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    crate::ui::button::spawn_button(
                        parent,
                        "Resume",
                        MenuAction::SelectMenu(AppStatus::Playing),
                    );
                    crate::ui::button::spawn_button(parent, "Restart", MenuAction::Restart);
                    crate::ui::button::spawn_button(
                        parent,
                        "Quit",
                        MenuAction::SelectMenu(AppStatus::Menu),
                    );
                });
        });
}
