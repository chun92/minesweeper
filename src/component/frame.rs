use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Frame();

impl Frame {
    pub fn new() -> Self {
        Self::default()
    }
}