use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}