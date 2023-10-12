// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

use crate::resources::FREQ;
use bevy::prelude::*;

use crate::app_status::{MenuAction, MenuElem};
use crate::level::Levels;

use crate::ui::button::STYLE_TEXT;

use super::button::{STYLE_BUTTON, TEXT_COLOR};

pub fn spawn(
    mut commands: Commands,
    levels: Res<Levels>,
    state: Res<crate::resources::GameResources>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MenuElem,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Select a level",
                STYLE_TEXT.clone(),
            ));
            let max_known_level: usize = *state.highscores.keys().max().unwrap_or(&0) + 1;
            info!("max lev {}", max_known_level);
            for (pos, level) in levels.0.iter().enumerate() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        let score = state.highscores.get(&pos);
                        if pos <= max_known_level {
                            crate::ui::button::spawn_button(
                                parent,
                                &level.name,
                                MenuAction::LoadLevel(pos),
                            );
                        } else {
                            parent
                                .spawn(NodeBundle {
                                    style: STYLE_BUTTON.clone(),
                                    background_color: Color::rgb(0.08, 0.08, 0.08).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Locked",
                                        TextStyle {
                                            font_size: 40.0,
                                            color: TEXT_COLOR,
                                            ..default()
                                        },
                                    ));
                                });
                        }
                        let score = match score {
                            Some(score) => format!("{:.03} sec", *score as f32 * FREQ),
                            None => "         ".into(),
                        };
                        parent.spawn(TextBundle::from_section(&score, STYLE_TEXT.clone()));
                    });
            }
            let total_score: usize = state.highscores.values().sum();
            if total_score > 0 {
                parent.spawn(TextBundle::from_section(
                    format!("Total score: {:.03} sec", total_score as f32 * FREQ),
                    STYLE_TEXT.clone(),
                ));
            }
        });
}
