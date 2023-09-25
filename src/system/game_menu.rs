//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const DEFAULT_RESOLUTION: (f32, f32) = (1280.0, 720.0);

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

pub mod menu {
    use bevy::{app::AppExit, prelude::*};
    use bevy::window::{PrimaryWindow, WindowResolution};

    use crate::system::game_state::GameState;
    use crate::system::game_difficulty::Difficulty;
    use super::{despawn_screen, get_text_font, Volume, TEXT_COLOR, DEFAULT_RESOLUTION};

    // This plugin manages the menu, with 5 different screens:
    // - a main menu with "New Game", "Settings", "Quit"
    // - a settings menu with two submenus and a back button
    // - two settings screen with a setting that can be set and a back button
    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                // At start, the menu is not enabled. This will be changed in `menu_setup` when
                // entering the `GameState::Menu` state.
                // Current screen in the menu is handled by an independent state from `GameState`
                .add_state::<MenuState>()
                .add_systems(OnEnter(GameState::Menu), (
                    menu_setup,
                    crate::core::init::grid::clear
                ))
                // Systems to handle the main menu screen
                .add_systems(OnEnter(MenuState::Main), main_menu_setup)
                .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
                // Systems to handle the settings menu screen
                .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
                .add_systems(
                    OnExit(MenuState::Settings),
                    despawn_screen::<OnSettingsMenuScreen>,
                )
                // Systems to handle the display settings screen
                .add_systems(
                    OnEnter(MenuState::SettingsDifficulty),
                    difficulty_settings_menu_setup,
                )
                .add_systems(
                    Update,
                    (
                        setting_button::<Difficulty>
                            .run_if(in_state(MenuState::SettingsDifficulty)),
                    ),
                )
                .add_systems(
                    OnExit(MenuState::SettingsDifficulty),
                    despawn_screen::<OnDifficultySettingsMenuScreen>,
                )
                // Systems to handle the sound settings screen
                .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
                .add_systems(
                    Update,
                    setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
                )
                .add_systems(
                    OnExit(MenuState::SettingsSound),
                    despawn_screen::<OnSoundSettingsMenuScreen>,
                )
                // Common systems to all screens that handles buttons behavior
                .add_systems(
                    Update,
                    (menu_action, button_system).run_if(in_state(GameState::Menu)),
                );
        }
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsDifficulty,
        SettingsSound,
        #[default]
        Disabled,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;

    // Tag component used to tag entities added on the settings menu screen
    #[derive(Component)]
    struct OnSettingsMenuScreen;

    // Tag component used to tag entities added on the display settings menu screen
    #[derive(Component)]
    struct OnDifficultySettingsMenuScreen;

    // Tag component used to tag entities added on the sound settings menu screen
    #[derive(Component)]
    struct OnSoundSettingsMenuScreen;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    // Tag component used to mark which setting is currently selected
    #[derive(Component)]
    struct SelectedOption;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDifficulty,
        SettingsSound,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

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

    // This system updates the settings when a new value for a setting is selected, and marks
    // the button as the one currently selected
    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != *button_setting {
                let (previous_button, mut previous_color) = selected_query.single_mut();
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>, mut q_windows: Query<&mut Window, With<PrimaryWindow>>,) {
        q_windows.single_mut().resolution = WindowResolution::new(DEFAULT_RESOLUTION.0, DEFAULT_RESOLUTION.1);
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Common style for all buttons on the screen
        let button_style = Style {
            width: Val::Px(400.0),
            height: Val::Px(100.0),
            margin: UiRect::all(Val::Px(30.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_style = Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(30.0),
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 30.0,
            color: TEXT_COLOR,
            font: get_text_font(&asset_server),
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn(
                            TextBundle::from_section(
                                "Mine Sweeper",
                                TextStyle {
                                    font_size: 60.0,
                                    color: TEXT_COLOR,
                                    font: get_text_font(&asset_server),
                                },
                            )
                            .with_style(Style {
                                margin: UiRect {
                                    left: Val::Px(200.0),
                                    right: Val::Px(200.0),
                                    top: Val::Px(50.0),
                                    bottom: Val::Px(50.0),
                                },
                                ..default()
                            }),
                        );

                        // Display three buttons for each action available from the main menu:
                        // - new game
                        // - settings
                        // - quit
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
                                let icon = asset_server.load("right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
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
                                MenuButtonAction::Settings,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("wrench.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Settings",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("exitRight.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style,
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section("Quit", button_text_style));
                            });
                    });
            });
    }

    fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let button_style = Style {
            width: Val::Px(300.0),
            height: Val::Px(100.0),
            margin: UiRect::all(Val::Px(30.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = TextStyle {
            font_size: 30.0,
            color: TEXT_COLOR,
            font: get_text_font(&asset_server),
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnSettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        for (action, text) in [
                            (MenuButtonAction::SettingsDifficulty, "Difficulty"),
                            (MenuButtonAction::SettingsSound, "Sound"),
                            (MenuButtonAction::BackToMainMenu, "Back"),
                        ] {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    action,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        text,
                                        button_text_style.clone(),
                                    ));
                                });
                        }
                    });
            });
    }

    fn difficulty_settings_menu_setup(mut commands: Commands, display_quality: Res<Difficulty>, asset_server: Res<AssetServer>) {
        let button_style = Style {
            width: Val::Px(300.0),
            height: Val::Px(100.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 30.0,
            color: TEXT_COLOR,
            font: get_text_font(&asset_server),
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnDifficultySettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                        // use the default value, `FlexDirection::Row`, from left to right.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::MIDNIGHT_BLUE.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Display a label for the current setting
                                parent.spawn(TextBundle::from_section(
                                    "Difficulty",
                                    button_text_style.clone(),
                                ).with_style(Style {
                                    margin: UiRect {
                                        left: Val::Px(30.0),
                                        ..Default::default()
                                    },
                                    ..default()
                                }));
                                // Display a button for each possible value
                                for difficulty_setting in [
                                    Difficulty::Easy,
                                    Difficulty::Normal,
                                    Difficulty::Hard,
                                ] {
                                    let mut entity = parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(200.0),
                                                height: Val::Px(100.0),
                                                ..button_style.clone()
                                            },
                                            background_color: NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        difficulty_setting,
                                    ));
                                    entity.with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            format!("{difficulty_setting:?}"),
                                            button_text_style.clone(),
                                        ));
                                    });
                                    if *display_quality == difficulty_setting {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            });
                        // Display the back button to return to the settings screen
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::BackToSettings,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Back", button_text_style));
                            });
                    });
            });
    }

    fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>, asset_server: Res<AssetServer>) {
        let button_style = Style {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = TextStyle {
            font_size: 30.0,
            color: TEXT_COLOR,
            font: get_text_font(&asset_server),
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnSoundSettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::MIDNIGHT_BLUE.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Volume",
                                    button_text_style.clone(),
                                ));
                                for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                    let mut entity = parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(30.0),
                                                height: Val::Px(65.0),
                                                ..button_style.clone()
                                            },
                                            background_color: NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        Volume(volume_setting),
                                    ));
                                    if *volume == Volume(volume_setting) {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::BackToSettings,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section("Back", button_text_style));
                            });
                    });
            });
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                    MenuButtonAction::Play => {
                        game_state.set(GameState::Init);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsDifficulty => {
                        menu_state.set(MenuState::SettingsDifficulty);
                    }
                    MenuButtonAction::SettingsSound => {
                        menu_state.set(MenuState::SettingsSound);
                    }
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => {
                        menu_state.set(MenuState::Settings);
                    }
                }
            }
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn get_text_font(asset_server: &Res<AssetServer>) -> Handle<Font> {
    asset_server.load("PressStart2P-Regular.ttf")
}