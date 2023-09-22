use bevy::prelude::*;
use crate::component::smile::SmileComponent;

pub fn update(
    mut q_smiles: Query<(&SmileComponent, &mut TextureAtlasSprite)>
) {
    for (smile, mut sprite) in q_smiles.iter_mut() {
        sprite.index = smile.state as usize;
    }
}