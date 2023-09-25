use bevy::prelude::*;
use crate::system::game_state::GameState;

pub fn keyboard_events_system(
    buttons: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if buttons.just_pressed(KeyCode::Back) {
        next_state.set(GameState::Menu);
    }
}