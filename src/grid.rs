use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: 30,
            height: 16,
        }
    }
}

#[derive(Resource)]
pub struct TotalMine(pub u32);

impl Default for TotalMine {
    fn default() -> Self {
        Self(99)
    }
}

#[derive(Resource, Default)]
pub struct RemainingMine(pub u32);