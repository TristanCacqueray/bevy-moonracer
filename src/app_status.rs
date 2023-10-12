// Copyright (C) 2023 by Tristan de Cacqueray
// SPDX-License-Identifier: MIT

//! This module defines the main app status.
//!
//! Checkout the bevy's example: games/game_menu.rs

use bevy::prelude::*;

use bevy_ui_navigation::prelude::{NavEvent, NavEventReaderExt, NavRequestSystem};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppStatus {
    #[default]
    Splash,
    Menu,
    SelectLevel,
    Playing,
    Paused,
}

#[derive(Component)]
pub struct MenuElem;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::ui::button::Plug)
            .add_state::<AppStatus>()
            .add_systems(Startup, load_app_status_from_env)
            .add_systems(Update, handle_nav_events.after(NavRequestSystem))
            .add_systems(Update, handle_app_input)
            .add_plugins((splash::Plug, select::Plug, menu::Plug, pause::Plug));
    }
}

#[derive(Component)]
pub enum MenuAction {
    Play,
    Restart,
    SelectMenu(AppStatus),
    LoadLevel(usize),
    Quit,
}

fn handle_nav_events(
    mut buttons: Query<&mut MenuAction>,
    mut events: EventReader<NavEvent>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    mut next_app_status: ResMut<NextState<AppStatus>>,
    mut next_game_status: ResMut<NextState<GameStatus>>,
    mut resources: ResMut<crate::resources::GameResources>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut buttons,
        |mut button| match &mut *button {
            MenuAction::Play => {
                info!("Starting the game!");
                next_app_status.set(AppStatus::Playing);
                next_game_status.set(GameStatus::Spawning);
            }
            MenuAction::Quit => app_exit_events.send(bevy::app::AppExit),
            MenuAction::Restart => next_game_status.set(GameStatus::Spawning),
            MenuAction::SelectMenu(app_status) => {
                next_app_status.set(*app_status);
                // despawn level?
            }
            MenuAction::LoadLevel(pos) => {
                info!("Loading level {}", pos);
                resources.current_level = *pos;
                next_app_status.set(AppStatus::Playing);
                next_game_status.set(GameStatus::Spawning);
            }
        },
    )
}

const P: ScanCode = ScanCode(25);
const P_W: ScanCode = ScanCode(80);

fn handle_app_input(
    keyboard_input: Res<Input<ScanCode>>,
    app_status: Res<State<AppStatus>>,
    game_status: Res<State<crate::game_status::GameStatus>>,
    gamepad_input: Res<Input<GamepadButton>>,
    mut next_app_status: ResMut<NextState<AppStatus>>,
) {
    for ev in keyboard_input.get_just_pressed() {
        match *ev {
            P | P_W => {
                if game_status.is_playing() {
                    let next_status = if app_status.get() == &AppStatus::Paused {
                        AppStatus::Playing
                    } else {
                        AppStatus::Paused
                    };
                    next_app_status.set(next_status);
                }
            }
            _ => {}
        }
    }
    // Skip splash screen on any input
    if app_status.get() == &AppStatus::Splash
        && (keyboard_input.get_just_pressed().len() > 0
            || gamepad_input.get_just_pressed().len() > 0)
    {
        next_app_status.set(AppStatus::Menu)
    }
}

use crate::game_status::GameStatus;
fn load_app_status_from_env(
    mut next_app_status: ResMut<NextState<AppStatus>>,
    mut next_game_status: ResMut<NextState<GameStatus>>,
) {
    match std::env::args().last().unwrap_or("".to_string()).as_str() {
        "menu" => next_app_status.set(AppStatus::Menu),
        "play" => {
            next_app_status.set(AppStatus::Playing);
            next_game_status.set(GameStatus::Spawning);
        }
        "select" => next_app_status.set(AppStatus::SelectLevel),
        _ => {}
    }
}

mod splash {
    use super::*;

    pub struct Plug;
    impl Plugin for Plug {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(AppStatus::Splash), splash_setup)
                .add_systems(Update, countdown.run_if(in_state(AppStatus::Splash)))
                .add_systems(OnExit(AppStatus::Splash), despawn);
        }
    }
    fn splash_setup(mut commands: Commands) {
        info!("Splash setup!");
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
                parent.spawn(TextBundle::from_section(
                    "MoonRacer",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    }

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn countdown(
        mut game_state: ResMut<NextState<AppStatus>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(AppStatus::Menu);
        }
    }
}

mod menu {
    use super::*;
    pub struct Plug;
    impl Plugin for Plug {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(AppStatus::Menu), menu_setup)
                .add_systems(OnExit(AppStatus::Menu), despawn);
        }
    }

    fn menu_setup(mut commands: Commands) {
        info!("Menu setup!");

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
                parent.spawn((
                    TextBundle::from_section(
                        "MoonRacer",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    MenuElem,
                ));
                crate::ui::button::spawn_button(parent, "New Game", MenuAction::Play);
                crate::ui::button::spawn_button(parent, "Quit", MenuAction::Quit);
            });
    }
}

mod pause {
    use super::*;
    pub struct Plug;
    impl Plugin for Plug {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(AppStatus::Paused), crate::ui::pause::spawn)
                .add_systems(OnExit(AppStatus::Paused), despawn);
        }
    }
}

mod select {
    use super::*;
    pub struct Plug;
    impl Plugin for Plug {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(AppStatus::SelectLevel), crate::ui::levels::spawn)
                .add_systems(OnExit(AppStatus::SelectLevel), despawn);
        }
    }
}

fn despawn(to_despawn: Query<Entity, With<MenuElem>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
