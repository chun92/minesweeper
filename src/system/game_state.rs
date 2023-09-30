use bevy::prelude::*;
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Init,
    Ready,
    Playing,
    Win,
    Defeated
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuGameState {
    #[default]
    Closed,
    Opened,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuInfoState {
    #[default]
    Closed,
    Opened,
}

pub fn run_if_all_menu_closed(
    game_menu_state: Res<State<MenuGameState>>,
    info_menu_state: Res<State<MenuInfoState>>,
) -> bool {
    if *game_menu_state == MenuGameState::Closed && *info_menu_state == MenuInfoState::Closed {
        true
    } else {
        false
    }
}