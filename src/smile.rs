use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SmileSprite {
    Normal = 0,
    Pressed = 1,
    Querying = 2,
    Win = 3,
    Defeat = 4,
}

#[derive(Component)]
pub struct SmileComponent{
    pub is_pressed: bool,
    pub state: SmileSprite,
}

impl SmileComponent {
    pub fn new() -> Self {
        Self {
            is_pressed: false,
            state: SmileSprite::Normal,
        }
    }

    pub fn pressed(&mut self) {
        if self.is_pressed {
            return;
        }
        self.is_pressed = true;
        self.state = SmileSprite::Pressed;
    }

    pub fn released(&mut self, 
        next_state: &mut NextState<super::game_state::GameState>) {
        if !self.is_pressed {
            return;
        }
        self.is_pressed = false;
        self.state = SmileSprite::Normal;
        next_state.set(super::game_state::GameState::Ready);
    }
}