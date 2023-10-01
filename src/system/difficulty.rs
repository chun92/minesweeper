use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}
