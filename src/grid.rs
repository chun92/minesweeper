use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: 24,
            height: 16,
        }
    }
}