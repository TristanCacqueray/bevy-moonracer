// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

use bevy::prelude::*;

use crate::app_status::{MenuAction, MenuElem};
use crate::level::Levels;

use crate::ui::button::STYLE_TEXT;

pub fn spawn(mut commands: Commands, levels: Res<Levels>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..default()
            },
            MenuElem,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Select a level",
                        STYLE_TEXT.clone(),
                    ));
                    let score_style = TextStyle {
                        font_size: 40.0,
                        color: crate::ui::button::TEXT_COLOR,
                        //
                        ..default()
                    };
                    for (pos, level) in levels.0.iter().enumerate() {
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                MenuElem,
                            ))
                            .with_children(|parent| {
                                crate::ui::button::spawn_button(
                                    parent,
                                    &level.name,
                                    MenuAction::LoadLevel(pos),
                                );
                                parent.spawn(TextBundle::from_section("score", STYLE_TEXT.clone()));
                            });
                    }
                });
        });
}
