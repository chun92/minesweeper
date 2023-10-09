use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum Difficulty {
    Easy,
    Normal,
    #[default]
    Hard,
}

impl Difficulty {
    pub fn to_string(&self) -> String {
        match self {
            Self::Easy => "Easy".to_string(),
            Self::Normal => "Normal".to_string(),
            Self::Hard => "Hard".to_string(),
        }
    }
}
