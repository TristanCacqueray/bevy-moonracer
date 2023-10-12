// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

use bevy::prelude::*;

use crate::{
    app_status::{AppStatus, MenuAction, MenuElem},
    level::Levels,
    resources::{GameResources, FREQ},
};

use super::button::STYLE_TEXT;

pub fn spawn(mut commands: Commands, state: Res<GameResources>, levels: Res<Levels>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            MenuElem,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::BLACK.into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    if state.made_highscore {
                        parent.spawn(TextBundle::from_section(
                            "New HighScore!",
                            STYLE_TEXT.clone(),
                        ));
                    };
                    parent.spawn(TextBundle::from_section(
                        format!("Final Score: {}", state.elapsed()),
                        STYLE_TEXT.clone(),
                    ));
                    let has_remaining_level = state.current_level + 1 < levels.0.len();
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            if has_remaining_level {
                                crate::ui::button::spawn_button(
                                    parent,
                                    "Next Level",
                                    MenuAction::LoadLevel(state.current_level + 1),
                                );
                            }
                            crate::ui::button::spawn_button(parent, "Restart", MenuAction::Restart);
                            if !has_remaining_level {
                                crate::ui::button::spawn_button(
                                    parent,
                                    "Select Level",
                                    MenuAction::SelectMenu(AppStatus::SelectLevel),
                                );
                            }
                        });
                    if !has_remaining_level {
                        let total_score: usize = state.highscores.values().sum();
                        parent.spawn(TextBundle::from_section(
                            format!(
                                "GG, you finished moonracer in {:.03}!",
                                total_score as f32 * FREQ
                            ),
                            STYLE_TEXT.clone(),
                        ));
                    }
                });
        });
}
