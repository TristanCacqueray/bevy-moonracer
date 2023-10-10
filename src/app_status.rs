//! This plugin defines the main app status.

use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppStatus {
    #[default]
    Splash,
    Menu,
    Playing,
    Paused,
}

#[derive(Component)]
struct MenuElem;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_state::<AppStatus>()
            .add_systems(Startup, load_app_status_from_env)
            .add_plugins((splash::Plug, menu::Plug, pause::Plug));
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
        _ => {}
    }
}

mod splash {
    use super::*;

    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
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
                .add_systems(OnExit(AppStatus::Menu), despawn)
                .add_systems(
                    Update,
                    (menu_action, button_system).run_if(in_state(AppStatus::Menu)),
                );
        }
    }

    fn menu_setup(mut commands: Commands) {
        info!("Menu setup!");
        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            ..default()
        };

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
                        "Menu",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    MenuElem,
                ));
                parent
                    .spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MenuButtonAction::Play,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "New Game",
                            button_text_style.clone(),
                        ));
                    });
                parent
                    .spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        MenuButtonAction::Quit,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                    });
            });
    }

    #[derive(Component)]
    struct SelectedOption;

    const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Quit,
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<bevy::app::AppExit>,
        mut next_app_status: ResMut<NextState<AppStatus>>,
        mut next_game_status: ResMut<NextState<GameStatus>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => app_exit_events.send(bevy::app::AppExit),
                    MenuButtonAction::Play => {
                        next_app_status.set(AppStatus::Playing);
                        next_game_status.set(GameStatus::Spawning)
                        // menu_state.set(MenuState::Disabled);
                    }
                }
            }
        }
    }
}

mod pause {
    use super::*;
    pub struct Plug;
    impl Plugin for Plug {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(AppStatus::Paused), pause_setup)
                .add_systems(OnExit(AppStatus::Paused), despawn);
        }
    }
    fn pause_setup(mut _commands: Commands) {}
}

fn despawn(to_despawn: Query<Entity, With<MenuElem>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
