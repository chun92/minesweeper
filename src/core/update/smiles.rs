use bevy::prelude::*;
use crate::component::smile::{SmileComponent, SmileSprite};

pub fn update(
    mut q_smiles: Query<(&SmileComponent, &mut TextureAtlasSprite)>
) {
    for (smile, mut sprite) in q_smiles.iter_mut() {
        sprite.index = smile.state as usize;
    }
}

pub fn set_win(
    mut q_smiles: Query<&mut SmileComponent>,
) {
    for mut smile in q_smiles.iter_mut() {
        smile.state = SmileSprite::Win;
    }
}

pub fn set_defeat(
    mut q_smiles: Query<&mut SmileComponent>,
) {
    for mut smile in q_smiles.iter_mut() {
        smile.state = SmileSprite::Defeat;
    }
}