use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum Difficulty {
    Easy,
    Normal,
    #[default]
    Hard,
}
