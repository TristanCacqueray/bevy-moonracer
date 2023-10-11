// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module integrates the ui-navigation crate.

use bevy::prelude::*;
use bevy_ui_navigation::{
    prelude::{FocusState, Focusable, NavEvent, NavRequest},
    systems::InputMapping,
    DefaultNavigationPlugins, NavRequestSystem,
};
use lazy_static::lazy_static;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
// const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
// const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

lazy_static! {
    static ref STYLE_BUTTON: Style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    static ref STYLE_TEXT: TextStyle = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };
}

pub fn spawn_button<C: Component>(commands: &mut ChildBuilder, name: &str, action: C) {
    commands
        .spawn((
            ButtonBundle {
                style: STYLE_BUTTON.clone(),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            Focusable::default(),
            action,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(name, STYLE_TEXT.clone()));
        });
}

pub fn button_system(
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>,
) {
    for (focusable, mut material) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            *material = HOVERED_BUTTON.into();
        } else {
            *material = NORMAL_BUTTON.into();
        }
    }
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultNavigationPlugins)
            .init_resource::<InputMapping>()
            .add_systems(Startup, setup_navigation_input)
            .add_systems(
                Update,
                (
                    return_trigger_action.before(NavRequestSystem),
                    (button_system, print_nav_events).after(NavRequestSystem),
                ),
            );
    }
}

fn setup_navigation_input(mut input_mapping: ResMut<InputMapping>) {
    info!("Setup nav!");
    input_mapping.keyboard_navigation = true;
    input_mapping.focus_follows_mouse = true;
}

fn print_nav_events(mut events: EventReader<NavEvent>) {
    for event in events.read() {
        info!("got nav event: {:?}", event);
    }
}

fn return_trigger_action(mut requests: EventWriter<NavRequest>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Return) {
        requests.send(NavRequest::Action);
    }
}
