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


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AboutWindowState {
    #[default]
    Closed,
    Opened,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum RankingWindowState {
    #[default]
    Closed,
    Opened,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum DataReadingState {
    #[default]
    Idle,
    Ready,
    Done,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LoginPopupState {
    #[default]
    Closed,
    Opened,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LoginState {
    #[default]
    Not,
    Done,
}


pub fn run_if_all_menu_closed(
    game_menu_state: Res<State<MenuGameState>>,
    info_menu_state: Res<State<MenuInfoState>>,
    login_popup_state: Res<State<LoginPopupState>>,
) -> bool {
    if *game_menu_state == MenuGameState::Closed && *info_menu_state == MenuInfoState::Closed && *login_popup_state == LoginPopupState::Closed {
        true
    } else {
        false
    }
}