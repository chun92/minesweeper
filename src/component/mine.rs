use bevy::prelude::*;

#[derive(Resource)]
pub struct TotalMine(pub u32);

impl Default for TotalMine {
    fn default() -> Self {
        Self(99)
    }
}

impl TotalMine {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn init(&mut self, num_mines: u32) {
        self.0 = num_mines;
    }
}

#[derive(Resource, Default)]
pub struct RemainingMine(pub i32);

#[derive(Resource, Default)]
pub struct Time(pub f32);